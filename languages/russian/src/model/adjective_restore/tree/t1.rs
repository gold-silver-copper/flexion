use flexion::Path;

/// Words ending -утый.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 500);
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1107);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1972);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 2959);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2960);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1974);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1975);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 1111);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1977);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 2962);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 2963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1978);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2964);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 1979);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1981);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2966);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2967);
									match s {
										[] => true,
										['ч', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 4139);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 4140);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 2968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 1982);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 1113);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1986);
							match s {
								[] => true,
								['в', 'ы', s @ ..] => {
									super::push(out, 2969);
									s.is_empty()
								}
								['д', 'ы', s @ ..] => {
									super::push(out, 2970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 1987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1988);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 2971);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 4141);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 4142);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 2972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1115);
					match s {
						[] => true,
						['е', 'м', 'з', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1991);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1992);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1993);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2973);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 2974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1118);
					match s {
						[] => true,
						['т', 'о', s @ ..] => {
							super::push(out, 1994);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 2975);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 3787);
											s.is_empty()
										}
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

/// Words ending -ытый.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	s.is_empty()
}

/// Words ending -ятый.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	s.is_empty()
}

/// Words ending -вавшийся.
pub(crate) fn n3178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2977);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3788);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3789);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4143);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4144);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 4145);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4308);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4309);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4310);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 4311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 4146);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4312);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4313);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 4415);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 4416);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 4401);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4402);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 4403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4314);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4147);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4316);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4317);
							s.is_empty()
						}
						['с', 'е', s @ ..] => {
							super::push(out, 4318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4148);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4149);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4150);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4151);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3790);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3791);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 3792);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4153);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4319);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4320);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4321);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4323);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4154);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4155);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4156);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4157);
					s.is_empty()
				}
				['п', 'е', s @ ..] => {
					super::push(out, 4158);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4159);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4325);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4326);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4160);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4328);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4329);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4330);
							s.is_empty()
						}
						['п', 'е', s @ ..] => {
							super::push(out, 4331);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4332);
							s.is_empty()
						}
						['с', 'р', s @ ..] => {
							super::push(out, 4333);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4334);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жившийся.
pub(crate) fn n3194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2993);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3796);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4163);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4164);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 4165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3797);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -увшийся.
pub(crate) fn n2107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1998);
	match s {
		[] => true,
		['н', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 3799);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3800);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4338);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 4339);
									s.is_empty()
								}
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

/// Words ending -гшийся.
pub(crate) fn n1170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1121);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2001);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2002);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2003);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дшийся.
pub(crate) fn n1171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1122);
	s.is_empty()
}

/// Words ending -зшийся.
pub(crate) fn n1172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1123);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2005);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2006);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2007);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кшийся.
pub(crate) fn n1173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1124);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2008);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2998);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2009);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2010);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2011);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ршийся.
pub(crate) fn n1174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1125);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3801);
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							super::push(out, 3802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3000);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сшийся.
pub(crate) fn n1175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1126);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2012);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['н', 'ы', s @ ..] => {
					super::push(out, 3001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2014);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2015);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тшийся.
pub(crate) fn n1176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1127);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3002);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3003);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вающийся.
pub(crate) fn n3209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3006);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3804);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3805);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4167);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 4168);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4169);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 4170);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4340);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4341);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4342);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 4343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 4171);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4344);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4345);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 4417);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 4418);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 4419);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 4404);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4405);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 4406);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4346);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4172);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4348);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4407);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4349);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4409);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 4420);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 4421);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 4410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4173);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4174);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4175);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4176);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3806);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4178);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4350);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4351);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4352);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4354);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4179);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4180);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4181);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4182);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4356);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4183);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4184);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4358);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4359);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4185);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4361);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4362);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4363);
							s.is_empty()
						}
						['п', 'е', s @ ..] => {
							super::push(out, 4364);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4365);
							s.is_empty()
						}
						['с', 'е', s @ ..] => {
							super::push(out, 4366);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4367);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новский.
pub(crate) fn n1198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1148);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2043);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3030);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2044);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 3031);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3812);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 3813);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3814);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 3815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'к', s @ ..] => {
					super::push(out, 3032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товский.
pub(crate) fn n1201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1151);
	s.is_empty()
}

/// Words ending -гский.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 512);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 2046);
							match s {
								[] => true,
								['е', 'ч', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 3816);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 4186);
													s.is_empty()
												}
												['и', s @ ..] => {
													super::push(out, 4187);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 3817);
											s.is_empty()
										}
										['ю', s @ ..] => {
											super::push(out, 3818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1153);
					match s {
						[] => true,
						['б', 'с', s @ ..] => {
							super::push(out, 2048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1154);
					match s {
						[] => true,
						['б', 'е', s @ ..] => {
							super::push(out, 2049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дский.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	s.is_empty()
}

/// Words ending -еский.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 514);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1156);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2051);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 3033);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 3819);
											match s {
												[] => true,
												['м', 'и', 'с', '-', 'о', 'н', 'н', s @ ..] => {
													super::push(out, 4188);
													s.is_empty()
												}
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
							super::push(out, 2052);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3034);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 3035);
									match s {
										[] => true,
										['е', 'а', s @ ..] => {
											super::push(out, 3820);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 3036);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3037);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3821);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 3038);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3822);
											match s {
												[] => true,
												['ч', 'а', s @ ..] => {
													super::push(out, 4189);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 3039);
									match s {
										[] => true,
										['е', 'т', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 4190);
													match s {
														[] => true,
														['о', 'н', 'т', s @ ..] => super::t1::n4653(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 4191);
													s.is_empty()
												}
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
							super::push(out, 2053);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2054);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 3040);
									match s {
										[] => true,
										['а', 'т', s @ ..] => {
											super::push(out, 3823);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 4192);
													s.is_empty()
												}
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

/// Words ending -жский.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	s.is_empty()
}

/// Words ending -зский.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	s.is_empty()
}

/// Words ending -йский.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	s.is_empty()
}

/// Words ending -кский.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	s.is_empty()
}

/// Words ending -мский.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	s.is_empty()
}

/// Words ending -нский.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 516);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 2055);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3041);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 3042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2056);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3043);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1159);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 2057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 1161);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2058);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 1162);
					match s {
						[] => true,
						['ь', 'л', 'е', '-', s @ ..] => {
							super::push(out, 2060);
							match s {
								[] => true,
								['о', 'к', 'с', 'в', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3824);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 3825);
											s.is_empty()
										}
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
			super::push(out, 519);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 520);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рский.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 522);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 523);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1163);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 2062);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 1165);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2065);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2066);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2067);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 2068);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 2069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1166);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2070);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 2071);
							s.is_empty()
						}
						['о', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 2072);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 2073);
							s.is_empty()
						}
						['ю', 'ь', 'б', 'а', '\u{1}', s @ ..] => {
							super::push(out, 2074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2075);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2076);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 3045);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 3046);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						['н', 'а', 'л', s @ ..] => {
							super::push(out, 2078);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2079);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1167);
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 2082);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 2083);
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 3047);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 3048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 2084);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 2087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', 'л', s @ ..] => {
							super::push(out, 2088);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2089);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 3826);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 3827);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 3049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 2090);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 3050);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3051);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2092);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 3053);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 3054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 2093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1169);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 2095);
							s.is_empty()
						}
						['к', 'о', 'б', 'к', s @ ..] => {
							super::push(out, 2096);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 2097);
							match s {
								[] => true,
								['и', 'ж', 'е', 'р', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 3828);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 3829);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['а', s @ ..] => super::t1::n2212(s, out),
						['е', s @ ..] => super::t1::n2213(s, out),
						['и', s @ ..] => super::t1::n2214(s, out),
						['й', s @ ..] => super::t1::n2215(s, out),
						['к', s @ ..] => super::t1::n2216(s, out),
						['л', s @ ..] => super::t1::n2217(s, out),
						['н', s @ ..] => super::t1::n2218(s, out),
						['о', 'л', s @ ..] => super::t1::n2219(s, out),
						['р', s @ ..] => super::t1::n2220(s, out),
						['с', s @ ..] => super::t1::n2221(s, out),
						['у', s @ ..] => super::t1::n2222(s, out),
						['ф', s @ ..] => super::t1::n2223(s, out),
						['х', s @ ..] => super::t1::n2224(s, out),
						['ь', s @ ..] => super::t1::n2225(s, out),
						_ => false,
					}
				}
				['ф', 'о', s @ ..] => super::t1::n1226(s, out),
				['ц', s @ ..] => super::t1::n1227(s, out),
				['ч', s @ ..] => super::t1::n1228(s, out),
				['ш', s @ ..] => super::t1::n1229(s, out),
				['ь', s @ ..] => super::t1::n1230(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t1::n542(s, out),
		['о', s @ ..] => super::t1::n543(s, out),
		['у', s @ ..] => super::t1::n544(s, out),
		['ы', s @ ..] => super::t1::n545(s, out),
		['я', s @ ..] => super::t1::n546(s, out),
		_ => false,
	}
}

/// Words ending -сский.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	s.is_empty()
}

/// Words ending -тский.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1178);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 531);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1179);
					match s {
						[] => true,
						['к', 'с', 'л', s @ ..] => {
							super::push(out, 2116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьский.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1180);
					match s {
						[] => true,
						['т', 'а', 'д', s @ ..] => {
							super::push(out, 2117);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3060);
									s.is_empty()
								}
								['з', 'и', '-', s @ ..] => {
									super::push(out, 3061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2118);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3062);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 3063);
									s.is_empty()
								}
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

/// Words ending -авший.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['и', s @ ..] => super::t1::n1262(s, out),
				['о', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2163);
							match s {
								[] => true,
								['т', 'с', s @ ..] => super::t1::n3309(s, out),
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 2164);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2165);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				['ы', s @ ..] => super::t1::n1266(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t1::n570(s, out),
		['д', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 553);
			s.is_empty()
		}
		['з', s @ ..] => super::t1::n573(s, out),
		['к', s @ ..] => super::t1::n574(s, out),
		['л', s @ ..] => {
			super::push(out, 556);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 557);
			s.is_empty()
		}
		['н', s @ ..] => super::t1::n577(s, out),
		['п', s @ ..] => super::t1::n578(s, out),
		['р', s @ ..] => super::t1::n579(s, out),
		['с', s @ ..] => super::t1::n580(s, out),
		['т', s @ ..] => super::t1::n581(s, out),
		['х', s @ ..] => super::t1::n582(s, out),
		['ц', s @ ..] => super::t1::n583(s, out),
		['ч', s @ ..] => super::t1::n584(s, out),
		['ш', s @ ..] => super::t1::n585(s, out),
		['щ', s @ ..] => super::t1::n586(s, out),
		_ => false,
	}
}

/// Words ending -ерзнувший.
pub(crate) fn n3377(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3881);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 3882);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3883);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 4223);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язнувший.
pub(crate) fn n2324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2206);
	s.is_empty()
}

/// Words ending -инувший.
pub(crate) fn n1320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1258);
	s.is_empty()
}

/// Words ending -кнувший.
pub(crate) fn n1321(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2207);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2208);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2209);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3171);
					match s {
						[] => true,
						['щ', 'ы', s @ ..] => {
							super::push(out, 3884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2210);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2211);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онувший.
pub(crate) fn n1322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1259);
	s.is_empty()
}

/// Words ending -пнувший.
pub(crate) fn n1323(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2213);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2214);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3173);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 3885);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 3886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3174);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 3175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2215);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2216);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2217);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнувший.
pub(crate) fn n1324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1260);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2218);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3176);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 3177);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3178);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 3887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2219);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2220);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2221);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -снувший.
pub(crate) fn n1325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1261);
	s.is_empty()
}

/// Words ending -тнувший.
pub(crate) fn n1326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1262);
	s.is_empty()
}

/// Words ending -унувший.
pub(crate) fn n1327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1263);
	s.is_empty()
}

/// Words ending -хнувший.
pub(crate) fn n1328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1264);
	s.is_empty()
}

/// Words ending -ьнувший.
pub(crate) fn n1329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1265);
	s.is_empty()
}

/// Words ending -юнувший.
pub(crate) fn n1330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1266);
	s.is_empty()
}

/// Words ending -янувший.
pub(crate) fn n1331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1267);
	s.is_empty()
}

/// Words ending -ывший.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	s.is_empty()
}

/// Words ending -явший.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	s.is_empty()
}

/// Words ending -ислеживающий.
pub(crate) fn n4665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4383);
	s.is_empty()
}

/// Words ending -кивающий.
pub(crate) fn n2355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2235);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3202);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3203);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3204);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3894);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3895);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4230);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 4231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 3896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3897);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3898);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4232);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4384);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											super::push(out, 4412);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 4385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3899);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3900);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывающий.
pub(crate) fn n1373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1307);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2243);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3205);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3206);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3901);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3902);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 4234);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 4235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3207);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3208);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3209);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3210);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3211);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2244);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3213);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2245);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 2246);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2247);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2248);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2249);
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3215);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2250);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2251);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3217);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3218);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3219);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3220);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 3221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2252);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3222);
					match s {
						[] => true,
						['б', 'а', 'р', 'б', s @ ..] => {
							super::push(out, 3903);
							match s {
								[] => true,
								['о', 'о', 'к', s @ ..] => {
									super::push(out, 4236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3223);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3904);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3225);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3226);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3906);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3907);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3908);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3909);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргающий.
pub(crate) fn n1374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1308);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дающий.
pub(crate) fn n633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1309);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2253);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3231);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3232);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 4237);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 4238);
											s.is_empty()
										}
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
		['ж', s @ ..] => {
			super::push(out, 1310);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зающий.
pub(crate) fn n635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					super::push(out, 2254);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кающий.
pub(crate) fn n636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1312);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2255);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1313);
			match s {
				[] => true,
				['е', 'ч', s @ ..] => {
					super::push(out, 2257);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нающий.
pub(crate) fn n639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 615);
	s.is_empty()
}

/// Words ending -пающий.
pub(crate) fn n640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 616);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1314);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3233);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 3234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 3235);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 3236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рающий.
pub(crate) fn n641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 617);
	s.is_empty()
}

/// Words ending -сающий.
pub(crate) fn n642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 618);
	s.is_empty()
}

/// Words ending -тающий.
pub(crate) fn n643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 619);
	s.is_empty()
}

/// Words ending -хающий.
pub(crate) fn n644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 620);
	s.is_empty()
}

/// Words ending -чающий.
pub(crate) fn n645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 621);
	match s {
		[] => true,
		['и', 'н', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2258);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3237);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шающий.
pub(crate) fn n646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 622);
	s.is_empty()
}

/// Words ending -щающий.
pub(crate) fn n647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 623);
	s.is_empty()
}

/// Words ending -тграммовый.
pub(crate) fn n2428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2302);
	s.is_empty()
}

/// Words ending -еммовый.
pub(crate) fn n1446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1373);
	s.is_empty()
}

/// Words ending -омовый.
pub(crate) fn n730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 703);
	s.is_empty()
}

/// Words ending -ьмовый.
pub(crate) fn n731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 704);
	s.is_empty()
}

/// Words ending -новый.
pub(crate) fn n327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 319);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 1374);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 2303);
							match s {
								[] => true,
								['а', 'к', '-', s @ ..] => {
									super::push(out, 3262);
									s.is_empty()
								}
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
			super::push(out, 706);
			match s {
				[] => true,
				['л', 'с', s @ ..] => {
					super::push(out, 1375);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 1376);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 707);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -повый.
pub(crate) fn n328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 320);
	s.is_empty()
}

/// Words ending -ровый.
pub(crate) fn n329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 321);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 708);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 709);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 710);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1377);
					s.is_empty()
				}
				['н', 'и', s @ ..] => {
					super::push(out, 1378);
					match s {
						[] => true,
						['л', 'и', 'ц', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 3263);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 3264);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3919);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3920);
											s.is_empty()
										}
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
			super::push(out, 712);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 713);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 714);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 3265);
									match s {
										[] => true,
										['т', 'н', s @ ..] => {
											super::push(out, 3921);
											match s {
												[] => true,
												['а', 'с', 'х', s @ ..] => {
													super::push(out, 4242);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 3266);
									match s {
										[] => true,
										['л', 'и', 'к', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 4243);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 4244);
													s.is_empty()
												}
												['х', s @ ..] => {
													super::push(out, 4245);
													match s {
														[] => true,
														['е', s @ ..] => super::t1::n4670(s, out),
														['у', s @ ..] => super::t1::n4671(s, out),
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
									super::push(out, 3267);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 3268);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3922);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3923);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 2305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 3269);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 3270);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3924);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3925);
											s.is_empty()
										}
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
					super::push(out, 1382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 716);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -совый.
pub(crate) fn n330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 322);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 717);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1383);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1384);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2306);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товый.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 323);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 718);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1385);
					match s {
						[] => true,
						['о', 'и', 'ф', s @ ..] => {
							match s {
								[] => true,
								['-', 'о', 'н', s @ ..] => {
									super::push(out, 3271);
									s.is_empty()
								}
								['о', 'н', s @ ..] => {
									super::push(out, 3272);
									s.is_empty()
								}
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
			super::push(out, 719);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 720);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1386);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1387);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ховый.
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 324);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 722);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 1389);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 1390);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цовый.
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 325);
	s.is_empty()
}

/// Words ending -лобообразный.
pub(crate) fn n4530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4253);
	s.is_empty()
}

/// Words ending -робообразный.
pub(crate) fn n4531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4254);
	s.is_empty()
}

/// Words ending -ефицированный.
pub(crate) fn n4532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4255);
	s.is_empty()
}

/// Words ending -ифицированный.
pub(crate) fn n4533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4256);
	s.is_empty()
}

/// Words ending -уганный.
pub(crate) fn n1618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1534);
	s.is_empty()
}

/// Words ending -данный.
pub(crate) fn n929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 891);
	s.is_empty()
}

/// Words ending -жанный.
pub(crate) fn n930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 892);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1535);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1536);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -занный.
pub(crate) fn n931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 893);
	s.is_empty()
}

/// Words ending -канный.
pub(crate) fn n932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 894);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1537);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1538);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2411);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'е', s @ ..] => {
			super::push(out, 1539);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2413);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2414);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 3349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2415);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2416);
					match s {
						[] => true,
						['л', 'п', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 3952);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 3953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 2417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 1540);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ланный.
pub(crate) fn n933(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1541);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1542);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -манный.
pub(crate) fn n934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 895);
	s.is_empty()
}

/// Words ending -нанный.
pub(crate) fn n935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 896);
	s.is_empty()
}

/// Words ending -панный.
pub(crate) fn n936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 897);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1544);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1545);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1546);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1547);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1548);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1549);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1550);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1551);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ранный.
pub(crate) fn n937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 898);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1553);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 1554);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 2418);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2419);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2420);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -санный.
pub(crate) fn n938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 899);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1555);
			match s {
				[] => true,
				['т', 'ы', s @ ..] => {
					super::push(out, 2421);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1556);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1557);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1558);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -танный.
pub(crate) fn n939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 900);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1560);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1561);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2423);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2424);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3350);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3351);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3352);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1562);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1563);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1564);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 1565);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2425);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1566);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2427);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3354);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2428);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1567);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ханный.
pub(crate) fn n940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 901);
	s.is_empty()
}

/// Words ending -чанный.
pub(crate) fn n941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 902);
	s.is_empty()
}

/// Words ending -шанный.
pub(crate) fn n942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 903);
	s.is_empty()
}

/// Words ending -енный.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 418);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 904);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1569);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 905);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1571);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2430);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2431);
							match s {
								[] => true,
								['г', 'ы', s @ ..] => {
									super::push(out, 3356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 2432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1572);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 906);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 1575);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 2433);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3357);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1576);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2434);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 2435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1577);
					match s {
						[] => true,
						['с', 'н', s @ ..] => {
							super::push(out, 2436);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3359);
									match s {
										[] => true,
										['р', 'т', 'с', 'о', 'р', 'п', '-', s @ ..] => {
											super::push(out, 3954);
											s.is_empty()
										}
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
			super::push(out, 907);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2437);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 2438);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 2439);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 2440);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 3360);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 3361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 2442);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 1580);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2444);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2445);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2446);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2447);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 3362);
									match s {
										[] => true,
										['о', 'в', 'с', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 4257);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 4258);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 3363);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 3364);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3955);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 3956);
											s.is_empty()
										}
										['о', 'н', s @ ..] => {
											super::push(out, 3957);
											match s {
												[] => true,
												['н', 'о', 'к', 'а', 'з', 'е', s @ ..] => {
													super::push(out, 4259);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 3958);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2448);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 3365);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 3366);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 3959);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 3960);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 3961);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 3367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				['й', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2449);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2450);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 3368);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 909);
			s.is_empty()
		}
		['ж', s @ ..] => super::t1::n949(s, out),
		['з', s @ ..] => {
			super::push(out, 911);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1600);
					match s {
						[] => true,
						['в', 'ы', s @ ..] => {
							super::push(out, 2486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 912);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 913);
			s.is_empty()
		}
		['л', s @ ..] => super::t1::n953(s, out),
		['м', s @ ..] => super::t1::n954(s, out),
		['н', s @ ..] => super::t1::n955(s, out),
		['о', s @ ..] => super::t1::n956(s, out),
		['п', s @ ..] => {
			super::push(out, 918);
			s.is_empty()
		}
		['р', s @ ..] => super::t1::n958(s, out),
		['с', s @ ..] => super::t1::n959(s, out),
		['т', s @ ..] => super::t1::n960(s, out),
		['у', s @ ..] => super::t1::n961(s, out),
		['ц', s @ ..] => super::t1::n962(s, out),
		['ч', s @ ..] => super::t1::n963(s, out),
		['ш', s @ ..] => super::t1::n964(s, out),
		['щ', s @ ..] => super::t1::n965(s, out),
		_ => false,
	}
}

/// Words ending -инный.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 419);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', 'х', s @ ..] => {
							super::push(out, 2744);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3677);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3678);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'в', 'х', s @ ..] => {
							super::push(out, 2745);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3679);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3680);
									s.is_empty()
								}
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

/// Words ending -онный.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 420);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['в', 'х', s @ ..] => {
							super::push(out, 2746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['п', 'а', 'и', 'д', 'х', s @ ..] => {
							super::push(out, 2747);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3681);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3682);
									s.is_empty()
								}
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
			super::push(out, 930);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1719);
					match s {
						[] => true,
						['л', 'и', 'м', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 3683);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 3684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 1720);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2748);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 3685);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4099);
											match s {
												[] => true,
												['п', 'о', 'х', s @ ..] => {
													super::push(out, 4294);
													match s {
														[] => true,
														['е', 'р', s @ ..] => super::t1::n4679(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 4100);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 4295);
													match s {
														[] => true,
														['р', 'е', 'п', '-', s @ ..] => super::t1::n4680(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 4101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 3686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 2749);
							match s {
								[] => true,
								['з', 'о', 'п', 'х', s @ ..] => {
									super::push(out, 3687);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 2750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['о', 'х', s @ ..] => {
					super::push(out, 1721);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 932);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 933);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 1723);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2752);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унный.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 421);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 934);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 2754);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 2755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынный.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 422);
	s.is_empty()
}

/// Words ending -янный.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	s.is_empty()
}

/// Words ending -нерный.
pub(crate) fn n986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 947);
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			super::push(out, 1755);
			match s {
				[] => true,
				['п', 'а', s @ ..] => {
					super::push(out, 2773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1756);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2774);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перный.
pub(crate) fn n987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1757);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2776);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1758);
			s.is_empty()
		}
		['о', 'к', s @ ..] => {
			super::push(out, 1759);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стерный.
pub(crate) fn n1858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1760);
	match s {
		[] => true,
		['о', 'к', s @ ..] => {
			super::push(out, 2778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ферный.
pub(crate) fn n990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 951);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 1761);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1762);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -черный.
pub(crate) fn n991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 952);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 1763);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьерный.
pub(crate) fn n993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	s.is_empty()
}

/// Words ending -ирный.
pub(crate) fn n437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['р', 'а', 'в', 'к', 'х', s @ ..] => {
					super::push(out, 1765);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2779);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 2780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орный.
pub(crate) fn n438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 956);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 1766);
					match s {
						[] => true,
						['в', '-', s @ ..] => {
							super::push(out, 2781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 1767);
					match s {
						[] => true,
						['е', 'ч', s @ ..] => {
							super::push(out, 2782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урный.
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	s.is_empty()
}

/// Words ending -юрный.
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 430);
	s.is_empty()
}

/// Words ending -ярный.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 431);
	s.is_empty()
}

/// Words ending -мно-красный.
pub(crate) fn n4575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4296);
	s.is_empty()
}

/// Words ending -нно-красный.
pub(crate) fn n4576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4297);
	s.is_empty()
}

/// Words ending -ато-красный.
pub(crate) fn n4577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4298);
	s.is_empty()
}

/// Words ending -лто-красный.
pub(crate) fn n4578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4299);
	match s {
		[] => true,
		['е', 'ж', '-', s @ ..] => {
			super::push(out, 4398);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -клетный.
pub(crate) fn n1896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1797);
	s.is_empty()
}

/// Words ending -олетный.
pub(crate) fn n1897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1798);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 2809);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 2810);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2811);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 2812);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2813);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2814);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3708);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плетный.
pub(crate) fn n1898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1799);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2815);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слетный.
pub(crate) fn n1899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1800);
	s.is_empty()
}

/// Words ending -котлетный.
pub(crate) fn n1900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1801);
	s.is_empty()
}

/// Words ending -мулетный.
pub(crate) fn n1901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1802);
	s.is_empty()
}

/// Words ending -флетный.
pub(crate) fn n1902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1803);
	s.is_empty()
}

/// Words ending -метный.
pub(crate) fn n1029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 990);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1804);
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2816);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1805);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1806);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1807);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 3710);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2818);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2819);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2820);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нетный.
pub(crate) fn n1030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 991);
	s.is_empty()
}

/// Words ending -петный.
pub(crate) fn n1031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 992);
	s.is_empty()
}

/// Words ending -ретный.
pub(crate) fn n1032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 993);
	s.is_empty()
}

/// Words ending -сетный.
pub(crate) fn n1033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 994);
	s.is_empty()
}

/// Words ending -тетный.
pub(crate) fn n1034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 995);
	s.is_empty()
}

/// Words ending -уетный.
pub(crate) fn n1035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 996);
	s.is_empty()
}

/// Words ending -фетный.
pub(crate) fn n1036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 997);
	s.is_empty()
}

/// Words ending -четный.
pub(crate) fn n1037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 998);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1809);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2822);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1810);
			match s {
				[] => true,
				['а', 'р', '-', s @ ..] => {
					super::push(out, 2824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шетный.
pub(crate) fn n1038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 999);
	s.is_empty()
}

/// Words ending -щетный.
pub(crate) fn n1039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1000);
	s.is_empty()
}

/// Words ending -итный.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	s.is_empty()
}

/// Words ending -йтный.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 445);
	s.is_empty()
}

/// Words ending -ктный.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1001);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 1811);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1812);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 2825);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 2826);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3712);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 1813);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 2828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1002);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нтный.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1003);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1814);
					match s {
						[] => true,
						['а', 'в', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 3714);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 3715);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4113);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 4114);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'г', s @ ..] => {
					super::push(out, 1815);
					match s {
						[] => true,
						['е', 'с', 'х', s @ ..] => {
							super::push(out, 2829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1816);
					match s {
						[] => true,
						['о', 'п', 'м', 'о', 'к', 'х', s @ ..] => {
							super::push(out, 2830);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3716);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1817);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 1818);
					match s {
						[] => true,
						['о', 'р', 'п', 'х', s @ ..] => {
							super::push(out, 2831);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3718);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3719);
									s.is_empty()
								}
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
			super::push(out, 1004);
			match s {
				[] => true,
				['м', 'е', 'р', 'о', 'т', s @ ..] => {
					super::push(out, 1819);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2832);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отный.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1005);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1820);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1006);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1822);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 1823);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2834);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 2835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртный.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	s.is_empty()
}

/// Words ending -стный.
pub(crate) fn n460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1007);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 1824);
					match s {
						[] => true,
						['о', 'л', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 3720);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 3721);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4115);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 4116);
											s.is_empty()
										}
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
					super::push(out, 1825);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 1826);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 2836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1008);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1827);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1828);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2837);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 2838);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3722);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3723);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1829);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3724);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3725);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 2839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1009);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1010);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1830);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 1831);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2840);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2841);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 2842);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2843);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1832);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 1833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1011);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2845);
							match s {
								[] => true,
								['х', 'е', s @ ..] => {
									super::push(out, 3726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 2846);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 2847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1012);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1013);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ттный.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	s.is_empty()
}

/// Words ending -утный.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 1014);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1015);
			match s {
				[] => true,
				['и', 'м', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 2848);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2849);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 2850);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3727);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3728);
									s.is_empty()
								}
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
			super::push(out, 1016);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -штный.
pub(crate) fn n463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['о', 'к', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1834);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1835);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытный.
pub(crate) fn n464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	s.is_empty()
}

/// Words ending -ьтный.
pub(crate) fn n465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	s.is_empty()
}

/// Words ending -ютный.
pub(crate) fn n466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 456);
	s.is_empty()
}

/// Words ending -ятный.
pub(crate) fn n467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	s.is_empty()
}

/// Words ending -очный.
pub(crate) fn n472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 462);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1036);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 1037);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1849);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1850);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2867);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2868);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 3733);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1851);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1852);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2869);
							match s {
								[] => true,
								['о', 'г', 'х', s @ ..] => {
									super::push(out, 3735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2870);
							match s {
								[] => true,
								['и', 'н', s @ ..] => {
									super::push(out, 3736);
									match s {
										[] => true,
										['а', 'л', 'п', '-', s @ ..] => {
											super::push(out, 4117);
											match s {
												[] => true,
												['о', 'н', s @ ..] => {
													match s {
														[] => true,
														['м', s @ ..] => super::t1::n4682(s, out),
														['р', s @ ..] => super::t1::n4683(s, out),
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
		['г', s @ ..] => {
			super::push(out, 1038);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1853);
					match s {
						[] => true,
						['с', 'о', s @ ..] => {
							super::push(out, 2871);
							match s {
								[] => true,
								['п', '-', s @ ..] => {
									super::push(out, 3737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1854);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2872);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 2873);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2874);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1855);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1856);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2876);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 3738);
									s.is_empty()
								}
								['о', 'р', s @ ..] => {
									super::push(out, 3739);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 2877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1857);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1040);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1859);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 1860);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 2878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1861);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2879);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3740);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 4118);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 4119);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 3741);
									match s {
										[] => true,
										['о', 'о', s @ ..] => {
											super::push(out, 4120);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 2880);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2881);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 2882);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2883);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 2884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1862);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1863);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1864);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1866);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1867);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 2885);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 2886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1869);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1872);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1873);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2887);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 2888);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2889);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3742);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 3743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 2890);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 2891);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1875);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1876);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1877);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1878);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'е', s @ ..] => {
			super::push(out, 1044);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1880);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1881);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1045);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1883);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1884);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2893);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 2894);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t1::n1087(s, out),
		['т', s @ ..] => super::t1::n1088(s, out),
		_ => false,
	}
}

/// Words ending -ехнедельный.
pub(crate) fn n4580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4300);
	s.is_empty()
}

/// Words ending -ухнедельный.
pub(crate) fn n4581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4301);
	s.is_empty()
}

/// Words ending -етостроительный.
pub(crate) fn n4582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4302);
	s.is_empty()
}

/// Words ending -стостроительный.
pub(crate) fn n4583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4303);
	s.is_empty()
}

/// Words ending -дильный.
pub(crate) fn n2021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1913);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 2930);
			match s {
				[] => true,
				['р', 'п', 'о', 'к', s @ ..] => {
					super::push(out, 3771);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4133);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 4134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ольный.
pub(crate) fn n1106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1064);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1914);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 1915);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 2931);
					match s {
						[] => true,
						['и', 'с', 'х', s @ ..] => {
							super::push(out, 3772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1916);
			match s {
				[] => true,
				['у', 'х', s @ ..] => {
					super::push(out, 2932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1917);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2933);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2934);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3773);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1918);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 1919);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2935);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2936);
					match s {
						[] => true,
						['к', 'х', s @ ..] => {
							super::push(out, 3775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 2937);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3776);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тно-технический.
pub(crate) fn n4653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4371);
	s.is_empty()
}

/// Words ending -атерский.
pub(crate) fn n2212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2099);
	s.is_empty()
}

/// Words ending -етерский.
pub(crate) fn n2213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2100);
	s.is_empty()
}

/// Words ending -итерский.
pub(crate) fn n2214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2101);
	s.is_empty()
}

/// Words ending -йтерский.
pub(crate) fn n2215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2102);
	s.is_empty()
}

/// Words ending -ктерский.
pub(crate) fn n2216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2103);
	s.is_empty()
}

/// Words ending -лтерский.
pub(crate) fn n2217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2104);
	s.is_empty()
}

/// Words ending -нтерский.
pub(crate) fn n2218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2105);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3055);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3056);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3057);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3058);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лотерский.
pub(crate) fn n2219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2106);
	s.is_empty()
}

/// Words ending -ртерский.
pub(crate) fn n2220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2107);
	s.is_empty()
}

/// Words ending -стерский.
pub(crate) fn n2221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2108);
	s.is_empty()
}

/// Words ending -утерский.
pub(crate) fn n2222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2109);
	s.is_empty()
}

/// Words ending -фтерский.
pub(crate) fn n2223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2110);
	s.is_empty()
}

/// Words ending -хтерский.
pub(crate) fn n2224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2111);
	s.is_empty()
}

/// Words ending -ьтерский.
pub(crate) fn n2225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2112);
	s.is_empty()
}

/// Words ending -оферский.
pub(crate) fn n1226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1172);
	s.is_empty()
}

/// Words ending -церский.
pub(crate) fn n1227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1173);
	s.is_empty()
}

/// Words ending -черский.
pub(crate) fn n1228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1174);
	s.is_empty()
}

/// Words ending -шерский.
pub(crate) fn n1229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1175);
	s.is_empty()
}

/// Words ending -ьерский.
pub(crate) fn n1230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1176);
	s.is_empty()
}

/// Words ending -ирский.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 525);
	s.is_empty()
}

/// Words ending -орский.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 526);
	s.is_empty()
}

/// Words ending -урский.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 527);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 1177);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 2113);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2114);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырский.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 528);
	s.is_empty()
}

/// Words ending -ярский.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 529);
	s.is_empty()
}

/// Words ending -ивавший.
pub(crate) fn n1262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1202);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2148);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 2149);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 2150);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 2151);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3074);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3075);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3830);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3076);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3077);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3078);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3832);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3079);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3080);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 3081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2152);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 2153);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3082);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3083);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3834);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 3835);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4193);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 4194);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4195);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4196);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4197);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4198);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 4199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 3836);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4200);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4201);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4202);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 4372);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 4373);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 3837);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 3838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3084);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3085);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3086);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2154);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3088);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3089);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3090);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3839);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3091);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3092);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3841);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3842);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4203);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 4204);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 3843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3093);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3844);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3845);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4205);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 4374);
											match s {
												[] => true,
												['о', 'п', '\u{1}', s @ ..] => {
													super::push(out, 4411);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 4375);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 4206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 3846);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3094);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3095);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 3096);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 3097);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 3098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2155);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2156);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3099);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2157);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2158);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 2159);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 2160);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 2161);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3100);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3101);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3849);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 3850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 2162);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ствовавший.
pub(crate) fn n3309(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3851);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 3852);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3853);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 3854);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3855);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3856);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4207);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4208);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4209);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4376);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 4377);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4378);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4379);
							s.is_empty()
						}
						['н', 'о', 'и', s @ ..] => {
							super::push(out, 4380);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4381);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 4382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3857);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 3858);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывавший.
pub(crate) fn n1266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1206);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2167);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3102);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3103);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3859);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3860);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4211);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 4212);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 4213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3104);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3105);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3106);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3107);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3108);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3109);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3110);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 3111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2168);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3112);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2169);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 2170);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2171);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3114);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3116);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2172);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3118);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3119);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4214);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2173);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3122);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3125);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3129);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 3131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2174);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2175);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3132);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3134);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 3136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2176);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3138);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3139);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3861);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3142);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3863);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3864);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3865);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4216);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 3866);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3867);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3868);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3144);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гавший.
pub(crate) fn n570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 551);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1207);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1208);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 1209);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1210);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1211);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1212);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2177);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3146);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3147);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3870);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3871);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 3872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1213);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1214);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -завший.
pub(crate) fn n573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 554);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1217);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1218);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1219);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1220);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1221);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3148);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 3149);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 3150);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3151);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3152);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 3153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1222);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кавший.
pub(crate) fn n574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 555);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1223);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1224);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1225);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2181);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2182);
					match s {
						[] => true,
						['щ', 'ы', s @ ..] => {
							super::push(out, 3154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1226);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2184);
					s.is_empty()
				}
				['е', 'ч', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1227);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1228);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 1229);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 3155);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1230);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1231);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -навший.
pub(crate) fn n577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 558);
	s.is_empty()
}

/// Words ending -павший.
pub(crate) fn n578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 559);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1232);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1233);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3157);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 3158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2187);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1234);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1235);
			s.is_empty()
		}
		['р', 'е', 'с', s @ ..] => {
			super::push(out, 1236);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1237);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1238);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1239);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -равший.
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1240);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2189);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -савший.
pub(crate) fn n580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	s.is_empty()
}

/// Words ending -тавший.
pub(crate) fn n581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 562);
	s.is_empty()
}

/// Words ending -хавший.
pub(crate) fn n582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 563);
	s.is_empty()
}

/// Words ending -цавший.
pub(crate) fn n583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 564);
	s.is_empty()
}

/// Words ending -чавший.
pub(crate) fn n584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 565);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1241);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1242);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3159);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3873);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 4218);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 4219);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 3874);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3875);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 3160);
							s.is_empty()
						}
						['ж', 'е', s @ ..] => {
							super::push(out, 3161);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 3876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'е', s @ ..] => {
							super::push(out, 3162);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 3877);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3163);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3878);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3879);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 4220);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 4221);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 4222);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 3880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 3164);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 3165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шавший.
pub(crate) fn n585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 566);
	s.is_empty()
}

/// Words ending -щавший.
pub(crate) fn n586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 567);
	s.is_empty()
}

/// Words ending -ехкилометровый.
pub(crate) fn n4670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4388);
	s.is_empty()
}

/// Words ending -ухкилометровый.
pub(crate) fn n4671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4389);
	s.is_empty()
}

/// Words ending -женный.
pub(crate) fn n949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 910);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1587);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2451);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 2452);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2453);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2454);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3370);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 3962);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 3371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2455);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2456);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3372);
							s.is_empty()
						}
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3963);
									s.is_empty()
								}
								['з', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4260);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 4261);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'н', s @ ..] => {
									super::push(out, 3964);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 3373);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3374);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 3966);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 3967);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 3968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3375);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3376);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3377);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2457);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1588);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1589);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2459);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2460);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2461);
					match s {
						[] => true,
						['с', 'у', s @ ..] => {
							super::push(out, 3379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2463);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3380);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3381);
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 3969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2464);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3382);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3383);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3384);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 2465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1590);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1591);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1592);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2468);
					match s {
						[] => true,
						['б', 'с', s @ ..] => {
							super::push(out, 3386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2469);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'о', 'д', 'о', 'р', s @ ..] => {
			super::push(out, 1593);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1594);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3387);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3388);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3389);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3390);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 3970);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 3971);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 3972);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 4262);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 4390);
													s.is_empty()
												}
												['к', s @ ..] => {
													super::push(out, 4391);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 3973);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 3974);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 3975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 3391);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2472);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3393);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3394);
							s.is_empty()
						}
						['р', 'о', 'т', 'и', s @ ..] => {
							super::push(out, 3395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2473);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2474);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 3976);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 3977);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 3978);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 3979);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 3980);
									match s {
										[] => true,
										['с', 'а', 'н', '\u{1}', s @ ..] => super::t1::n4540(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t1::n2621(s, out),
				['х', s @ ..] => super::t1::n2622(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t1::n1683(s, out),
		['у', s @ ..] => super::t1::n1684(s, out),
		['ю', s @ ..] => super::t1::n1685(s, out),
		['я', s @ ..] => super::t1::n1686(s, out),
		_ => false,
	}
}

/// Words ending -ленный.
pub(crate) fn n953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 914);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1603);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2487);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 3406);
							match s {
								[] => true,
								['с', 'о', s @ ..] => {
									super::push(out, 3993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2488);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2489);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2490);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3407);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3408);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3994);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 3409);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 3410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2491);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2492);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3411);
							s.is_empty()
						}
						['с', 'о', s @ ..] => {
							super::push(out, 3412);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2493);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2495);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2496);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3414);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2497);
					s.is_empty()
				}
				['л', 'о', s @ ..] => {
					match s {
						[] => true,
						['д', 'ы', s @ ..] => {
							super::push(out, 3416);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2498);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3418);
							s.is_empty()
						}
						['к', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 3996);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 3997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 3419);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3420);
							s.is_empty()
						}
						['р', 'о', s @ ..] => {
							super::push(out, 3421);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2499);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3423);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 3998);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 3999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4000);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 3424);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 3425);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3426);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 4002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', 'л', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3427);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 2500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1604);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2501);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2502);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2503);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3429);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 3430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2504);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3431);
							s.is_empty()
						}
						['ж', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4003);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 3432);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3433);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3434);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 3435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2505);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4005);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 3436);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4007);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 4008);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 4009);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 4010);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3437);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 4011);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 3438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2506);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2507);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t1::n1694(s, out),
		['д', s @ ..] => super::t1::n1695(s, out),
		['е', s @ ..] => super::t1::n1696(s, out),
		['з', s @ ..] => {
			super::push(out, 1607);
			s.is_empty()
		}
		['и', s @ ..] => super::t1::n1698(s, out),
		['к', s @ ..] => super::t1::n1699(s, out),
		['м', s @ ..] => super::t1::n1700(s, out),
		['о', s @ ..] => super::t1::n1701(s, out),
		['п', s @ ..] => super::t1::n1702(s, out),
		['р', 'е', 'в', 'с', 'о', s @ ..] => {
			super::push(out, 1613);
			s.is_empty()
		}
		['с', s @ ..] => super::t1::n1704(s, out),
		['т', s @ ..] => super::t1::n1705(s, out),
		['у', s @ ..] => super::t1::n1706(s, out),
		['х', 'ы', s @ ..] => super::t1::n1707(s, out),
		['ч', s @ ..] => super::t1::n1708(s, out),
		['ш', s @ ..] => super::t1::n1709(s, out),
		['ы', s @ ..] => super::t1::n1710(s, out),
		['я', s @ ..] => super::t1::n1711(s, out),
		_ => false,
	}
}

/// Words ending -менный.
pub(crate) fn n954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 915);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1622);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2558);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1623);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1624);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2560);
					match s {
						[] => true,
						['п', 'о', 'н', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 4035);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'в', s @ ..] => {
					super::push(out, 2561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1625);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1626);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 3487);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1627);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2562);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1628);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1629);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1630);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 2564);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3489);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1631);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1632);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 1633);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ненный.
pub(crate) fn n955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 916);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1634);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2566);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 2567);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2568);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2569);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2570);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3491);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3493);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3494);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3495);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 3496);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3497);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3498);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3499);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3500);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 3501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1635);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 1636);
			match s {
				[] => true,
				['а', 'р', 'с', s @ ..] => {
					super::push(out, 2572);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1637);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1638);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2574);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 3503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 2575);
					s.is_empty()
				}
				['л', 'ч', 'ы', s @ ..] => {
					super::push(out, 2576);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2577);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3504);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 4278);
											s.is_empty()
										}
										['в', s @ ..] => {
											super::push(out, 4279);
											s.is_empty()
										}
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
					super::push(out, 2578);
					s.is_empty()
				}
				['р', 'х', s @ ..] => {
					super::push(out, 2579);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2580);
					s.is_empty()
				}
				['ц', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3505);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3506);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4037);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4038);
									s.is_empty()
								}
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
			super::push(out, 1640);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3507);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1641);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2581);
					match s {
						[] => true,
						['р', 'д', 'е', s @ ..] => {
							super::push(out, 3509);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 4039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2582);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3510);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 3511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2583);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2584);
					s.is_empty()
				}
				['я', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 3512);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3513);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1642);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3515);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2585);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 2586);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2587);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3517);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3518);
							s.is_empty()
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 4040);
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									super::push(out, 4041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2588);
					s.is_empty()
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3519);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3520);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3521);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3522);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3523);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4042);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 4043);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 4044);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3524);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1643);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1644);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2589);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2590);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3526);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1645);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 2591);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2592);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2593);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2594);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2595);
					match s {
						[] => true,
						['о', s @ ..] => super::t1::n3763(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1646);
			s.is_empty()
		}
		['р', s @ ..] => super::t1::n1737(s, out),
		['с', s @ ..] => super::t1::n1738(s, out),
		['т', s @ ..] => super::t1::n1739(s, out),
		['ч', s @ ..] => super::t1::n1740(s, out),
		['ш', s @ ..] => super::t1::n1741(s, out),
		['я', s @ ..] => super::t1::n1742(s, out),
		_ => false,
	}
}

/// Words ending -оенный.
pub(crate) fn n956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 917);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2598);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 3535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1652);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1653);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1654);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 1655);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1656);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ренный.
pub(crate) fn n958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 919);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1657);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2600);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2601);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3537);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3538);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 4048);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 4049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 2602);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2603);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3540);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 3542);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4050);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 3543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2604);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2605);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1658);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2607);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3544);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2608);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1659);
			s.is_empty()
		}
		['г', 'а', 'б', s @ ..] => {
			super::push(out, 1660);
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2610);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3546);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2611);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4052);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 3548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1661);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2612);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 3549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2613);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2614);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2615);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2616);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2617);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1662);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1663);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 2619);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2620);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 3550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1664);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2622);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3551);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 4054);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 4280);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 4281);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 4055);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4056);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4057);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4058);
									match s {
										[] => true,
										['а', 'р', 'о', 'п', s @ ..] => {
											super::push(out, 4282);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 4059);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2623);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2624);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3552);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3554);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3555);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 3556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3557);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2625);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2626);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3559);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3561);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3562);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 3563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1665);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2628);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 2629);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2630);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 2631);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3564);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t1::n1759(s, out),
		['щ', s @ ..] => {
			super::push(out, 1667);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1668);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1669);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2640);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 2641);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сенный.
pub(crate) fn n959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 920);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2642);
					s.is_empty()
				}
				['п', 'о', s @ ..] => {
					super::push(out, 2643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2644);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2645);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 3568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2646);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 3569);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4062);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4063);
									s.is_empty()
								}
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
			super::push(out, 1670);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1671);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тенный.
pub(crate) fn n960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 921);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1672);
			match s {
				[] => true,
				['м', 'ы', s @ ..] => {
					super::push(out, 2647);
					s.is_empty()
				}
				['р', 'б', 'о', 'и', 'р', 'п', 'о', 'г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
					super::push(out, 2648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1673);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1674);
			match s {
				[] => true,
				['с', 'х', s @ ..] => {
					super::push(out, 2649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1675);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1676);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2650);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1677);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 1678);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2652);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 3570);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4064);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4065);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4066);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3571);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2653);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 2654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1679);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уенный.
pub(crate) fn n961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 922);
	s.is_empty()
}

/// Words ending -ценный.
pub(crate) fn n962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 923);
	s.is_empty()
}

/// Words ending -ченный.
pub(crate) fn n963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 924);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1680);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2655);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2656);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2657);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3573);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 3574);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3575);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2658);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2659);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 3577);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3578);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1681);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2660);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3580);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3581);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3582);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 3583);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3584);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3585);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2662);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2663);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3587);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3588);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3589);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4067);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4068);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 3590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3591);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3592);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3593);
							s.is_empty()
						}
						['з', 'и', 'е', s @ ..] => {
							super::push(out, 3594);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3595);
							s.is_empty()
						}
						['т', 'с', s @ ..] => {
							super::push(out, 3596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2664);
					match s {
						[] => true,
						['е', 'р', 'п', s @ ..] => {
							super::push(out, 3597);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3598);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3599);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3600);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', 'е', 'р', 'б', s @ ..] => {
					super::push(out, 2665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2666);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3602);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3603);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3604);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3605);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3606);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1683);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2668);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 2669);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3608);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1685);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2670);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2671);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2672);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2673);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 3610);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3611);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 4069);
									s.is_empty()
								}
								['в', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 4283);
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 4284);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 4285);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 4286);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 4287);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', 'о', 'п', s @ ..] => {
									super::push(out, 4070);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 4071);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 4072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 3612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2674);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2675);
					match s {
						[] => true,
						['о', 'т', s @ ..] => {
							super::push(out, 3613);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 4073);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 3614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t1::n2857(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1686);
			s.is_empty()
		}
		['р', s @ ..] => super::t1::n1783(s, out),
		['у', s @ ..] => super::t1::n1784(s, out),
		['ь', s @ ..] => {
			super::push(out, 1689);
			s.is_empty()
		}
		['ю', s @ ..] => super::t1::n1786(s, out),
		['я', s @ ..] => super::t1::n1787(s, out),
		_ => false,
	}
}

/// Words ending -шенный.
pub(crate) fn n964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 925);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1692);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2697);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 2698);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 2699);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2700);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 4081);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2701);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2702);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3634);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4083);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 4290);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 4396);
													s.is_empty()
												}
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
							super::push(out, 3635);
							match s {
								[] => true,
								['с', 'у', s @ ..] => {
									super::push(out, 4084);
									s.is_empty()
								}
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
			super::push(out, 1693);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1694);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3636);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2704);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3638);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2705);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1695);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2707);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2708);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1696);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1697);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2710);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2711);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2712);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2713);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3640);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3641);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3642);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3643);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4085);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 4086);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 4087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 3644);
							match s {
								[] => true,
								['о', 'в', s @ ..] => {
									super::push(out, 4088);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 3645);
							match s {
								[] => true,
								['о', 'п', 'ы', s @ ..] => {
									super::push(out, 4089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1698);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1699);
			match s {
				[] => true,
				['е', 'в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3646);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3647);
							match s {
								[] => true,
								['с', 'е', s @ ..] => {
									super::push(out, 4090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3649);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 3650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1700);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2715);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3651);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3652);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3653);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4091);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 4092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2716);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2717);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2718);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2719);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3655);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2720);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 3657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 1701);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1702);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 1703);
			match s {
				[] => true,
				['н', 'е', 'м', 'у', 'е', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 2722);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенный.
pub(crate) fn n965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 926);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1704);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2723);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2724);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2725);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2726);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3658);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 4093);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 4094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 3659);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 3660);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3661);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2728);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1705);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1706);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2730);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2731);
					match s {
						[] => true,
						['а', 'з', 'х', s @ ..] => {
							super::push(out, 3663);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4095);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2732);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1707);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3664);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3665);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4097);
									s.is_empty()
								}
								['о', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4291);
											s.is_empty()
										}
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 4292);
											s.is_empty()
										}
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
					super::push(out, 2734);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2735);
					match s {
						[] => true,
						['а', 'з', 'о', s @ ..] => {
							super::push(out, 3666);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 4098);
									match s {
										[] => true,
										['е', 'м', 'о', 'п', '\u{1}', s @ ..] => {
											super::push(out, 4293);
											s.is_empty()
										}
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
			super::push(out, 1708);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1709);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1710);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2736);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 2737);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 3667);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3668);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'ы', s @ ..] => {
					super::push(out, 2738);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2739);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3670);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3671);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3672);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3673);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1711);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1712);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2740);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2741);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2742);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1713);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 1714);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 1715);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1716);
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3675);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рехоперационный.
pub(crate) fn n4679(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 4413);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 4414);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --перфорационный.
pub(crate) fn n4680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4397);
	s.is_empty()
}

/// Words ending -мно-планировочный.
pub(crate) fn n4682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4399);
	s.is_empty()
}

/// Words ending -рно-планировочный.
pub(crate) fn n4683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4400);
	s.is_empty()
}

/// Words ending -сочный.
pub(crate) fn n1087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1046);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1886);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 2896);
					s.is_empty()
				}
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3744);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 3745);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4121);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4122);
									s.is_empty()
								}
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
			super::push(out, 1887);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 2897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -точный.
pub(crate) fn n1088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1047);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1889);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2898);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3746);
							match s {
								[] => true,
								['р', 'е', 'п', 'о', s @ ..] => {
									super::push(out, 4123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 3747);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3748);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1890);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2900);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3749);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2901);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3751);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3752);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2902);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2903);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3754);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 2904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1891);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1892);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1893);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2905);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2906);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2908);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1894);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1895);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word настороженный.
pub(crate) fn n4540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4263);
	s.is_empty()
}

/// Words ending -тоженный.
pub(crate) fn n2621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2475);
	s.is_empty()
}

/// Words ending -хоженный.
pub(crate) fn n2622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2476);
	s.is_empty()
}

/// Words ending -рженный.
pub(crate) fn n1683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1595);
	match s {
		[] => true,
		['е', 'в', 'т', 'а', s @ ..] => {
			super::push(out, 2477);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уженный.
pub(crate) fn n1684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1596);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2478);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 2479);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2480);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 3396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2481);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3397);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3398);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 3981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3399);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3982);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 3983);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3400);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3985);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4264);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3987);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 3988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3989);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2482);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2483);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 2484);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юженный.
pub(crate) fn n1685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1597);
	s.is_empty()
}

/// Words ending -яженный.
pub(crate) fn n1686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1598);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3401);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3402);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3991);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3992);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4266);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3403);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3404);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гленный.
pub(crate) fn n1694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1605);
	match s {
		[] => true,
		['у', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3439);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3440);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дленный.
pub(crate) fn n1695(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2509);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еленный.
pub(crate) fn n1696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1606);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3442);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 4013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 3443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2511);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2512);
			match s {
				[] => true,
				['а', 'г', s @ ..] => {
					super::push(out, 3444);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3445);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 3446);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3447);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', 'я', 'т', '\u{1}', s @ ..] => {
			super::push(out, 2513);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2514);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2515);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2516);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3449);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3450);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 3451);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3452);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4014);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3453);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2517);
			s.is_empty()
		}
		['ц', 'а', s @ ..] => {
			super::push(out, 2518);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иленный.
pub(crate) fn n1698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1608);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 2519);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 2520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кленный.
pub(crate) fn n1699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1609);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2521);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мленный.
pub(crate) fn n1700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2523);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 4016);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2524);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 3455);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3456);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 3457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2525);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2526);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3458);
					match s {
						[] => true,
						['е', 'в', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 4268);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3459);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3460);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4018);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4019);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4392);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 4393);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 4270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3461);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2527);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2528);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2529);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4020);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 4394);
											s.is_empty()
										}
										['в', s @ ..] => {
											super::push(out, 4395);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 4021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2530);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2531);
			s.is_empty()
		}
		['я', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 3463);
					s.is_empty()
				}
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 4022);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оленный.
pub(crate) fn n1701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1611);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2532);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 2533);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2534);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 3464);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2535);
			s.is_empty()
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 3466);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2536);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 3468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2537);
			match s {
				[] => true,
				['о', 'д', s @ ..] => {
					super::push(out, 3469);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2538);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пленный.
pub(crate) fn n1702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1612);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2539);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2540);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3471);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 4024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4271);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3472);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3473);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 4025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 3474);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 3475);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 4026);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4027);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2541);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3476);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2542);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3478);
					s.is_empty()
				}
				['к', 'о', 'в', s @ ..] => {
					super::push(out, 3479);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 4273);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3480);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3481);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4029);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4030);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4275);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4276);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4277);
									s.is_empty()
								}
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
			super::push(out, 2543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сленный.
pub(crate) fn n1704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1614);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3482);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4031);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							super::push(out, 4032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 3483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2544);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тленный.
pub(crate) fn n1705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1615);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2545);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2546);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4033);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2547);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уленный.
pub(crate) fn n1706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1616);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2548);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2549);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2550);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 3485);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2551);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыхленный.
pub(crate) fn n1707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1617);
	s.is_empty()
}

/// Words ending -членный.
pub(crate) fn n1708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1618);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2552);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 2553);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шленный.
pub(crate) fn n1709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1619);
	s.is_empty()
}

/// Words ending -ыленный.
pub(crate) fn n1710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1620);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 2554);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 2555);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2556);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2557);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яленный.
pub(crate) fn n1711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1621);
	s.is_empty()
}

/// Words ending -ороненный.
pub(crate) fn n3763(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4046);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 4047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рненный.
pub(crate) fn n1737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1647);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3528);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3529);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сненный.
pub(crate) fn n1738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1648);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 3530);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3532);
					s.is_empty()
				}
				['ъ', 'б', s @ ..] => {
					super::push(out, 3533);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3534);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тненный.
pub(crate) fn n1739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1649);
	s.is_empty()
}

/// Words ending -чненный.
pub(crate) fn n1740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1650);
	s.is_empty()
}

/// Words ending -шненный.
pub(crate) fn n1741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1651);
	s.is_empty()
}

/// Words ending -яненный.
pub(crate) fn n1742(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 2596);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2597);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уренный.
pub(crate) fn n1759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1666);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2632);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 3566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2633);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 2634);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 2635);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 2636);
			s.is_empty()
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 3567);
					s.is_empty()
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4060);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2637);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2638);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 2639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -точенный.
pub(crate) fn n2857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2676);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3615);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4075);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4076);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4077);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4288);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 4289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4078);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рченный.
pub(crate) fn n1783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1687);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2677);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2678);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3617);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3619);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3620);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ученный.
pub(crate) fn n1784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1688);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2679);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 2680);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 2681);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2682);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 3621);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3622);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2683);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2684);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 2685);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2686);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3624);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 3625);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3626);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3627);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3628);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3629);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2687);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2688);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2689);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юченный.
pub(crate) fn n1786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1690);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2690);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4079);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2691);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яченный.
pub(crate) fn n1787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1691);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2693);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2694);
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 3631);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3632);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2695);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2696);
			s.is_empty()
		}
		_ => false,
	}
}

