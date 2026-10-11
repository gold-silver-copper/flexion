use flexion::Path;

/// Words ending -na.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 734);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3932);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3933);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3934);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 12071);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3935);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12073);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 23035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3936);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12075);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 3937);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12077);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3938);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12079);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12080);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23036);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 23037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3939);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3940);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3941);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12081);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 12082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3942);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3943);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12083);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 23038);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 23039);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32213);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12084);
							match s {
								[] => true,
								['o', 'm', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 23040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3944);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3945);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3946);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3947);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12085);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 12086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 735);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 736);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3949);
					match s {
						[] => true,
						['o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 12087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3950);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3951);
					match s {
						[] => true,
						['q', '\u{1}', s @ ..] => {
							super::push(out, 12088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ñ', s @ ..] => {
					super::push(out, 3953);
					match s {
						[] => true,
						['i', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 12090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 737);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 738);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3956);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12091);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23041);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', 'd', '\u{1}', s @ ..] => {
							super::push(out, 12092);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 12093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3957);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3958);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3959);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12094);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12095);
							match s {
								[] => true,
								['s', 'd', s @ ..] => {
									super::push(out, 23042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 3960);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3961);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3962);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12097);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 23043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 12098);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3963);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3964);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12101);
							match s {
								[] => true,
								['k', 'o', '\u{1}', s @ ..] => {
									super::push(out, 23044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 12102);
							match s {
								[] => true,
								['l', 'l', 'a', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32214);
											s.is_empty()
										}
										['a', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 32215);
											s.is_empty()
										}
										['n', 'e', 'h', 'c', 'd', 'ä', 'm', '\u{1}', s @ ..] => {
											super::push(out, 32216);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3965);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 12104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3966);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12105);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12106);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 12107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3967);
					s.is_empty()
				}
				['w', s @ ..] => super::t5::n3981(s, out),
				['z', s @ ..] => super::t5::n3982(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t5::n741(s, out),
		['n', s @ ..] => super::t5::n742(s, out),
		['o', s @ ..] => super::t5::n743(s, out),
		['r', s @ ..] => super::t5::n744(s, out),
		['s', s @ ..] => super::t5::n745(s, out),
		['t', s @ ..] => super::t5::n746(s, out),
		['u', s @ ..] => super::t5::n747(s, out),
		['w', s @ ..] => super::t5::n748(s, out),
		['ä', s @ ..] => super::t5::n749(s, out),
		_ => false,
	}
}

/// Words ending -oa.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 749);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3989);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3990);
					match s {
						[] => true,
						['u', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 750);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 751);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['d', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3992);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pa.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 754);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3993);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3994);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3995);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3996);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 756);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3997);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3998);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12126);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23055);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 12127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 758);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4000);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 12128);
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 23056);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 12129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4001);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12130);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23057);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 12131);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4002);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 761);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 762);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 763);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ra.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 765);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 766);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4004);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4005);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12133);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12134);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12135);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 12136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4006);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12137);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4007);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4008);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4009);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12140);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
									super::push(out, 23058);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 12141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4010);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 12142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4011);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12143);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12144);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12145);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4012);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4013);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12147);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12148);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4014);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4015);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12150);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12151);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4016);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4017);
					match s {
						[] => true,
						['g', 'r', 'i', 'b', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 12153);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4018);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12155);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 12156);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4019);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4020);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12158);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 23059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'l', 'u', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 767);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4021);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4022);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12161);
							match s {
								[] => true,
								['l', 'a', 'r', s @ ..] => {
									super::push(out, 23060);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 12162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4023);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12163);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4024);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4025);
					match s {
						[] => true,
						['e', 'd', 'n', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 12166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4026);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4027);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 12168);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4029);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12170);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23061);
									s.is_empty()
								}
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 23062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4030);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12171);
							s.is_empty()
						}
						['n', 'a', 'r', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 12172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4031);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4032);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12173);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12174);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4033);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4034);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4035);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4036);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4037);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ñ', s @ ..] => {
					super::push(out, 4038);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 771);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4040);
					match s {
						[] => true,
						['s', 'd', s @ ..] => {
							super::push(out, 12181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t5::n4054(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t5::n773(s, out),
		['k', s @ ..] => super::t5::n774(s, out),
		['l', s @ ..] => {
			super::push(out, 774);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 775);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 776);
			s.is_empty()
		}
		['o', s @ ..] => super::t5::n778(s, out),
		['p', s @ ..] => super::t5::n779(s, out),
		['r', s @ ..] => super::t5::n780(s, out),
		['t', s @ ..] => super::t5::n781(s, out),
		['u', s @ ..] => super::t5::n782(s, out),
		['w', s @ ..] => super::t5::n783(s, out),
		['y', s @ ..] => super::t5::n784(s, out),
		['ä', s @ ..] => super::t5::n785(s, out),
		_ => false,
	}
}

/// Words ending -sa.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4082);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4083);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4084);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4085);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4087);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 4088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					super::push(out, 4089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12210);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 12211);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12212);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23072);
									s.is_empty()
								}
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
			super::push(out, 789);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4090);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 790);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4092);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 12213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 791);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4094);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4095);
					match s {
						[] => true,
						['x', s @ ..] => {
							super::push(out, 12214);
							match s {
								[] => true,
								['i', 's', 'i', '\u{1}', s @ ..] => {
									super::push(out, 23073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4096);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12215);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 12216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4097);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 12217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4098);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 12218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4099);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4100);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12220);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 792);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4101);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12222);
							match s {
								[] => true,
								['n', 'e', 'k', 'n', 'a', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 23074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 12223);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12224);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12225);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12226);
							match s {
								[] => true,
								['d', 'n', 'e', 's', 'u', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 23076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4102);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12227);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12228);
							match s {
								[] => true,
								['n', 'o', 'c', 's', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 23077);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4103);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 12229);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4104);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12231);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 12232);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4105);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12234);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 23078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4107);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12236);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 12237);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4108);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4109);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4111);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4112);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4113);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4114);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4115);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4116);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12242);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4117);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 12244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4118);
					match s {
						[] => true,
						['i', 'm', s @ ..] => {
							super::push(out, 12245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4119);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12246);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 12247);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4120);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ñ', s @ ..] => {
					super::push(out, 4121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4122);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4123);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4124);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4125);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4126);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 12251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4127);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4128);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 796);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 797);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 798);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4131);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12254);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 12255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4132);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4133);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4134);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12256);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 4135);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4136);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4137);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4138);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4139);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12258);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 23079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 12259);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 23080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 799);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4141);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12261);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4142);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12263);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4143);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12265);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12266);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12267);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 12268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4144);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12269);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12270);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12272);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 12273);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 4147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4148);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 23081);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 23082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4149);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4150);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4151);
					match s {
						[] => true,
						['r', 'a', 'k', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 23083);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 23084);
									s.is_empty()
								}
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
			super::push(out, 803);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 4152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4153);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12276);
							match s {
								[] => true,
								['c', 'n', s @ ..] => {
									super::push(out, 23085);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 12277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4154);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4155);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 12278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t5::n806(s, out),
		['t', s @ ..] => super::t5::n807(s, out),
		['u', s @ ..] => super::t5::n808(s, out),
		['x', s @ ..] => {
			super::push(out, 808);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['h', s @ ..] => super::t5::n4180(s, out),
				_ => false,
			}
		}
		['z', s @ ..] => super::t5::n811(s, out),
		['ä', s @ ..] => super::t5::n812(s, out),
		_ => false,
	}
}

/// Words ending -ua.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 812);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 813);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4167);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12300);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4168);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4169);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 12302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 815);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 816);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4170);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 818);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 4172);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 819);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4174);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12304);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 12305);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12306);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 12307);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 23093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4175);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 820);
			match s {
				[] => true,
				['n', 'a', 'v', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4179);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4180);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4181);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 12308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4183);
					match s {
						[] => true,
						['s', 'i', 'r', 'u', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 12309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4184);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 4185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 824);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4186);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12310);
							match s {
								[] => true,
								['s', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 23094);
									s.is_empty()
								}
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
			super::push(out, 826);
			match s {
				[] => true,
				['e', 'n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 4188);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wa.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 828);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4189);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4190);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4191);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4192);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4194);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['n', 'a', 'w', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4197);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4198);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4199);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4201);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4202);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 12312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4203);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4204);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4205);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 12313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 836);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 837);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 838);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4207);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4208);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 12314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 841);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4210);
					match s {
						[] => true,
						['m', '-', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4211);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xa.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 4212);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ya.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 845);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 847);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4216);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['e', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 4218);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4219);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23095);
									s.is_empty()
								}
								['g', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 23096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4220);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12317);
							s.is_empty()
						}
						['l', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4221);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 851);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -za.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 853);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 4224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4226);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4227);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4228);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 12319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4230);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12320);
							match s {
								[] => true,
								['r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 23097);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12321);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 12322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4232);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4233);
					match s {
						[] => true,
						['a', 'y', '\u{1}', s @ ..] => {
							super::push(out, 12323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12324);
							s.is_empty()
						}
						['r', 't', s @ ..] => {
							super::push(out, 12325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4234);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4235);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23098);
									s.is_empty()
								}
								['l', 'h', 'ü', 'k', 'f', 'e', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 23099);
									s.is_empty()
								}
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

/// Words ending -äa.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4236);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 860);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 862);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 863);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ça.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ieb.
pub(crate) fn n881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 880);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4258);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					super::push(out, 12331);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 12332);
					match s {
						[] => true,
						['t', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 23109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'h', 'c', 's', 'a', 't', 'd', s @ ..] => {
					super::push(out, 12333);
					s.is_empty()
				}
				['r', 'e', 'f', 'f', s @ ..] => {
					super::push(out, 12334);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12335);
					match s {
						[] => true,
						['s', 'n', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 23110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4259);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4260);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12336);
					match s {
						[] => true,
						['a', 'n', 'e', 'n', s @ ..] => {
							super::push(out, 23111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 12337);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 23113);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 23114);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 38420);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 38421);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 42027);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 42028);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 38422);
											s.is_empty()
										}
										['l', 'e', s @ ..] => {
											super::push(out, 38423);
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 42029);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 42030);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'h', s @ ..] => {
											super::push(out, 38424);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 38425);
											match s {
												[] => true,
												['a', 'p', s @ ..] => {
													super::push(out, 42031);
													s.is_empty()
												}
												['e', 'u', s @ ..] => {
													super::push(out, 42032);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 42033);
													s.is_empty()
												}
												['u', 't', 'a', 'r', 'e', s @ ..] => {
													super::push(out, 42034);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['g', 'n', 'u', 't', 'f', s @ ..] => {
													super::push(out, 42035);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 42036);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 38426);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 42037);
													match s {
														[] => true,
														['l', s @ ..] => super::t5::n46721(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ß', s @ ..] => {
											super::push(out, 38427);
											match s {
												[] => true,
												['o', 'r', 'g', '\u{1}', s @ ..] => {
													super::push(out, 42038);
													s.is_empty()
												}
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
							super::push(out, 23115);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 38428);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 38429);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 23116);
							match s {
								[] => true,
								['e', 'v', 'l', s @ ..] => {
									super::push(out, 32221);
									match s {
										[] => true,
										['a', 'n', 'a', 'k', 'r', 'h', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 38430);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 23117);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32222);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 32223);
									s.is_empty()
								}
								['i', 'e', s @ ..] => {
									super::push(out, 32224);
									s.is_empty()
								}
								['t', 'i', s @ ..] => {
									super::push(out, 32225);
									s.is_empty()
								}
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
			super::push(out, 4261);
			match s {
				[] => true,
				['l', 'e', 't', s @ ..] => {
					super::push(out, 12338);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12339);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gb.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 884);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 885);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 886);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 887);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ib.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 888);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4264);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4266);
					s.is_empty()
				}
				['-', 'i', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 4267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 891);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4268);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12340);
							match s {
								[] => true,
								['r', 'e', 'v', 'n', s @ ..] => {
									super::push(out, 23118);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12341);
							s.is_empty()
						}
						['r', 'e', 't', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 23119);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 23120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4269);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 12342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 893);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 895);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lb.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 897);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4275);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 898);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4277);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4278);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4279);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12343);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 12344);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 32226);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 32227);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 12345);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23121);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32228);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 12346);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12347);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12348);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12349);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12350);
							match s {
								[] => true,
								['t', 'ü', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 23122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4281);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4282);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4283);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4284);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mb.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 899);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12351);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'h', 't', 'n', 'e', 't', 's', 'r', 'ü', 'f', '\u{1}', s @ ..] => {
			super::push(out, 900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ob.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 901);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4285);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 903);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 904);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4286);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4287);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 23123);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 23124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4288);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 905);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rb.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 906);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 4290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12354);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 12355);
							match s {
								[] => true,
								['ä', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 23125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['a', s @ ..] => {
							super::push(out, 12356);
							match s {
								[] => true,
								['l', 'u', 'p', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 23126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 12357);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32229);
											s.is_empty()
										}
										['l', 'a', 'n', s @ ..] => {
											super::push(out, 32230);
											match s {
												[] => true,
												['i', 'm', 'o', 'n', 'o', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 38431);
													s.is_empty()
												}
												['o', 'i', 't', 'i', 's', 'o', 'p', 'ä', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 38432);
													s.is_empty()
												}
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
							super::push(out, 12358);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12359);
							match s {
								[] => true,
								['a', 'd', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 23127);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['k', 'i', 't', 'r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 32231);
											s.is_empty()
										}
										['s', 'h', 'c', 'e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 32232);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 32233);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 23128);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12360);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 23129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12361);
							match s {
								[] => true,
								['a', 'i', 'l', 'i', 'x', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 23130);
									s.is_empty()
								}
								['e', 't', 't', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 23131);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 12362);
							match s {
								[] => true,
								['d', 'n', 'a', 't', 's', 'u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 23132);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 23133);
									match s {
										[] => true,
										['c', 'u', 'r', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 32234);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'i', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 32235);
											match s {
												[] => true,
												['s', 'i', 'm', 'e', '\u{1}', s @ ..] => {
													super::push(out, 38433);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'k', 'n', 'u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 32236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', 'i', 'f', 'ä', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12363);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 12364);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 23134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4291);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12365);
							match s {
								[] => true,
								['b', 't', 't', 'e', 'w', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 32237);
											s.is_empty()
										}
										['n', 'e', 'e', s @ ..] => {
											super::push(out, 32238);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 32239);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 32240);
											match s {
												[] => true,
												['e', 'u', s @ ..] => {
													super::push(out, 38434);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 38435);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 38436);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'r', 'o', 'd', s @ ..] => {
											super::push(out, 32241);
											s.is_empty()
										}
										['ß', s @ ..] => {
											super::push(out, 32242);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12366);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32243);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 32244);
											match s {
												[] => true,
												['l', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 38437);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => super::t5::n33866(s, out),
										['s', s @ ..] => super::t5::n33867(s, out),
										['t', s @ ..] => super::t5::n33868(s, out),
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
		['o', s @ ..] => super::t5::n908(s, out),
		['u', s @ ..] => super::t5::n909(s, out),
		['ä', s @ ..] => super::t5::n910(s, out),
		_ => false,
	}
}

/// Words ending -sb.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 910);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 911);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4294);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stb.
pub(crate) fn n914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 913);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4295);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ub.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 914);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 4296);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4297);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 12369);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12370);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12371);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12372);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12373);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23140);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											super::push(out, 32248);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'a', s @ ..] => {
									super::push(out, 23141);
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 32249);
											match s {
												[] => true,
												['i', 'r', 'f', 'a', 'd', 'ü', 's', '\u{1}', s @ ..] => {
													super::push(out, 38441);
													s.is_empty()
												}
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
							super::push(out, 12374);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 23142);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4298);
					match s {
						[] => true,
						['k', 'n', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12375);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12376);
							match s {
								[] => true,
								['a', 'g', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 23143);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', 'o', 'r', 'd', '\u{1}', s @ ..] => {
											super::push(out, 32250);
											s.is_empty()
										}
										['h', 'c', 'r', 'i', 'k', '\u{1}', s @ ..] => {
											super::push(out, 32251);
											s.is_empty()
										}
										['l', 'l', 'e', 't', 's', 'k', 'n', 'a', 't', '\u{1}', s @ ..] => {
											super::push(out, 32252);
											s.is_empty()
										}
										['s', 's', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 32253);
											s.is_empty()
										}
										['t', 'n', 'a', 'm', 'a', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 32254);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'e', 'b', 'n', 'e', 'f', 'l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 23144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4299);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12377);
							match s {
								[] => true,
								['d', 'n', s @ ..] => {
									super::push(out, 23145);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 38442);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 38443);
													s.is_empty()
												}
												['t', 's', s @ ..] => {
													super::push(out, 38444);
													s.is_empty()
												}
												['ß', s @ ..] => {
													super::push(out, 38445);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 32255);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 23146);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 23147);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 23148);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 32256);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 32257);
											s.is_empty()
										}
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
		['b', s @ ..] => {
			super::push(out, 915);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4300);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4301);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 12378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4302);
					match s {
						[] => true,
						['z', 'l', 'e', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4303);
					s.is_empty()
				}
				['i', 't', 't', 'ö', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4304);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['l', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12380);
							s.is_empty()
						}
						['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 12381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'r', 'e', 'u', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4305);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4306);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 4307);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4308);
					match s {
						[] => true,
						['t', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 12383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 916);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4309);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23149);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 32258);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 23150);
									match s {
										[] => true,
										['u', 'a', '-', s @ ..] => {
											super::push(out, 32259);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 23151);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 32260);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 32261);
											s.is_empty()
										}
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
					super::push(out, 4310);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 12384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4311);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => super::t5::n12551(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t5::n918(s, out),
		['p', s @ ..] => super::t5::n919(s, out),
		['r', s @ ..] => super::t5::n920(s, out),
		['s', '\u{1}', s @ ..] => super::t5::n921(s, out),
		_ => false,
	}
}

/// Words ending -vb.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	s.is_empty()
}

/// Words ending -wb.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 921);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4315);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rad.
pub(crate) fn n968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 966);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4346);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12400);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 12401);
					match s {
						[] => true,
						['n', 'a', 'n', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 23158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4347);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4348);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 12402);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12403);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12404);
					match s {
						[] => true,
						['ä', 'k', '\u{1}', s @ ..] => {
							super::push(out, 23159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 12405);
					match s {
						[] => true,
						['r', 'e', 's', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 23160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4349);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4350);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 12406);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23161);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 23162);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 23163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12407);
					match s {
						[] => true,
						['g', 'n', 'u', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32262);
									s.is_empty()
								}
								['k', 'c', s @ ..] => {
									super::push(out, 32263);
									s.is_empty()
								}
								['r', 'e', 'h', s @ ..] => {
									super::push(out, 32264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 23164);
							match s {
								[] => true,
								['o', 'i', s @ ..] => {
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['k', 'a', 'r', 't', 's', 'b', 'a', '\u{1}', s @ ..] => {
													super::push(out, 42039);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 42040);
													s.is_empty()
												}
												_ => false,
											}
										}
										['x', 'e', 'l', 'f', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 38446);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 32265);
									match s {
										[] => true,
										['a', 'h', 'c', 's', 't', 'i', s @ ..] => {
											super::push(out, 38447);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 32266);
									match s {
										[] => true,
										['e', 'h', 't', 'n', 'u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 38448);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ä', s @ ..] => {
									super::push(out, 32267);
									match s {
										[] => true,
										['t', 'i', s @ ..] => {
											match s {
												[] => true,
												['d', 'i', 'u', 'q', 'i', 'l', '\u{1}', s @ ..] => {
													super::push(out, 42041);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 42042);
													s.is_empty()
												}
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
					super::push(out, 12408);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 23165);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 23166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12409);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 23167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4351);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4352);
			match s {
				[] => true,
				['e', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 12410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4353);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4354);
			match s {
				[] => true,
				['h', 'a', s @ ..] => {
					super::push(out, 12411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4355);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4356);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4357);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12412);
					match s {
						[] => true,
						['d', 'n', s @ ..] => {
							super::push(out, 23168);
							match s {
								[] => true,
								['i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 32268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'a', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23169);
							s.is_empty()
						}
						['n', 'e', 'k', s @ ..] => {
							super::push(out, 23170);
							match s {
								[] => true,
								['e', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 32269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'd', 'n', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 23171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12413);
					match s {
						[] => true,
						['t', 'o', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32270);
									s.is_empty()
								}
								['n', 'i', 'e', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 32271);
									s.is_empty()
								}
								['o', 'r', 't', 'k', 'e', 'l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 32272);
									s.is_empty()
								}
								['t', 'e', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 32273);
									s.is_empty()
								}
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
			super::push(out, 4358);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4359);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4360);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12414);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23172);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 23173);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ed.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4366);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4367);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 4368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4370);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12415);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32274);
											s.is_empty()
										}
										['r', 'e', 't', 'n', 'u', s @ ..] => {
											match s {
												[] => true,
												['n', 'e', s @ ..] => {
													match s {
														[] => true,
														['h', 'ö', 'h', '\u{1}', s @ ..] => super::t5::n44764(s, out),
														['s', 's', 'a', 'm', s @ ..] => super::t5::n44765(s, out),
														_ => false,
													}
												}
												['s', 'h', s @ ..] => {
													super::push(out, 38449);
													s.is_empty()
												}
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
					super::push(out, 4371);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12416);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 12417);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									super::push(out, 23174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 12418);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 23175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 12419);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23176);
									match s {
										[] => true,
										['t', 's', 'i', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 32275);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'i', 'm', s @ ..] => {
									match s {
										[] => true,
										['-', 'h', s @ ..] => {
											super::push(out, 32276);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 38450);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', 'd', 'l', s @ ..] => {
											super::push(out, 32277);
											s.is_empty()
										}
										['n', 'e', 'd', 'a', s @ ..] => {
											super::push(out, 32278);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['m', 'u', 'i', 'r', s @ ..] => {
													super::push(out, 38451);
													s.is_empty()
												}
												['t', 'a', 'n', s @ ..] => {
													super::push(out, 38452);
													s.is_empty()
												}
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
							super::push(out, 12420);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 32279);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 38453);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 32280);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 23177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12421);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 23178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'e', s @ ..] => {
									super::push(out, 23179);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 23180);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 32281);
											s.is_empty()
										}
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
					super::push(out, 4372);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12422);
							match s {
								[] => true,
								['c', 's', 'r', 'e', 't', s @ ..] => {
									super::push(out, 23181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4373);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12423);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23182);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 23183);
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['e', 'b', '\u{1}', s @ ..] => {
													super::push(out, 38454);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 38455);
													s.is_empty()
												}
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
							super::push(out, 12424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 977);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4374);
					s.is_empty()
				}
				['a', 't', '\u{1}', s @ ..] => {
					super::push(out, 4375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 978);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 979);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4376);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4377);
					s.is_empty()
				}
				['p', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 980);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4379);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 981);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4381);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 12425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -agd.
pub(crate) fn n985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 983);
	match s {
		[] => true,
		['j', s @ ..] => {
			super::push(out, 4382);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 12426);
					match s {
						[] => true,
						['g', 'n', 'i', s @ ..] => {
							super::push(out, 23184);
							match s {
								[] => true,
								['l', 't', s @ ..] => {
									super::push(out, 32282);
									s.is_empty()
								}
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
			super::push(out, 4383);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12427);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4384);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12429);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ahd.
pub(crate) fn n986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 984);
	s.is_empty()
}

/// Words ending -id.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 985);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 987);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4386);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4387);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4388);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 4389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 988);
			match s {
				[] => true,
				['r', 'a', 'c', 'm', s @ ..] => {
					super::push(out, 4390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 989);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4391);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 990);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 23185);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 23186);
									match s {
										[] => true,
										['u', 'i', 'n', s @ ..] => {
											super::push(out, 32283);
											match s {
												[] => true,
												['i', 'm', 'u', 'l', 'a', '\u{1}', s @ ..] => {
													super::push(out, 38456);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 12430);
							match s {
								[] => true,
								['m', 'u', 'i', 'n', 'i', 'm', 'u', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 23187);
									s.is_empty()
								}
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
			super::push(out, 991);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4393);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4394);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4395);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12431);
							match s {
								[] => true,
								['s', 'h', s @ ..] => {
									super::push(out, 23188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 12432);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 23189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4396);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12433);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12434);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 12435);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23191);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 32284);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 23192);
									match s {
										[] => true,
										['i', 't', 'a', s @ ..] => {
											super::push(out, 32285);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['t', 'l', s @ ..] => {
											super::push(out, 32286);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 32287);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 38457);
													s.is_empty()
												}
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
							super::push(out, 12436);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 23193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12437);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12438);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 12439);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 12440);
							match s {
								[] => true,
								['c', 's', 'n', s @ ..] => {
									super::push(out, 23194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 12441);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23195);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 12442);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12443);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12444);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4398);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4399);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4400);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 992);
			match s {
				[] => true,
				['l', 'u', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12447);
							s.is_empty()
						}
						['a', 't', 'p', s @ ..] => {
							super::push(out, 12448);
							s.is_empty()
						}
						['i', 'r', s @ ..] => {
							super::push(out, 12449);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12450);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12451);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23196);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 32288);
											match s {
												[] => true,
												['p', 'u', 'k', '\u{1}', s @ ..] => {
													super::push(out, 38458);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 23197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 12452);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 12453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 993);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4401);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4402);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4403);
					match s {
						[] => true,
						['s', 'o', 'h', 'p', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 23198);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 23199);
									s.is_empty()
								}
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
			super::push(out, 994);
			s.is_empty()
		}
		['l', s @ ..] => super::t5::n997(s, out),
		['m', s @ ..] => super::t5::n998(s, out),
		['n', s @ ..] => super::t5::n999(s, out),
		['o', s @ ..] => super::t5::n1000(s, out),
		['p', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['a', s @ ..] => super::t5::n4449(s, out),
				['i', s @ ..] => super::t5::n4450(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t5::n1002(s, out),
		['s', s @ ..] => super::t5::n1003(s, out),
		['t', s @ ..] => super::t5::n1004(s, out),
		['u', s @ ..] => super::t5::n1005(s, out),
		['v', s @ ..] => super::t5::n1006(s, out),
		['w', s @ ..] => super::t5::n1007(s, out),
		['x', s @ ..] => super::t5::n1008(s, out),
		['z', s @ ..] => super::t5::n1009(s, out),
		_ => false,
	}
}

/// Words ending -kd.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	s.is_empty()
}

/// Words ending -ld.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1008);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 4455);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12535);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 23252);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 23253);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 32340);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 12536);
							match s {
								[] => true,
								['d', 'l', 'o', s @ ..] => {
									super::push(out, 23254);
									s.is_empty()
								}
								['t', 's', s @ ..] => {
									super::push(out, 23255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'b', s @ ..] => {
							super::push(out, 12537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1009);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4456);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12538);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 12539);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 23256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 12540);
							s.is_empty()
						}
						['k', 'c', 'i', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12541);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12542);
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									super::push(out, 23257);
									s.is_empty()
								}
								['l', 'a', 'b', 't', s @ ..] => {
									super::push(out, 23258);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12543);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'ü', 'r', 'r', s @ ..] => {
											super::push(out, 32341);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 32342);
											s.is_empty()
										}
										['g', 'g', 'o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 32343);
											s.is_empty()
										}
										['i', 'l', s @ ..] => {
											super::push(out, 32344);
											s.is_empty()
										}
										['n', 'h', 'o', 'b', 'a', s @ ..] => {
											super::push(out, 32345);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 32346);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['s', s @ ..] => super::t5::n40794(s, out),
												['u', s @ ..] => super::t5::n40795(s, out),
												_ => false,
											}
										}
										['z', 'i', 'e', 'w', '\u{1}', s @ ..] => super::t5::n33979(s, out),
										_ => false,
									}
								}
								['r', s @ ..] => super::t5::n24028(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t5::n12724(s, out),
						['s', s @ ..] => super::t5::n12725(s, out),
						['t', s @ ..] => super::t5::n12726(s, out),
						['y', s @ ..] => super::t5::n12727(s, out),
						_ => false,
					}
				}
				['g', s @ ..] => super::t5::n4478(s, out),
				['h', s @ ..] => super::t5::n4479(s, out),
				['i', s @ ..] => super::t5::n4480(s, out),
				['v', s @ ..] => super::t5::n4481(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t5::n1012(s, out),
		['o', s @ ..] => super::t5::n1013(s, out),
		['r', s @ ..] => super::t5::n1014(s, out),
		['u', s @ ..] => super::t5::n1015(s, out),
		_ => false,
	}
}

/// Words ending -md.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4475);
					match s {
						[] => true,
						['k', 'i', s @ ..] => {
							super::push(out, 12585);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23318);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 23319);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 23320);
									match s {
										[] => true,
										['a', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 32394);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'd', s @ ..] => {
							super::push(out, 12586);
							s.is_empty()
						}
						['t', 'h', 'c', 'a', 'n', 'r', s @ ..] => {
							super::push(out, 12587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1015);
			match s {
				[] => true,
				['ö', s @ ..] => {
					super::push(out, 4476);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nd.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['a', s @ ..] => super::t5::n1018(s, out),
		['e', s @ ..] => {
			super::push(out, 1017);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4489);
					s.is_empty()
				}
				['-', 'y', 'p', 'p', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4490);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4491);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12675);
							match s {
								[] => true,
								['r', 'o', s @ ..] => {
									super::push(out, 23443);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											super::push(out, 32494);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'r', s @ ..] => {
									super::push(out, 23444);
									match s {
										[] => true,
										['h', 'a', 'j', 't', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 32495);
											s.is_empty()
										}
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
					super::push(out, 4492);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12676);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4493);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12678);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12679);
							match s {
								[] => true,
								['j', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 32496);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 32497);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 32498);
											s.is_empty()
										}
										['s', 'g', s @ ..] => {
											super::push(out, 32499);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 23445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4494);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12680);
							match s {
								[] => true,
								['r', 't', 'b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 23446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 12681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4495);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4496);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4497);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12683);
							match s {
								[] => true,
								['n', 'e', 's', 's', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 23447);
									s.is_empty()
								}
								['s', 'g', 'n', 'i', 'l', 't', 'h', 'c', 'ü', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 23448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4498);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12684);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 23449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 12685);
							match s {
								[] => true,
								['s', 't', s @ ..] => {
									super::push(out, 23450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4499);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 12686);
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32500);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 32501);
											s.is_empty()
										}
										['n', 'h', 'e', 'z', '\u{1}', s @ ..] => {
											super::push(out, 32502);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 32503);
											s.is_empty()
										}
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
					super::push(out, 4500);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4501);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 12687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4502);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12688);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 23451);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 23452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4503);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4504);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 12690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4505);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12691);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 32504);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 32505);
											match s {
												[] => true,
												['e', 'r', 'f', '\u{1}', s @ ..] => {
													super::push(out, 38587);
													s.is_empty()
												}
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
				['k', s @ ..] => super::t5::n4527(s, out),
				['m', s @ ..] => {
					super::push(out, 4507);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4508);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23460);
									s.is_empty()
								}
								['l', s @ ..] => super::t5::n24247(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t5::n4530(s, out),
				['w', s @ ..] => super::t5::n4531(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t5::n1021(s, out),
		['r', s @ ..] => super::t5::n1022(s, out),
		['u', s @ ..] => super::t5::n1023(s, out),
		['ä', s @ ..] => super::t5::n1024(s, out),
		_ => false,
	}
}

/// Words ending -od.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4530);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4533);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4534);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4535);
					match s {
						[] => true,
						['-', 'r', 'e', 'g', 'n', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 12756);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 12757);
							s.is_empty()
						}
						['r', 'e', 'p', s @ ..] => {
							super::push(out, 12758);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 23496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4536);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 4537);
					match s {
						[] => true,
						['g', 'd', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 12759);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 12760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4539);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4541);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4542);
					match s {
						[] => true,
						['l', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 12761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4543);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4544);
					match s {
						[] => true,
						['e', 't', 's', s @ ..] => {
							super::push(out, 12762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4545);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1034);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4546);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pd.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1035);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1036);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rd.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4547);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4548);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 12764);
							match s {
								[] => true,
								['a', 't', 's', 'd', s @ ..] => {
									super::push(out, 23497);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 32524);
											s.is_empty()
										}
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
					super::push(out, 4549);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12765);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 23498);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4550);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4551);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4552);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12767);
							match s {
								[] => true,
								['p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 23499);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12768);
							match s {
								[] => true,
								['y', 'm', '\u{1}', s @ ..] => {
									super::push(out, 23500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4553);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4554);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4555);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4556);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12771);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 23501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4557);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12773);
							match s {
								[] => true,
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 23502);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4558);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4559);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12775);
							s.is_empty()
						}
						['q', 'c', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 12776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4560);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4561);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12778);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 23503);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 12779);
							match s {
								[] => true,
								['r', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 23504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 4562);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4564);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', s @ ..] => {
									super::push(out, 23505);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 32525);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'e', s @ ..] => {
									super::push(out, 23506);
									s.is_empty()
								}
								['g', 'n', 'u', s @ ..] => {
									super::push(out, 23507);
									s.is_empty()
								}
								['k', 'i', s @ ..] => {
									super::push(out, 23508);
									s.is_empty()
								}
								['n', 'e', 's', s @ ..] => {
									super::push(out, 23509);
									s.is_empty()
								}
								['r', 'u', s @ ..] => {
									super::push(out, 23510);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 23511);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 32526);
											s.is_empty()
										}
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
					super::push(out, 4565);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4568);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12782);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12783);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12785);
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12786);
							s.is_empty()
						}
						['p', 'i', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 12787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4570);
					match s {
						[] => true,
						['x', s @ ..] => {
							super::push(out, 12788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4571);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12789);
							match s {
								[] => true,
								['i', 'v', 'a', 'l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 23512);
									s.is_empty()
								}
								['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 23513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4572);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4573);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4574);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12790);
							s.is_empty()
						}
						['n', s @ ..] => super::t5::n12978(s, out),
						['s', s @ ..] => super::t5::n12979(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4575);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 4576);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 12793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t5::n1044(s, out),
		['ä', s @ ..] => super::t5::n1045(s, out),
		_ => false,
	}
}

/// Words ending -sd.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1044);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1045);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ud.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4577);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4581);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1051);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4583);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4586);
					match s {
						[] => true,
						['e', 'f', 'f', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vd.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1054);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yd.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12795);
							s.is_empty()
						}
						['y', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1056);
			match s {
				[] => true,
				['e', 'd', 'l', 'a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12797);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12798);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 23517);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12799);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12800);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 12801);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 12802);
							match s {
								[] => true,
								['x', s @ ..] => {
									super::push(out, 23519);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 12803);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 23520);
									match s {
										[] => true,
										['e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 32532);
											s.is_empty()
										}
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
			super::push(out, 1057);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', 'd', 'n', s @ ..] => {
							super::push(out, 12804);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12805);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23521);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 23522);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12806);
							match s {
								[] => true,
								['e', 'p', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 32533);
											s.is_empty()
										}
										['u', 's', 'f', s @ ..] => {
											super::push(out, 32534);
											s.is_empty()
										}
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

/// Words ending -üd.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4587);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -faufgabe.
pub(crate) fn n24315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23523);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 32535);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ngabe.
pub(crate) fn n12996(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 23524);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 23525);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 32536);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 32537);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sgabe.
pub(crate) fn n12997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12809);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 23526);
			match s {
				[] => true,
				['n', 'u', 's', s @ ..] => {
					super::push(out, 32538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 23527);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 32539);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 38593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -habe.
pub(crate) fn n4616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4592);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 12810);
			match s {
				[] => true,
				['s', 's', s @ ..] => {
					super::push(out, 23528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 12811);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12812);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 23529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 12813);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -labe.
pub(crate) fn n4617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4593);
	s.is_empty()
}

/// Words ending -nabe.
pub(crate) fn n4618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4594);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12814);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 12815);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 12816);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 23530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 12817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rabe.
pub(crate) fn n4619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4595);
	s.is_empty()
}

/// Words ending -tabe.
pub(crate) fn n4620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4596);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 12818);
			match s {
				[] => true,
				['h', 'c', 'u', 'b', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32540);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 32541);
							match s {
								[] => true,
								['c', 'o', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 38594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', s @ ..] => {
							super::push(out, 32542);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 38595);
									s.is_empty()
								}
								['o', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 38596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'd', 'n', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 32543);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 32544);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 32545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wabe.
pub(crate) fn n4621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4597);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 12819);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bbe.
pub(crate) fn n1068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1066);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4598);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12820);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4599);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ebe.
pub(crate) fn n1069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1067);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4600);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4601);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12821);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12822);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 23532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4602);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12823);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12824);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23533);
							s.is_empty()
						}
						['n', 'e', 't', 's', 'a', s @ ..] => {
							super::push(out, 23534);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 23535);
							match s {
								[] => true,
								['e', 't', 't', s @ ..] => {
									super::push(out, 32546);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 32547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 23536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12825);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 23537);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 23538);
							match s {
								[] => true,
								['e', 'g', 't', s @ ..] => {
									super::push(out, 32548);
									match s {
										[] => true,
										['l', 'e', s @ ..] => {
											super::push(out, 38597);
											s.is_empty()
										}
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
		['l', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4603);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4604);
			s.is_empty()
		}
		['s', 's', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4605);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 4606);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12826);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12827);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 23539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ibe.
pub(crate) fn n1070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1068);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4607);
			match s {
				[] => true,
				['h', 'c', 's', 'd', s @ ..] => {
					super::push(out, 12828);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 23540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 12829);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12830);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 23541);
							match s {
								[] => true,
								['c', 's', 'e', s @ ..] => {
									super::push(out, 32549);
									s.is_empty()
								}
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
			super::push(out, 4608);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 4609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12832);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lbe.
pub(crate) fn n1071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1069);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4610);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 12834);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12835);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 12836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4612);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 4613);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 12838);
					match s {
						[] => true,
						['e', 'g', 's', s @ ..] => {
							super::push(out, 23544);
							match s {
								[] => true,
								['l', 'e', 'm', s @ ..] => {
									super::push(out, 32550);
									s.is_empty()
								}
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

/// Words ending -mbe.
pub(crate) fn n1072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1070);
	match s {
		[] => true,
		['o', 'b', '-', s @ ..] => {
			super::push(out, 4614);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4615);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -obe.
pub(crate) fn n1073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1071);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4616);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4617);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rbe.
pub(crate) fn n1074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1072);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4618);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4619);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12842);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 32551);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 38598);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 32552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 12843);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12844);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12845);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23545);
							match s {
								[] => true,
								['m', 'r', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 32553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12846);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 23546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12847);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12848);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 23547);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 32554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 12849);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23548);
							match s {
								[] => true,
								['g', 's', s @ ..] => {
									super::push(out, 32555);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 38599);
											s.is_empty()
										}
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
			super::push(out, 4620);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 12850);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sbe.
pub(crate) fn n1075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1073);
	s.is_empty()
}

/// Words ending -ube.
pub(crate) fn n1076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1074);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4621);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12851);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12852);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23549);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 23550);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'd', s @ ..] => {
											super::push(out, 38600);
											match s {
												[] => true,
												['n', 'i', s @ ..] => {
													super::push(out, 42108);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 38601);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 32556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 23551);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 23552);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 23553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12853);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4622);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 4623);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4624);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -übe.
pub(crate) fn n1077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1075);
	match s {
		[] => true,
		['r', 't', s @ ..] => {
			super::push(out, 4626);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12856);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ce.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1076);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4627);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4628);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 12857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4629);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 12858);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4630);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4631);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4632);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1078);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4633);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4634);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12860);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32557);
											s.is_empty()
										}
										['-', 'e', 'm', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 32558);
											s.is_empty()
										}
										['e', 'm', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 32559);
											s.is_empty()
										}
										['k', 'c', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 32560);
											s.is_empty()
										}
										['t', 'n', 'o', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 32561);
											s.is_empty()
										}
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
					super::push(out, 4635);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12861);
							s.is_empty()
						}
						['p', 'm', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12862);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4636);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4637);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4638);
					match s {
						[] => true,
						['r', 'e', 's', s @ ..] => {
							match s {
								[] => true,
								['-', 'b', s @ ..] => {
									super::push(out, 23554);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 23555);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 23556);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 23557);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 32562);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 32563);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 32564);
											match s {
												[] => true,
												['l', 'l', 'e', 'z', 'r', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 38602);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 32565);
											match s {
												[] => true,
												['d', 'n', 'u', 'k', '\u{1}', s @ ..] => {
													super::push(out, 38603);
													s.is_empty()
												}
												['g', s @ ..] => {
													super::push(out, 38604);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 23558);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 32566);
											match s {
												[] => true,
												['m', 'm', 'i', 'z', '\u{1}', s @ ..] => {
													super::push(out, 38605);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ö', s @ ..] => {
											super::push(out, 32567);
											match s {
												[] => true,
												['k', 'i', 'l', '\u{1}', s @ ..] => {
													super::push(out, 38606);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 23559);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 32568);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 32569);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 23560);
									match s {
										[] => true,
										['k', 'r', 'a', 'm', 's', 't', 'i', 'e', 'b', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 32570);
											s.is_empty()
										}
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
			super::push(out, 1079);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4639);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12864);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23561);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 23562);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 23563);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 23564);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 23565);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 12865);
							s.is_empty()
						}
						['i', s @ ..] => super::t5::n13055(s, out),
						['l', s @ ..] => super::t5::n13056(s, out),
						['m', s @ ..] => super::t5::n13057(s, out),
						['n', s @ ..] => super::t5::n13058(s, out),
						['r', s @ ..] => super::t5::n13059(s, out),
						['s', s @ ..] => super::t5::n13060(s, out),
						['t', s @ ..] => super::t5::n13061(s, out),
						['u', s @ ..] => super::t5::n13062(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t5::n4664(s, out),
				['o', s @ ..] => super::t5::n4665(s, out),
				['u', s @ ..] => super::t5::n4666(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t5::n1082(s, out),
		['u', s @ ..] => super::t5::n1083(s, out),
		_ => false,
	}
}

/// Words ending -de.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1082);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4648);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4649);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12879);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4650);
					match s {
						[] => true,
						['i', 'r', 'b', '-', s @ ..] => {
							super::push(out, 12880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4651);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							super::push(out, 12881);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4652);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 4653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4654);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4655);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12883);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12884);
							match s {
								[] => true,
								['k', 'o', 'h', 'c', 's', 'r', s @ ..] => {
									super::push(out, 23576);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 38607);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 38608);
													s.is_empty()
												}
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
							super::push(out, 12885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4656);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 12886);
							match s {
								[] => true,
								['y', 'd', 'a', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 23577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['a', s @ ..] => {
							super::push(out, 12887);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12888);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 23578);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 23579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12889);
							match s {
								[] => true,
								['o', 'l', 'i', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 23580);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 12890);
							match s {
								[] => true,
								['d', 'a', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 23581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4657);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12891);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12892);
							match s {
								[] => true,
								['r', 'a', 'm', 't', s @ ..] => {
									super::push(out, 23582);
									match s {
										[] => true,
										['a', 'l', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 32573);
											s.is_empty()
										}
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
					super::push(out, 4658);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12893);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 32574);
											match s {
												[] => true,
												['v', 'o', 'l', '\u{1}', s @ ..] => {
													super::push(out, 38609);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'e', 'l', s @ ..] => {
											super::push(out, 32575);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 12894);
							match s {
								[] => true,
								['g', 'n', 'e', 'l', 'h', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 23583);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 23584);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 12895);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 23585);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 12896);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 23586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 12897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4659);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12898);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4660);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 12899);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 23588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4661);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 12900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1083);
			match s {
				[] => true,
				['ü', s @ ..] => {
					super::push(out, 4663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1084);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4664);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4665);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 12901);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12902);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23589);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 23590);
									match s {
										[] => true,
										['d', 'n', 'a', s @ ..] => {
											super::push(out, 32576);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 32577);
											s.is_empty()
										}
										['s', s @ ..] => super::t5::n34230(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => super::t5::n4690(s, out),
				['r', s @ ..] => super::t5::n4691(s, out),
				['w', s @ ..] => super::t5::n4692(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1085);
			s.is_empty()
		}
		['i', s @ ..] => super::t5::n1088(s, out),
		['l', s @ ..] => super::t5::n1089(s, out),
		['m', s @ ..] => super::t5::n1090(s, out),
		['n', s @ ..] => super::t5::n1091(s, out),
		['o', s @ ..] => super::t5::n1092(s, out),
		['r', s @ ..] => super::t5::n1093(s, out),
		['u', s @ ..] => super::t5::n1094(s, out),
		['y', s @ ..] => super::t5::n1095(s, out),
		['ä', s @ ..] => super::t5::n1096(s, out),
		['ö', s @ ..] => super::t5::n1097(s, out),
		['ü', s @ ..] => super::t5::n1098(s, out),
		_ => false,
	}
}

/// Words ending -ee.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1097);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4740);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4741);
					match s {
						[] => true,
						['i', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 13033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1098);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1099);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4742);
					match s {
						[] => true,
						['s', 't', s @ ..] => {
							super::push(out, 13034);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 23716);
									s.is_empty()
								}
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
			super::push(out, 1100);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4743);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 4744);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4745);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13035);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13036);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 23717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4746);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13037);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 13038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', 't', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 4747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1101);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4748);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4749);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4750);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1102);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4752);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13039);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23718);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 23719);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 23720);
									match s {
										[] => true,
										['m', '-', s @ ..] => {
											super::push(out, 32658);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 13040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4753);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4754);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 13041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1103);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4755);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4756);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4757);
					match s {
						[] => true,
						['r', 'e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 13042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1104);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4758);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4759);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4760);
					match s {
						[] => true,
						['m', 'e', 's', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13043);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4761);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 13045);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 13046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4762);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 13047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4763);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 13048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4764);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13049);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4765);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4766);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 13052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1105);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4767);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 13053);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 13054);
							match s {
								[] => true,
								['k', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 23721);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4768);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 13055);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 23722);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 23723);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4769);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									super::push(out, 23724);
									s.is_empty()
								}
								['l', 'a', 't', s @ ..] => {
									super::push(out, 23725);
									s.is_empty()
								}
								['n', 'e', 'i', s @ ..] => {
									super::push(out, 23726);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'u', 'i', 'e', 'r', 'f', 'e', 'b', 's', s @ ..] => {
											super::push(out, 32659);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 32660);
											s.is_empty()
										}
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
					super::push(out, 4770);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 13056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1106);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4771);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4772);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13057);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13058);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23727);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 13059);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4773);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 13061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4774);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 13062);
							match s {
								[] => true,
								['o', 't', 't', 'r', 'e', 'z', 'n', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 23728);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4775);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t5::n1109(s, out),
		['r', s @ ..] => super::t5::n1110(s, out),
		['s', s @ ..] => super::t5::n1111(s, out),
		['t', s @ ..] => super::t5::n1112(s, out),
		['v', s @ ..] => {
			super::push(out, 1111);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => super::t5::n1114(s, out),
		_ => false,
	}
}

/// Words ending -fe.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1113);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4812);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4813);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4814);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13094);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1114);
			match s {
				[] => true,
				['i', 't', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 13096);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							super::push(out, 13097);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 23736);
									s.is_empty()
								}
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
			super::push(out, 1115);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13098);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 13099);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 13100);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 23737);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 13101);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23738);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 13102);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13103);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 13104);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13105);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23739);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 32662);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 32663);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 23740);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 23741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 13106);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13107);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 13108);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 23742);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 32664);
											match s {
												[] => true,
												['u', 'l', 's', s @ ..] => {
													super::push(out, 38641);
													s.is_empty()
												}
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
					super::push(out, 4816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13109);
							s.is_empty()
						}
						['n', 'ß', s @ ..] => {
							super::push(out, 13110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4817);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 13111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4818);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 4819);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 13112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 4820);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 13113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1116);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 13114);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 13115);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 13116);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 13117);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 23744);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 23745);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 23746);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 23747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 13118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4821);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 13119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1117);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13120);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13121);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 13122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13123);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 23748);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 23749);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 32665);
											match s {
												[] => true,
												['i', 'b', 'b', 'a', '\u{1}', s @ ..] => super::t5::n40956(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => super::t5::n24555(s, out),
								['r', 'r', 'ü', 'd', '\u{1}', s @ ..] => super::t5::n24556(s, out),
								['s', 'e', 'l', '\u{1}', s @ ..] => super::t5::n24557(s, out),
								_ => false,
							}
						}
						['h', 'e', s @ ..] => super::t5::n13322(s, out),
						['l', 'a', 'i', s @ ..] => super::t5::n13323(s, out),
						['n', s @ ..] => super::t5::n13324(s, out),
						['r', s @ ..] => super::t5::n13325(s, out),
						['s', s @ ..] => super::t5::n13326(s, out),
						['t', s @ ..] => super::t5::n13327(s, out),
						_ => false,
					}
				}
				['ü', s @ ..] => super::t5::n4850(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t5::n1120(s, out),
		['r', s @ ..] => super::t5::n1121(s, out),
		['u', s @ ..] => super::t5::n1122(s, out),
		_ => false,
	}
}

/// Words ending -ge.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1121);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4832);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4833);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 13141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4834);
					match s {
						[] => true,
						['n', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 13142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4835);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 13143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4836);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13144);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 13145);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 13146);
							match s {
								[] => true,
								['o', 'i', 'r', s @ ..] => {
									super::push(out, 23760);
									match s {
										[] => true,
										['e', 't', 'k', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 32670);
											s.is_empty()
										}
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
					super::push(out, 4837);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13147);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 23761);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 32671);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 38643);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 'n', 'k', '\u{1}', s @ ..] => {
									super::push(out, 23762);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 23763);
									s.is_empty()
								}
								['m', 'u', 'j', '\u{1}', s @ ..] => {
									super::push(out, 23764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 13148);
							match s {
								[] => true,
								['e', 'b', 'h', s @ ..] => {
									super::push(out, 23765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 13149);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13150);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23766);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 32672);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13151);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 13152);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 23767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13153);
							match s {
								[] => true,
								['e', 'p', s @ ..] => {
									super::push(out, 23768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4838);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13154);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 13155);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 'd', 'n', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 13157);
							s.is_empty()
						}
						['t', 'r', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 13158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4839);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13159);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13160);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 23770);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 23771);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 23772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13161);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13162);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13163);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13164);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t5::n4868(s, out),
				['s', s @ ..] => {
					super::push(out, 4842);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13173);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 23783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => super::t5::n13372(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t5::n4870(s, out),
				['v', 'o', 'r', 'y', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4844);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4845);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 13182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t5::n1124(s, out),
		['e', s @ ..] => super::t5::n1125(s, out),
		['g', s @ ..] => super::t5::n1126(s, out),
		['h', s @ ..] => super::t5::n1127(s, out),
		['i', s @ ..] => super::t5::n1128(s, out),
		['l', s @ ..] => super::t5::n1129(s, out),
		['n', s @ ..] => super::t5::n1130(s, out),
		['o', s @ ..] => super::t5::n1131(s, out),
		['r', s @ ..] => super::t5::n1132(s, out),
		['u', s @ ..] => super::t5::n1133(s, out),
		['ä', s @ ..] => super::t5::n1134(s, out),
		['è', s @ ..] => super::t5::n1135(s, out),
		['ü', s @ ..] => super::t5::n1136(s, out),
		_ => false,
	}
}

/// Words ending -he.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['c', s @ ..] => super::t6::n1137(s, out),
		['e', s @ ..] => {
			super::push(out, 1136);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4915);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 13318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1137);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1138);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4918);
					match s {
						[] => true,
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 13319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1139);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4919);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 13320);
							match s {
								[] => true,
								['r', 'a', 'a', s @ ..] => {
									super::push(out, 23958);
									s.is_empty()
								}
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
			super::push(out, 1140);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4920);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 13321);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 23959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4921);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1141);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4923);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 13323);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 13324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4924);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 4925);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 13325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'f', s @ ..] => {
									super::push(out, 23961);
									s.is_empty()
								}
								['u', 'n', s @ ..] => {
									super::push(out, 23962);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13326);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13327);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13328);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 23963);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 23964);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 32763);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 23965);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 23966);
									s.is_empty()
								}
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
			super::push(out, 1143);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4926);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4927);
					match s {
						[] => true,
						['n', 'e', 'n', s @ ..] => {
							super::push(out, 13329);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 23967);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4928);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 13330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4929);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 4930);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 1144);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4931);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 13332);
							s.is_empty()
						}
						['l', 'e', 'h', s @ ..] => {
							super::push(out, 13333);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 32764);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 32765);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 23968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 13334);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4932);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 13336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 1145);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 13337);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 13338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ie.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1146);
			match s {
				[] => true,
				['k', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 4933);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4934);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 13339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1147);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4936);
					match s {
						[] => true,
						['l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 13340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 13341);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 13342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4937);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 13343);
							match s {
								[] => true,
								['z', '-', 'i', 'z', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 23969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4938);
					match s {
						[] => true,
						['h', 'p', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 32766);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 32767);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 23970);
									s.is_empty()
								}
								['n', 'e', 'n', s @ ..] => {
									super::push(out, 23971);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 32768);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 32769);
											s.is_empty()
										}
										['i', 'l', s @ ..] => {
											super::push(out, 32770);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 32771);
											s.is_empty()
										}
										['n', 'h', s @ ..] => {
											super::push(out, 32772);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 32773);
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 38687);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 38688);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 38689);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 38690);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 32774);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 38691);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 23972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4939);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 4940);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 13345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4941);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4942);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 13346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4943);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13347);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4944);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4945);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 23973);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 32775);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'p', 'y', 'l', 'o', 'p', 'o', s @ ..] => {
									super::push(out, 23974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4946);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13349);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 13350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4947);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 13351);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13352);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 23975);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 23976);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32776);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 23977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4948);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13353);
							match s {
								[] => true,
								['k', 'y', s @ ..] => {
									super::push(out, 23978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 13354);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 23979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4949);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 4950);
					match s {
						[] => true,
						['p', 'o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 23980);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 23981);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 23982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 4951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t6::n1151(s, out),
		['f', s @ ..] => super::t6::n1152(s, out),
		['g', s @ ..] => super::t6::n1153(s, out),
		['h', s @ ..] => super::t6::n1154(s, out),
		['k', s @ ..] => super::t6::n1155(s, out),
		['l', s @ ..] => super::t6::n1156(s, out),
		['m', s @ ..] => super::t6::n1157(s, out),
		['n', s @ ..] => super::t6::n1158(s, out),
		['p', s @ ..] => super::t6::n1159(s, out),
		['r', s @ ..] => super::t6::n1160(s, out),
		['s', s @ ..] => super::t6::n1161(s, out),
		['t', s @ ..] => super::t6::n1162(s, out),
		['u', s @ ..] => super::t6::n1163(s, out),
		['v', s @ ..] => super::t6::n1164(s, out),
		['w', s @ ..] => super::t6::n1165(s, out),
		['x', s @ ..] => super::t6::n1166(s, out),
		['z', s @ ..] => super::t6::n1167(s, out),
		_ => false,
	}
}

/// Words ending -je.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1166);
			match s {
				[] => true,
				['d', 'i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 5046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1167);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1168);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5047);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1169);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5049);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1170);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5051);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1171);
			match s {
				[] => true,
				['n', 'u', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5052);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5053);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ke.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5054);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5055);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 5056);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 13509);
							match s {
								[] => true,
								['t', 'o', 'r', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 24126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 13510);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 13511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5057);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5058);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 13513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5059);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 13514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5060);
					match s {
						[] => true,
						['n', 's', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 13515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5061);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13516);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 13517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5062);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5063);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5064);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13519);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13520);
							s.is_empty()
						}
						['o', 't', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 13521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5065);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5066);
					match s {
						[] => true,
						['k', 'n', 'e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 24127);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 24128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5067);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5068);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13522);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 24129);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 24130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 13523);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13524);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5069);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13526);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 5070);
					match s {
						[] => true,
						['h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 13528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1173);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5071);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 13529);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 13530);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 13531);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13532);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 24131);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13533);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 24132);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 24133);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13534);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 24134);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32882);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 's', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 13535);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5072);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13537);
							match s {
								[] => true,
								['n', 'o', s @ ..] => {
									super::push(out, 24135);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 32883);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 13538);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 24136);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 32884);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 13539);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24137);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 13540);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 24138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13541);
							match s {
								[] => true,
								['h', 'c', 's', 'n', 'a', s @ ..] => super::t6::n24973(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t6::n13774(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t6::n5109(s, out),
				['o', s @ ..] => super::t6::n5110(s, out),
				['u', s @ ..] => super::t6::n5111(s, out),
				['ä', s @ ..] => super::t6::n5112(s, out),
				['ü', s @ ..] => super::t6::n5113(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t6::n1176(s, out),
		['f', s @ ..] => super::t6::n1177(s, out),
		['h', s @ ..] => {
			super::push(out, 1176);
			s.is_empty()
		}
		['i', s @ ..] => super::t6::n1179(s, out),
		['k', s @ ..] => super::t6::n1180(s, out),
		['l', s @ ..] => super::t6::n1181(s, out),
		['n', s @ ..] => super::t6::n1182(s, out),
		['o', s @ ..] => super::t6::n1183(s, out),
		['p', s @ ..] => super::t6::n1184(s, out),
		['r', s @ ..] => super::t6::n1185(s, out),
		['s', s @ ..] => super::t6::n1186(s, out),
		['t', s @ ..] => super::t6::n1187(s, out),
		['u', s @ ..] => super::t6::n1188(s, out),
		['y', s @ ..] => super::t6::n1189(s, out),
		['z', s @ ..] => super::t6::n1190(s, out),
		['ö', s @ ..] => super::t6::n1191(s, out),
		_ => false,
	}
}

/// Words ending -le.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1190);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5131);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5132);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13603);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5133);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5134);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13606);
							match s {
								[] => true,
								['h', 'c', 's', 't', 'm', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 24180);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13607);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 24181);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 32898);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 38810);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 32899);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 32900);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 24182);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 32901);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13608);
							match s {
								[] => true,
								['n', 'y', 's', '\u{1}', s @ ..] => {
									super::push(out, 24183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5135);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 13609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5136);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13610);
							match s {
								[] => true,
								['l', 'l', 'i', '\u{1}', s @ ..] => {
									super::push(out, 24184);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5137);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 13612);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 24185);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 32902);
											match s {
												[] => true,
												['e', 'n', 'o', s @ ..] => {
													super::push(out, 38811);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 32903);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 13613);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5138);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 13614);
							match s {
								[] => true,
								['r', 'e', 'v', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 24187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 13615);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 24188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 13616);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13617);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 13618);
							match s {
								[] => true,
								['e', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 24189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13619);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 24190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13620);
							match s {
								[] => true,
								['o', 's', 's', 'i', 'm', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 24191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 13621);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 13622);
							match s {
								[] => true,
								['u', s @ ..] => super::t6::n25027(s, out),
								_ => false,
							}
						}
						['z', s @ ..] => super::t6::n13859(s, out),
						_ => false,
					}
				}
				['k', s @ ..] => super::t6::n5176(s, out),
				['m', s @ ..] => super::t6::n5177(s, out),
				['n', s @ ..] => super::t6::n5178(s, out),
				['r', s @ ..] => super::t6::n5179(s, out),
				['s', s @ ..] => super::t6::n5180(s, out),
				['t', s @ ..] => super::t6::n5181(s, out),
				['u', s @ ..] => super::t6::n5182(s, out),
				['v', s @ ..] => super::t6::n5183(s, out),
				['x', s @ ..] => super::t6::n5184(s, out),
				['z', s @ ..] => super::t6::n5185(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t6::n1193(s, out),
		['c', s @ ..] => super::t6::n1194(s, out),
		['d', s @ ..] => super::t6::n1195(s, out),
		['e', s @ ..] => super::t6::n1196(s, out),
		['f', s @ ..] => super::t6::n1197(s, out),
		['g', s @ ..] => super::t6::n1198(s, out),
		['h', s @ ..] => super::t6::n1199(s, out),
		['i', s @ ..] => super::t6::n1200(s, out),
		['k', s @ ..] => super::t6::n1201(s, out),
		['l', s @ ..] => super::t6::n1202(s, out),
		['m', s @ ..] => super::t6::n1203(s, out),
		['o', s @ ..] => super::t6::n1204(s, out),
		['p', s @ ..] => super::t6::n1205(s, out),
		['r', s @ ..] => super::t6::n1206(s, out),
		['s', s @ ..] => super::t6::n1207(s, out),
		['t', s @ ..] => super::t6::n1208(s, out),
		['u', s @ ..] => super::t6::n1209(s, out),
		['w', s @ ..] => super::t6::n1210(s, out),
		['y', s @ ..] => super::t6::n1211(s, out),
		['z', s @ ..] => super::t6::n1212(s, out),
		['ö', s @ ..] => super::t6::n1213(s, out),
		['ü', s @ ..] => super::t6::n1214(s, out),
		_ => false,
	}
}

/// Words ending -me.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1213);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1214);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5240);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 13782);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['a', s @ ..] => {
							super::push(out, 13783);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 24322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 13784);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 13785);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 24323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13786);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 13787);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13788);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5242);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 13790);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 13791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5243);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5244);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13793);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 24324);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 32987);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 13794);
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									super::push(out, 24325);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 32988);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13795);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 24326);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 32989);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13796);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13797);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 24327);
									match s {
										[] => true,
										['n', 'a', s @ ..] => {
											super::push(out, 32990);
											match s {
												[] => true,
												['w', 'z', '\u{1}', s @ ..] => {
													super::push(out, 38833);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 24328);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 32991);
											match s {
												[] => true,
												['e', 'r', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 38834);
													s.is_empty()
												}
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
							super::push(out, 13798);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 24329);
									match s {
										[] => true,
										['o', 's', 'i', '\u{1}', s @ ..] => {
											super::push(out, 32992);
											s.is_empty()
										}
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
					super::push(out, 5245);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 13799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13800);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13801);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13802);
							s.is_empty()
						}
						['r', 'h', 'a', 'w', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 13803);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13804);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 13805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1215);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5247);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 5248);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 13806);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5249);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5251);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5252);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 13809);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13810);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1216);
			match s {
				[] => true,
				['a', s @ ..] => super::t6::n5291(s, out),
				['ö', s @ ..] => {
					super::push(out, 5255);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 13814);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t6::n1219(s, out),
		['l', s @ ..] => {
			super::push(out, 1218);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1219);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5264);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5265);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 13822);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t6::n25191(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t6::n5303(s, out),
				['o', s @ ..] => super::t6::n5304(s, out),
				['u', s @ ..] => super::t6::n5305(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t6::n1222(s, out),
		['r', s @ ..] => super::t6::n1223(s, out),
		['u', s @ ..] => super::t6::n1224(s, out),
		['ä', s @ ..] => super::t6::n1225(s, out),
		['è', s @ ..] => super::t6::n1226(s, out),
		_ => false,
	}
}

/// Words ending -ne.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1225);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5280);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['l', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 13840);
							s.is_empty()
						}
						['n', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 13841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5281);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5282);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 5283);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5284);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5285);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13843);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 24354);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 24355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 13844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5286);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 24356);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 33004);
											match s {
												[] => true,
												['t', 'ä', 'r', '\u{1}', s @ ..] => {
													super::push(out, 38840);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 24357);
									match s {
										[] => true,
										['p', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 33005);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 't', s @ ..] => {
									super::push(out, 24358);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 33006);
											s.is_empty()
										}
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
					super::push(out, 5287);
					s.is_empty()
				}
				['p', 'm', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 5288);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 13845);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 13846);
							s.is_empty()
						}
						['o', 'm', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 13847);
							s.is_empty()
						}
						['r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 13848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'n', 'a', 'g', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5289);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5290);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 13849);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 5291);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13850);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 13851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1226);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5293);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', 'r', s @ ..] => {
									super::push(out, 24360);
									match s {
										[] => true,
										['e', 't', 'n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 33007);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 24361);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 33008);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 33009);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 33010);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 24362);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 24363);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 33011);
											match s {
												[] => true,
												['d', 'n', 'u', s @ ..] => {
													super::push(out, 38841);
													s.is_empty()
												}
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
							super::push(out, 13853);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 24364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5294);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13854);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5295);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 13856);
							match s {
								[] => true,
								['s', 't', s @ ..] => {
									super::push(out, 24365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 13857);
							match s {
								[] => true,
								['u', 'r', 'ß', 'i', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 24366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5296);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 13858);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 13859);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 24367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => super::t6::n14103(s, out),
						['m', s @ ..] => super::t6::n14104(s, out),
						_ => false,
					}
				}
				['k', s @ ..] => super::t6::n5337(s, out),
				['l', s @ ..] => super::t6::n5338(s, out),
				['m', s @ ..] => super::t6::n5339(s, out),
				['r', s @ ..] => super::t6::n5340(s, out),
				['s', s @ ..] => super::t6::n5341(s, out),
				['v', s @ ..] => super::t6::n5342(s, out),
				['w', s @ ..] => super::t6::n5343(s, out),
				['z', s @ ..] => super::t6::n5344(s, out),
				['ß', s @ ..] => super::t6::n5345(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t6::n1229(s, out),
		['h', s @ ..] => super::t6::n1230(s, out),
		['i', s @ ..] => super::t6::n1231(s, out),
		['k', s @ ..] => super::t6::n1232(s, out),
		['l', s @ ..] => super::t6::n1233(s, out),
		['m', s @ ..] => super::t6::n1234(s, out),
		['n', s @ ..] => super::t6::n1235(s, out),
		['o', s @ ..] => super::t6::n1236(s, out),
		['r', s @ ..] => super::t6::n1237(s, out),
		['u', s @ ..] => super::t6::n1238(s, out),
		['y', s @ ..] => super::t6::n1239(s, out),
		['ä', s @ ..] => super::t6::n1240(s, out),
		['ö', s @ ..] => super::t6::n1241(s, out),
		['ü', s @ ..] => super::t6::n1242(s, out),
		_ => false,
	}
}

/// Words ending -oe.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1241);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1242);
			match s {
				[] => true,
				['p', 's', 'y', 'd', 's', s @ ..] => {
					super::push(out, 5379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1243);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1244);
			match s {
				[] => true,
				['-', 'c', 'a', 't', '-', 'c', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 5381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1245);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5382);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pe.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1246);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5383);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 14009);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14010);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									super::push(out, 24452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5384);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5385);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1247);
			match s {
				[] => true,
				['i', 'p', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5387);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1248);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5389);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1249);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5391);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5392);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 14013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5393);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 5394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1250);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5395);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5396);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 14014);
							match s {
								[] => true,
								['s', 'e', '\u{1}', s @ ..] => {
									super::push(out, 24453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5397);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 14015);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24454);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', s @ ..] => {
							super::push(out, 14016);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 24455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 5398);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 14017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1251);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5400);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5401);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5402);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5403);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 5404);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 14020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1252);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5405);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 14021);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24456);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 24457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14022);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 24458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14023);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24459);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 24460);
									match s {
										[] => true,
										['ß', s @ ..] => {
											super::push(out, 33037);
											s.is_empty()
										}
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
					super::push(out, 5406);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5407);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14024);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14025);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24461);
									match s {
										[] => true,
										['d', 'n', 'ä', 'l', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 33038);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 33039);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 24462);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 33040);
											s.is_empty()
										}
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
					super::push(out, 5408);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 14026);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 24463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 14027);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 24464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t6::n5451(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1253);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1254);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5410);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 14031);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => super::t6::n5453(s, out),
				['j', s @ ..] => super::t6::n5454(s, out),
				['k', s @ ..] => super::t6::n5455(s, out),
				['l', s @ ..] => {
					super::push(out, 5414);
					s.is_empty()
				}
				['n', s @ ..] => super::t6::n5457(s, out),
				['o', s @ ..] => super::t6::n5458(s, out),
				['p', s @ ..] => super::t6::n5459(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t6::n1257(s, out),
		['z', s @ ..] => super::t6::n1258(s, out),
		['ê', s @ ..] => super::t6::n1259(s, out),
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1258);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1259);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5421);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 14039);
							match s {
								[] => true,
								['e', 'u', 'e', 'n', 'r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 24471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5422);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5423);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 14041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5424);
					match s {
						[] => true,
						['l', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5425);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 14043);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24472);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14044);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14045);
							match s {
								[] => true,
								['a', 'y', 'm', '\u{1}', s @ ..] => {
									super::push(out, 24473);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5427);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5428);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5429);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 14048);
							match s {
								[] => true,
								['a', 'g', 'l', 'o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 24474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 14049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5430);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 5431);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14050);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24475);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 24476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14051);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 24477);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 24478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 14052);
							match s {
								[] => true,
								['e', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 24479);
									s.is_empty()
								}
								['g', 'e', 'l', 's', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 24480);
									s.is_empty()
								}
								['r', 'a', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 24481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 14053);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 14054);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 14055);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14056);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 24482);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 33051);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 14057);
							match s {
								[] => true,
								['o', 's', 'n', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 24483);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 24484);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 24485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14058);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 24486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14059);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 24487);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 14060);
							match s {
								[] => true,
								['u', 'o', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 24488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14061);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24489);
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 33052);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 38857);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 24490);
									match s {
										[] => true,
										['p', 'a', 'v', '\u{1}', s @ ..] => {
											super::push(out, 33053);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14062);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14063);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 24491);
									match s {
										[] => true,
										['o', 's', 's', 'g', 'n', 'u', 's', s @ ..] => {
											super::push(out, 33054);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 24492);
									s.is_empty()
								}
								['k', 'r', 'a', 'm', 'z', 'r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 24493);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 24494);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 24495);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 14064);
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 24496);
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 33055);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 14065);
							match s {
								[] => true,
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 24497);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1260);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1261);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1262);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5433);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 14066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t6::n5476(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t6::n1265(s, out),
		['f', s @ ..] => super::t6::n1266(s, out),
		['h', s @ ..] => super::t6::n1267(s, out),
		['i', s @ ..] => super::t6::n1268(s, out),
		['n', s @ ..] => super::t6::n1269(s, out),
		['o', s @ ..] => super::t6::n1270(s, out),
		['r', s @ ..] => super::t6::n1271(s, out),
		['t', s @ ..] => super::t6::n1272(s, out),
		['u', s @ ..] => super::t6::n1273(s, out),
		['v', s @ ..] => super::t6::n1274(s, out),
		['ä', s @ ..] => super::t6::n1275(s, out),
		['è', s @ ..] => super::t6::n1276(s, out),
		['ö', s @ ..] => super::t6::n1277(s, out),
		['ü', s @ ..] => super::t6::n1278(s, out),
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1277);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1278);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5512);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5513);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 5514);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 14158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5515);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5516);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14159);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14161);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 14162);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 24552);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 24553);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14163);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14164);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 14165);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 14166);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 14167);
							s.is_empty()
						}
						['i', 'g', 'g', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 14168);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14169);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14170);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14171);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14172);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 24554);
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 33095);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 38898);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 24555);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 33096);
											s.is_empty()
										}
										['g', 'n', 'u', 'r', 'e', 'i', 's', s @ ..] => {
											super::push(out, 33097);
											match s {
												[] => true,
												['i', 'l', 'a', 'i', 'r', s @ ..] => {
													super::push(out, 38899);
													s.is_empty()
												}
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
							super::push(out, 14173);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14174);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14176);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 14177);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 24556);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 33098);
											s.is_empty()
										}
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
					super::push(out, 5519);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14178);
							match s {
								[] => true,
								['k', 'e', s @ ..] => {
									super::push(out, 24557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5520);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5521);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14179);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 24558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1279);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5523);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5524);
					match s {
						[] => true,
						['h', 'c', 's', 'e', s @ ..] => {
							super::push(out, 14180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1280);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5525);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => super::t6::n5568(s, out),
				['i', s @ ..] => super::t6::n5569(s, out),
				['k', s @ ..] => super::t6::n5570(s, out),
				['l', s @ ..] => super::t6::n5571(s, out),
				['m', s @ ..] => super::t6::n5572(s, out),
				['n', s @ ..] => super::t6::n5573(s, out),
				['o', s @ ..] => super::t6::n5574(s, out),
				['r', s @ ..] => super::t6::n5575(s, out),
				['t', s @ ..] => super::t6::n5576(s, out),
				['u', s @ ..] => super::t6::n5577(s, out),
				['v', s @ ..] => super::t6::n5578(s, out),
				['w', s @ ..] => super::t6::n5579(s, out),
				['x', s @ ..] => super::t6::n5580(s, out),
				['z', s @ ..] => super::t6::n5581(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t6::n1283(s, out),
		['h', s @ ..] => super::t6::n1284(s, out),
		['i', s @ ..] => super::t6::n1285(s, out),
		['k', s @ ..] => super::t6::n1286(s, out),
		['l', s @ ..] => super::t6::n1287(s, out),
		['m', s @ ..] => super::t6::n1288(s, out),
		['n', s @ ..] => super::t6::n1289(s, out),
		['o', s @ ..] => super::t6::n1290(s, out),
		['p', s @ ..] => super::t6::n1291(s, out),
		['r', s @ ..] => super::t6::n1292(s, out),
		['s', s @ ..] => super::t6::n1293(s, out),
		['t', s @ ..] => super::t6::n1294(s, out),
		['u', s @ ..] => super::t6::n1295(s, out),
		['y', s @ ..] => super::t6::n1296(s, out),
		['ä', s @ ..] => super::t6::n1297(s, out),
		['ö', s @ ..] => super::t6::n1298(s, out),
		['ü', s @ ..] => super::t6::n1299(s, out),
		_ => false,
	}
}

/// Words ending -te.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5601);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 14353);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 14354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 5602);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14355);
							match s {
								[] => true,
								['n', 'i', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 24689);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 14356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5603);
					match s {
						[] => true,
						['l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 14357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5604);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5605);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 14359);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 14360);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14361);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 24690);
									match s {
										[] => true,
										['a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 33161);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 24691);
									match s {
										[] => true,
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 33162);
											s.is_empty()
										}
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
					super::push(out, 5606);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 14362);
							match s {
								[] => true,
								['t', 'a', 'p', 'x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 24692);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 5607);
					match s {
						[] => true,
						['r', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5608);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14365);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14366);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14367);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 24693);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 24694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14368);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 14369);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14370);
							match s {
								[] => true,
								['m', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 24695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5610);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14371);
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 24696);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14372);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14373);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 24697);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 24698);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 14374);
							match s {
								[] => true,
								['a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 24699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5611);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5612);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 14375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5613);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5614);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24700);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 24701);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33163);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 14377);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 24702);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14378);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 24703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14379);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14380);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 24704);
									match s {
										[] => true,
										['l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 33164);
											s.is_empty()
										}
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
					super::push(out, 5615);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14381);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14382);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14383);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14384);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14385);
							match s {
								[] => true,
								['t', 'i', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 24705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1299);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5616);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5617);
					match s {
						[] => true,
						['a', 's', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 14386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1300);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5618);
					match s {
						[] => true,
						['a', 'w', 'r', 'e', 'v', 'z', s @ ..] => {
							super::push(out, 14387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t6::n1303(s, out),
		['f', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5628);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 14400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5629);
					s.is_empty()
				}
				['p', 'o', 't', 's', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5630);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5631);
					match s {
						[] => true,
						['ä', s @ ..] => {
							super::push(out, 14401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => super::t6::n5679(s, out),
				['ö', s @ ..] => super::t6::n5680(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t6::n1305(s, out),
		['h', s @ ..] => super::t6::n1306(s, out),
		['i', s @ ..] => super::t6::n1307(s, out),
		['j', s @ ..] => super::t6::n1308(s, out),
		['k', s @ ..] => super::t6::n1309(s, out),
		['l', s @ ..] => super::t6::n1310(s, out),
		['m', s @ ..] => super::t6::n1311(s, out),
		['n', s @ ..] => super::t6::n1312(s, out),
		['o', s @ ..] => super::t6::n1313(s, out),
		['r', s @ ..] => super::t6::n1314(s, out),
		['s', s @ ..] => super::t6::n1315(s, out),
		['t', s @ ..] => super::t6::n1316(s, out),
		['u', s @ ..] => super::t6::n1317(s, out),
		['y', s @ ..] => super::t6::n1318(s, out),
		['z', s @ ..] => super::t6::n1319(s, out),
		['ä', s @ ..] => super::t6::n1320(s, out),
		['ê', s @ ..] => super::t6::n1321(s, out),
		['ö', s @ ..] => super::t7::n1322(s, out),
		['ü', s @ ..] => super::t7::n1323(s, out),
		_ => false,
	}
}

