use flexion::Path;

/// Words ending -трируешь.
pub(crate) fn n11010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10443);
	match s {
		[] => true,
		['е', 'м', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24140);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24141);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17622);
			s.is_empty()
		}
		['н', 'е', 'ц', s @ ..] => {
			match s {
				[] => true,
				['н', 'о', 'к', 'с', s @ ..] => {
					super::push(out, 24142);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24143);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24144);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24146);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 24147);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['г', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31803);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32993);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 31804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'о', 'м', 'е', 'д', 'о', s @ ..] => {
					super::push(out, 24148);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 24149);
					match s {
						[] => true,
						['л', 'л', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31805);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31806);
									s.is_empty()
								}
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
			super::push(out, 17623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урируешь.
pub(crate) fn n11011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10444);
	match s {
		[] => true,
		['г', 'и', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24150);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 24151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'н', s @ ..] => {
			super::push(out, 17624);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фрируешь.
pub(crate) fn n11012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10445);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 17626);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17627);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 24153);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юрируешь.
pub(crate) fn n11014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10447);
	s.is_empty()
}

/// Words ending -орефлексируешь.
pub(crate) fn n35424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33424);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 33571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -офинансируешь.
pub(crate) fn n33730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31817);
	match s {
		[] => true,
		['д', 'е', s @ ..] => {
			super::push(out, 32995);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 32996);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 32997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енсируешь.
pub(crate) fn n18635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17637);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 24164);
			s.is_empty()
		}
		['п', 'м', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28976);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28977);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онсируешь.
pub(crate) fn n18636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17638);
	match s {
		[] => true,
		['н', 'а', 'о', s @ ..] => {
			super::push(out, 24165);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'с', 'о', s @ ..] => {
			super::push(out, 24166);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28979);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -псируешь.
pub(crate) fn n11020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10453);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17639);
			match s {
				[] => true,
				['л', 'л', 'о', 'к', 'с', s @ ..] => {
					super::push(out, 24167);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсируешь.
pub(crate) fn n11021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10454);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17640);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17641);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 17642);
			match s {
				[] => true,
				['к', 'с', s @ ..] => {
					super::push(out, 24168);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссируешь.
pub(crate) fn n11022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10455);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17643);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24169);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24170);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24171);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17644);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24172);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28983);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31818);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 28984);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31819);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31820);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31821);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31822);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31823);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31824);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32998);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 28985);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 24173);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17645);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 24174);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28987);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									super::push(out, 31826);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 24175);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28988);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 28989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17646);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17647);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -усируешь.
pub(crate) fn n11023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10456);
	match s {
		[] => true,
		['к', 'о', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24177);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24178);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьсируешь.
pub(crate) fn n11024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10457);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 17648);
			match s {
				[] => true,
				['а', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28990);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28991);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28993);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юсируешь.
pub(crate) fn n11025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10458);
	match s {
		[] => true,
		['д', 'о', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24179);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24180);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -имитируешь.
pub(crate) fn n25648(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29008);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 29009);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 29010);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ымитируешь.
pub(crate) fn n25649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24207);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 29011);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ритируешь.
pub(crate) fn n18676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17673);
	s.is_empty()
}

/// Words ending -фитируешь.
pub(crate) fn n18677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17674);
	s.is_empty()
}

/// Words ending -цитируешь.
pub(crate) fn n18678(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24208);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24209);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24210);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24211);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29014);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йтируешь.
pub(crate) fn n11030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10463);
	s.is_empty()
}

/// Words ending -ктируешь.
pub(crate) fn n11031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10464);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24212);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 24213);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31832);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31833);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31834);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32999);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 24214);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 24215);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24216);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24217);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29015);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 29016);
							match s {
								[] => true,
								['о', 'к', 'с', s @ ..] => {
									super::push(out, 31836);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 24218);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24219);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29017);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 29018);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 29019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24220);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31837);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31838);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31839);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24221);
					match s {
						[] => true,
						['с', 'н', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33000);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33001);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33002);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 33003);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33004);
											s.is_empty()
										}
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
					super::push(out, 24222);
					match s {
						[] => true,
						['р', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31841);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 31842);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31843);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24223);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 24224);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 24225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17675);
			s.is_empty()
		}
		['у', 'р', 'т', 'с', 'н', 'и', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24226);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24227);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтируешь.
pub(crate) fn n11032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10465);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17676);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 17677);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24228);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 24229);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 24230);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 24231);
					match s {
						[] => true,
						['р', 'о', 'с', s @ ..] => {
							super::push(out, 29020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24232);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29021);
							match s {
								[] => true,
								['л', 'г', 'е', 'р', 'а', s @ ..] => {
									super::push(out, 31845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 29022);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31846);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33005);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ц', '\u{1}', s @ ..] => {
									super::push(out, 31847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 29023);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31848);
									match s {
										[] => true,
										['е', 'п', 'с', 'к', 'э', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33426);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33427);
													s.is_empty()
												}
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
							super::push(out, 29024);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33006);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33007);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 29025);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 29026);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 31850);
									s.is_empty()
								}
								['к', 'о', 'д', 'а', s @ ..] => {
									super::push(out, 31851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24233);
					s.is_empty()
				}
				['ц', 'к', 'а', 'е', s @ ..] => {
					super::push(out, 24234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17678);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24235);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 24236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29027);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 29028);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31852);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33008);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 33009);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 29029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24237);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29030);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 29031);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29032);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24238);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 29033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отируешь.
pub(crate) fn n11033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10466);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17679);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 17680);
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['й', 'о', 'б', 'а', s @ ..] => {
					super::push(out, 24239);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24240);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17681);
			match s {
				[] => true,
				['и', 'п', 'т', s @ ..] => {
					super::push(out, 24241);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'а', 'б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29037);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 29038);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 29039);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31857);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 29040);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 24242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17682);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'а', 'к', 'с', s @ ..] => {
			super::push(out, 17683);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -птируешь.
pub(crate) fn n11034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10467);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17684);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ртируешь.
pub(crate) fn n11035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10468);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17685);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24244);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 29041);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31859);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33010);
											s.is_empty()
										}
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
					super::push(out, 24245);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17686);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24246);
					match s {
						[] => true,
						['н', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31860);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31861);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33011);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31862);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31863);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 24247);
					match s {
						[] => true,
						['е', 'д', 'с', s @ ..] => {
							super::push(out, 29043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 24248);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17687);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24249);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 24250);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24251);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29045);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29046);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 29047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стируешь.
pub(crate) fn n11036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10469);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17688);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24252);
					match s {
						[] => true,
						['л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31865);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31866);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33012);
											s.is_empty()
										}
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
					super::push(out, 24253);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 29048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17689);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24254);
					match s {
						[] => true,
						['н', 'и', 'о', 'р', s @ ..] => {
							super::push(out, 29049);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24255);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29050);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29051);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 29052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17690);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24256);
					match s {
						[] => true,
						['с', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31869);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 24257);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17691);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 24258);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 24259);
					s.is_empty()
				}
				['п', 'м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29054);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 29055);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 29056);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31873);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['г', 'е', 'д', 'о', s @ ..] => {
					super::push(out, 24260);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24261);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 29058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 17692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ттируешь.
pub(crate) fn n11037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10470);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 17693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утируешь.
pub(crate) fn n11038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10471);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17694);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 17695);
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24263);
					s.is_empty()
				}
				['с', 'и', 'д', 'о', s @ ..] => {
					super::push(out, 24264);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 17696);
			match s {
				[] => true,
				['м', 'о', 'к', 'е', s @ ..] => {
					super::push(out, 24265);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 17697);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24266);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24267);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтируешь.
pub(crate) fn n11039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10472);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29062);
							s.is_empty()
						}
						['ф', 'с', 'а', 'а', s @ ..] => {
							super::push(out, 29063);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24268);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['п', 'а', 'т', 'а', 'к', 'с', s @ ..] => {
							super::push(out, 29064);
							s.is_empty()
						}
						['с', 'н', 'о', 'к', 'о', s @ ..] => {
							super::push(out, 29065);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31875);
									s.is_empty()
								}
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

/// Words ending -ютируешь.
pub(crate) fn n11040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10473);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17698);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24269);
					match s {
						[] => true,
						['д', 'о', s @ ..] => {
							super::push(out, 29066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17699);
			s.is_empty()
		}
		['ш', 'а', 'р', 'а', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24270);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иваешься.
pub(crate) fn n14286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13513);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19538);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 19539);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
							super::push(out, 29289);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31927);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 29290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 24913);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 29291);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'а', 'р', 'о', s @ ..] => {
					super::push(out, 24914);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24915);
					s.is_empty()
				}
				['с', 'о', s @ ..] => {
					super::push(out, 24916);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', 'о', s @ ..] => {
					super::push(out, 24917);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'о', s @ ..] => {
					super::push(out, 24918);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'в', 'о', s @ ..] => {
					super::push(out, 24919);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19540);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 24920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 19541);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 19542);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 19543);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24922);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29297);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 29298);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 29299);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 24923);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 24924);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 24925);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29302);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29303);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31929);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 33037);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 33428);
													s.is_empty()
												}
												_ => false,
											}
										}
										['в', s @ ..] => {
											super::push(out, 33038);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 33429);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 33039);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33430);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 33431);
													match s {
														[] => true,
														['н', s @ ..] => super::t11::n35578(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 33432);
													match s {
														[] => true,
														['с', s @ ..] => super::t11::n35579(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 33433);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 33434);
													match s {
														[] => true,
														['в', s @ ..] => super::t11::n35580(s, out),
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
									super::push(out, 31930);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 29304);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29305);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 31932);
									match s {
										[] => true,
										['д', s @ ..] => {
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
				['з', s @ ..] => {
					super::push(out, 24926);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29306);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 29307);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 29308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24927);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29309);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 29310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24928);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29311);
							s.is_empty()
						}
						['р', 'п', s @ ..] => {
							super::push(out, 29312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24929);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24930);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24931);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 24932);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29314);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31935);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 33041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 29315);
							s.is_empty()
						}
						['р', s @ ..] => super::t11::n31091(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t11::n26410(s, out),
				['ю', s @ ..] => super::t11::n26411(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t11::n20641(s, out),
		['л', s @ ..] => super::t11::n20642(s, out),
		['н', s @ ..] => super::t11::n20643(s, out),
		['п', s @ ..] => super::t11::n20644(s, out),
		['р', s @ ..] => super::t11::n20645(s, out),
		['х', s @ ..] => super::t11::n20646(s, out),
		['ч', s @ ..] => super::t11::n20647(s, out),
		['ш', s @ ..] => super::t11::n20648(s, out),
		['щ', s @ ..] => super::t11::n20649(s, out),
		_ => false,
	}
}

/// Words ending -ываешься.
pub(crate) fn n14289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13516);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19557);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24983);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29398);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31986);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31987);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33060);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31988);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33061);
											s.is_empty()
										}
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
					super::push(out, 24984);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29399);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 29400);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29401);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29402);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 31989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 24985);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 24986);
					match s {
						[] => true,
						['а', 'д', 'б', s @ ..] => {
							super::push(out, 29404);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24987);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24988);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24989);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29406);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19558);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24990);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24991);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29408);
							match s {
								[] => true,
								['а', 'ж', s @ ..] => {
									super::push(out, 31991);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33062);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33437);
													s.is_empty()
												}
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
							super::push(out, 29409);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 31992);
									match s {
										[] => true,
										['в', 'о', s @ ..] => {
											super::push(out, 33063);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33438);
													s.is_empty()
												}
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
			super::push(out, 19559);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24992);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 24993);
					match s {
						[] => true,
						['в', 'а', s @ ..] => {
							super::push(out, 29410);
							match s {
								[] => true,
								['н', 'о', s @ ..] => {
									super::push(out, 31993);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33064);
											s.is_empty()
										}
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
					super::push(out, 24994);
					match s {
						[] => true,
						['к', 'с', s @ ..] => {
							super::push(out, 29411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 24995);
					match s {
						[] => true,
						['л', 'г', 'е', s @ ..] => {
							super::push(out, 29412);
							match s {
								[] => true,
								['р', 'е', 'п', 'а', s @ ..] => {
									super::push(out, 31994);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33065);
											s.is_empty()
										}
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
			super::push(out, 19560);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29413);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 29414);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 29415);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 29416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 24996);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 24997);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 24998);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29418);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24999);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 25000);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29419);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 29420);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29421);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31996);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25001);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25002);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25003);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 25004);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29423);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 25005);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29425);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 29426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19561);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25006);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => super::t11::n33919(s, out),
								['и', s @ ..] => super::t11::n33920(s, out),
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 29427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t11::n26487(s, out),
				['п', s @ ..] => super::t11::n26488(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t11::n20659(s, out),
		['н', s @ ..] => super::t11::n20660(s, out),
		['п', s @ ..] => super::t11::n20661(s, out),
		['р', s @ ..] => super::t11::n20662(s, out),
		['с', s @ ..] => super::t11::n20663(s, out),
		['т', s @ ..] => super::t11::n20664(s, out),
		_ => false,
	}
}

/// Words ending -ргаешься.
pub(crate) fn n14295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13522);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19583);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25059);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 25060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29480);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29481);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 29482);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 29483);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 29484);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29485);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29486);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32020);
									s.is_empty()
								}
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
			super::push(out, 19584);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25061);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25062);
					match s {
						[] => true,
						['с', 'о', 'в', 'о', s @ ..] => {
							super::push(out, 29488);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32021);
									s.is_empty()
								}
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

/// Words ending -угаешься.
pub(crate) fn n14296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13523);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 19585);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25063);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25064);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19586);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25066);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25067);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 25068);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25069);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25070);
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29489);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29490);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 25071);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 19587);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25072);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25073);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгаешься.
pub(crate) fn n14297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13524);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19588);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 25074);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19589);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19590);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25075);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25076);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25077);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25078);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -матюгаешься.
pub(crate) fn n20688(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 25079);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 25080);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29497);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ягаешься.
pub(crate) fn n14299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13526);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19591);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25081);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25082);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19592);
			s.is_empty()
		}
		['т', 'о', 'п', s @ ..] => {
			super::push(out, 19593);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -идаешься.
pub(crate) fn n14307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13534);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25107);
					s.is_empty()
				}
				['а', 'н', 'о', s @ ..] => {
					super::push(out, 25108);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25109);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25110);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 25111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 19621);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 19622);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19623);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25113);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25114);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25115);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25116);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32033);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32034);
									s.is_empty()
								}
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
			super::push(out, 19624);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25117);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одаешься.
pub(crate) fn n14309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13536);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19626);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25118);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19627);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 19628);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25120);
					match s {
						[] => true,
						['г', 'б', s @ ..] => {
							super::push(out, 29512);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32035);
									s.is_empty()
								}
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
			super::push(out, 19629);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19630);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25122);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29513);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 32036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25123);
					match s {
						[] => true,
						['с', 'а', 'р', 'о', s @ ..] => {
							super::push(out, 29514);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32037);
									s.is_empty()
								}
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

/// Words ending -сдаешься.
pub(crate) fn n14310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13537);
	match s {
		[] => true,
		['а', 'н', s @ ..] => {
			super::push(out, 19631);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тдаешься.
pub(crate) fn n14311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13538);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19632);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удаешься.
pub(crate) fn n14312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13539);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 19633);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыдаешься.
pub(crate) fn n14313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13540);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19634);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юдаешься.
pub(crate) fn n14314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13541);
	match s {
		[] => true,
		['л', 'б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25124);
					match s {
						[] => true,
						['н', 'о', s @ ..] => {
							super::push(out, 29515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25125);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядаешься.
pub(crate) fn n14315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13542);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19636);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -акуликаешься.
pub(crate) fn n33969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32047);
	s.is_empty()
}

/// Words ending -никаешься.
pub(crate) fn n20785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19680);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 25175);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рикаешься.
pub(crate) fn n20786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19681);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25176);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сикаешься.
pub(crate) fn n20787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19682);
	s.is_empty()
}

/// Words ending -хикаешься.
pub(crate) fn n20788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19683);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чикаешься.
pub(crate) fn n20789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19684);
	s.is_empty()
}

/// Words ending -шикаешься.
pub(crate) fn n20790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19685);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 25178);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лкаешься.
pub(crate) fn n14340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13565);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19686);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 25179);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19687);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 25180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29550);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29551);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29552);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29553);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32048);
									s.is_empty()
								}
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

/// Words ending -мкаешься.
pub(crate) fn n14341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13566);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19688);
			match s {
				[] => true,
				['л', 'а', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29554);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 29555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19689);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29556);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 29557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'е', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 25181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19690);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нкаешься.
pub(crate) fn n14342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13567);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 19691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окаешься.
pub(crate) fn n14343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13568);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19692);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 25182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19693);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25183);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25184);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19694);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19695);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25187);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 25188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29559);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29560);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19696);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25189);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 25190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 19697);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25191);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25192);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25193);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пкаешься.
pub(crate) fn n14344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13569);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19698);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19699);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25195);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркаешься.
pub(crate) fn n14345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13570);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19700);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25196);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 25197);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 29562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 25198);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 25199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29563);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29564);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 29565);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29566);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 25200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29567);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32050);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 32051);
									s.is_empty()
								}
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
			super::push(out, 19701);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25201);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29568);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 32052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 29569);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 25202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29570);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29571);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32054);
									s.is_empty()
								}
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
			super::push(out, 19702);
			match s {
				[] => true,
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29572);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 29573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19703);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25203);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 29574);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32055);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32056);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32057);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 32058);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32059);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33083);
											s.is_empty()
										}
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
					super::push(out, 25204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29575);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29576);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 29577);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19704);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29579);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 19705);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25205);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32060);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32061);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 32062);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33084);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 32063);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25206);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25207);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29581);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32064);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 32065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 29582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 25208);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скаешься.
pub(crate) fn n14346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13571);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19706);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 25209);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29583);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29584);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19707);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25210);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32066);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32067);
									s.is_empty()
								}
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
			super::push(out, 19708);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25211);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25212);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25213);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25214);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 29588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25215);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29590);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29591);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 29592);
							s.is_empty()
						}
						['о', 'д', s @ ..] => {
							super::push(out, 29593);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 29594);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29595);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32068);
									s.is_empty()
								}
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
			super::push(out, 19709);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25217);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32069);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32070);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32071);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32072);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33085);
											s.is_empty()
										}
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
			super::push(out, 19710);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 19711);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25219);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 25220);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25221);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32073);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32074);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32075);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32076);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33086);
											s.is_empty()
										}
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

/// Words ending -ткаешься.
pub(crate) fn n14347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13572);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19712);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19713);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 25222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29598);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 29599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укаешься.
pub(crate) fn n14348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13573);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19714);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 19715);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 19716);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25223);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25224);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19717);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25225);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29601);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29602);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', '\u{1}', s @ ..] => {
			super::push(out, 19718);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 19719);
			match s {
				[] => true,
				['у', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29604);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29605);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цкаешься.
pub(crate) fn n14349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13574);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чкаешься.
pub(crate) fn n14350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13575);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19721);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29607);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29608);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29609);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29610);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32077);
									s.is_empty()
								}
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
			super::push(out, 19722);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29611);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19723);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25226);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29613);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32078);
									s.is_empty()
								}
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

/// Words ending -шкаешься.
pub(crate) fn n14351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13576);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19724);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25227);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25228);
					match s {
						[] => true,
						['е', 'т', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32079);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 32080);
									s.is_empty()
								}
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
			super::push(out, 19725);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 19726);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 19727);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкаешься.
pub(crate) fn n14352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13577);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19728);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25229);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 25230);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25231);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25232);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19729);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25234);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25235);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25236);
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29617);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 29618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25237);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25238);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25239);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25240);
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					super::push(out, 25241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19730);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25242);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25243);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 25244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19731);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19732);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 19733);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25246);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25247);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29620);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 29621);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 32081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'о', s @ ..] => {
							super::push(out, 29622);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 25248);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 25249);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 25250);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25251);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32083);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33087);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 32084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25252);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', 'с', s @ ..] => {
							super::push(out, 29624);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32085);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33088);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 29625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25253);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29626);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 29627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25254);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 25255);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25256);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькаешься.
pub(crate) fn n14353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13578);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19734);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25257);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 29629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 25258);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19735);
			match s {
				[] => true,
				['а', 'х', s @ ..] => {
					super::push(out, 25259);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'с', s @ ..] => {
					super::push(out, 25260);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19736);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юкаешься.
pub(crate) fn n14354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13579);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19737);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 19738);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29633);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32086);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 32087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 29634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29635);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 29636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 25262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'к', 'а', s @ ..] => {
			super::push(out, 19739);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25264);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 25265);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19740);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 19741);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25266);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 29638);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 32088);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33089);
											s.is_empty()
										}
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

/// Words ending -якаешься.
pub(crate) fn n14355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13580);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19742);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 19743);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19744);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19745);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -черпаешься.
pub(crate) fn n26834(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29674);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 29675);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 29676);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32103);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 32104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 29677);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32105);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 32106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 29678);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 29679);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упаешься.
pub(crate) fn n14391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13616);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 19801);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25331);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29680);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 29681);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25332);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 25333);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29683);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32109);
									s.is_empty()
								}
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
			super::push(out, 19802);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25334);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25335);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32110);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32111);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32112);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32113);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33090);
											s.is_empty()
										}
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
			super::push(out, 19803);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 25336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19804);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 19805);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25338);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25339);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыпаешься.
pub(crate) fn n14392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13617);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19806);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 19807);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25340);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29685);
							s.is_empty()
						}
						['н', 'о', s @ ..] => {
							super::push(out, 29686);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 25341);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 25342);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29688);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 32115);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 29689);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32116);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25343);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29690);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32117);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25344);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29691);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32118);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33091);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 29692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25345);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25346);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25347);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 29693);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32119);
									s.is_empty()
								}
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

/// Words ending -люпаешься.
pub(crate) fn n20917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19808);
	match s {
		[] => true,
		['х', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29694);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 29695);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япаешься.
pub(crate) fn n14394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13619);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19810);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25349);
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29696);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19811);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25350);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 25351);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остираешься.
pub(crate) fn n31510(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 32129);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 32130);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 32131);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 32132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -устираешься.
pub(crate) fn n31513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29716);
	s.is_empty()
}

/// Words ending -ыстираешься.
pub(crate) fn n31514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29717);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытираешься.
pub(crate) fn n26881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25373);
	s.is_empty()
}

/// Words ending -ораешься.
pub(crate) fn n14398(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19822);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 25374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 19823);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ураешься.
pub(crate) fn n14399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13623);
	match s {
		[] => true,
		['ч', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25375);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25376);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25377);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -квитаешься.
pub(crate) fn n20961(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25421);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 25422);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 25423);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -скитаешься.
pub(crate) fn n20962(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25424);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 25425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -питаешься.
pub(crate) fn n20963(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25426);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 25427);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 25428);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -читаешься.
pub(crate) fn n20964(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25429);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25430);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 25431);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32141);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32142);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32143);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25432);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29738);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 29739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25433);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29740);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25434);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 25435);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтаешься.
pub(crate) fn n14409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13633);
	match s {
		[] => true,
		['о', 'б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25436);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25437);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25438);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25439);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отаешься.
pub(crate) fn n14410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13634);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19844);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25440);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32145);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 32146);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32147);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33096);
											s.is_empty()
										}
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
			super::push(out, 19845);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 19846);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 25441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29744);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19847);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25442);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25443);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25444);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25445);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19848);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25446);
					match s {
						[] => true,
						['л', 'х', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32148);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 32149);
									s.is_empty()
								}
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
			super::push(out, 19849);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25447);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32150);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32151);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 19850);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25448);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птаешься.
pub(crate) fn n14411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13635);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19851);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 25450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртаешься.
pub(crate) fn n14412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13636);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25451);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 25452);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стаешься.
pub(crate) fn n14413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13637);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19852);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25453);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 25454);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 29748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32152);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32153);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32154);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 25455);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 25456);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29749);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25457);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29750);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 29751);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32157);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 29752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 25458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 19853);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19854);
			match s {
				[] => true,
				['л', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29753);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29754);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29755);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32158);
									s.is_empty()
								}
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
			super::push(out, 19855);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25460);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32159);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32160);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33097);
											s.is_empty()
										}
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
					super::push(out, 25461);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25462);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29757);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 32161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 29758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19856);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25463);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 25464);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29759);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32162);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32163);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33098);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 32164);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33099);
											s.is_empty()
										}
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
			super::push(out, 19857);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25465);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32165);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 32166);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33100);
											s.is_empty()
										}
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
			super::push(out, 19858);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25466);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 19859);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25467);
					s.is_empty()
				}
				['л', 'х', 'т', s @ ..] => {
					super::push(out, 25468);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 19860);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25469);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 29762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утаешься.
pub(crate) fn n14414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13638);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 19861);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25470);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19862);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25472);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29763);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 29764);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32167);
									s.is_empty()
								}
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
			super::push(out, 19863);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25473);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32168);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 32169);
									s.is_empty()
								}
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
			super::push(out, 19864);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25474);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25475);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25476);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 29765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25477);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25478);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25479);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтаешься.
pub(crate) fn n14415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13639);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19865);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25480);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 25481);
					s.is_empty()
				}
				['р', 'а', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29767);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29768);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 29769);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чтаешься.
pub(crate) fn n14416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13640);
	match s {
		[] => true,
		['е', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25482);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25483);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25484);
					s.is_empty()
				}
				['о', 'д', s @ ..] => {
					super::push(out, 25485);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытаешься.
pub(crate) fn n14417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13641);
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25486);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25487);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25488);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25489);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятаешься.
pub(crate) fn n14418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13642);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19866);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дыхаешься.
pub(crate) fn n21017(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25523);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 25524);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 25525);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25526);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 25527);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 25528);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пыхаешься.
pub(crate) fn n21019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19892);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25529);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					super::push(out, 29786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25530);
			match s {
				[] => true,
				['р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 29787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25531);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25532);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25533);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тыхаешься.
pub(crate) fn n21021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19894);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29790);
					s.is_empty()
				}
				['у', 'б', 'а', s @ ..] => {
					super::push(out, 29791);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25535);
			match s {
				[] => true,
				['е', 'ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32172);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32173);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юхаешься.
pub(crate) fn n14429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13653);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19895);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29792);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25536);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25537);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яхаешься.
pub(crate) fn n14430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13654);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19897);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25538);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25539);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аябедничаешься.
pub(crate) fn n35442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33442);
	s.is_empty()
}

/// Words ending -ъябедничаешься.
pub(crate) fn n35443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33443);
	s.is_empty()
}

/// Words ending -летничаешься.
pub(crate) fn n34108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32185);
	s.is_empty()
}

/// Words ending -стничаешься.
pub(crate) fn n31616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29807);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 32186);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьничаешься.
pub(crate) fn n27078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25565);
	s.is_empty()
}

/// Words ending -лчаешься.
pub(crate) fn n14438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13662);
	s.is_empty()
}

/// Words ending -нчаешься.
pub(crate) fn n14439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13663);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19910);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					super::push(out, 25566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19911);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25567);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 25568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29808);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 29809);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 29810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25569);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очаешься.
pub(crate) fn n14440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13664);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19912);
			match s {
				[] => true,
				['о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29811);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29812);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 29813);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29814);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19913);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25570);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рчаешься.
pub(crate) fn n14441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13665);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19914);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тчаешься.
pub(crate) fn n14442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13666);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -учаешься.
pub(crate) fn n14443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13667);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19916);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 19917);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25571);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19918);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25573);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19919);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 19920);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25574);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25575);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19921);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 19922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьчаешься.
pub(crate) fn n14444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13668);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19923);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ючаешься.
pub(crate) fn n14445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13669);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19924);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ячаешься.
pub(crate) fn n14446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13670);
	s.is_empty()
}

/// Words ending -ордуешься.
pub(crate) fn n21477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20324);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29969);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 29970);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 29971);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тдуешься.
pub(crate) fn n14768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13967);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 20325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удуешься.
pub(crate) fn n14769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13968);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 20326);
			match s {
				[] => true,
				['о', 'б', 'о', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29972);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29973);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 32250);
									s.is_empty()
								}
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

/// Words ending -ыдуешься.
pub(crate) fn n14770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13969);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 20327);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -организуешься.
pub(crate) fn n34187(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', 'р', 'е', s @ ..] => {
			super::push(out, 33127);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 33128);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ианизуешься.
pub(crate) fn n31812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29987);
	s.is_empty()
}

/// Words ending -канизуешься.
pub(crate) fn n31813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29988);
	s.is_empty()
}

/// Words ending -манизуешься.
pub(crate) fn n31814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29989);
	s.is_empty()
}

/// Words ending -ранизуешься.
pub(crate) fn n31815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29990);
	s.is_empty()
}

/// Words ending -еколонизуешься.
pub(crate) fn n27399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25863);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 29991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ризуешься.
pub(crate) fn n21499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20345);
	match s {
		[] => true,
		['а', 'т', 'и', s @ ..] => {
			super::push(out, 25864);
			match s {
				[] => true,
				['л', 'и', 'м', 'е', s @ ..] => {
					super::push(out, 29992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29993);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 32264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29994);
					s.is_empty()
				}
				['т', 'к', 'а', 'р', 'а', 'х', 'о', s @ ..] => {
					super::push(out, 29995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32265);
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
				['л', s @ ..] => {
					super::push(out, 29996);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29998);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 29999);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 25865);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', 'л', 'у', 'п', s @ ..] => {
			super::push(out, 25866);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тизуешься.
pub(crate) fn n21500(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25867);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -физуешься.
pub(crate) fn n21501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20346);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 25869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мзуешься.
pub(crate) fn n14787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13986);
	s.is_empty()
}

/// Words ending -ьзуешься.
pub(crate) fn n14788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13987);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 20347);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30003);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30004);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32268);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 30005);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									super::push(out, 32269);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 30006);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32271);
									s.is_empty()
								}
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

/// Words ending -ируешься.
pub(crate) fn n14831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14026);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 20405);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25946);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32280);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 32281);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33132);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33449);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 33450);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33451);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 33452);
													s.is_empty()
												}
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
					super::push(out, 25947);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 25948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 20406);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25949);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 30038);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32282);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 30039);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 33133);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33453);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 33134);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33454);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 33135);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33455);
													s.is_empty()
												}
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
					super::push(out, 25950);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 32283);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32284);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 32285);
									match s {
										[] => true,
										['л', 'у', 'к', s @ ..] => {
											match s {
												[] => true,
												['е', 'р', 'е', s @ ..] => {
													super::push(out, 33456);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33457);
													s.is_empty()
												}
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
							super::push(out, 30040);
							match s {
								[] => true,
								['р', 'а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33136);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 33137);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 33138);
											s.is_empty()
										}
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
					super::push(out, 25951);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32286);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 32287);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32288);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33139);
											match s {
												[] => true,
												['о', 'к', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t11::n35587(s, out),
														['а', s @ ..] => super::t11::n35588(s, out),
														['с', s @ ..] => super::t11::n35589(s, out),
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
				['ь', s @ ..] => {
					super::push(out, 25952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 20407);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 20408);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25953);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 30041);
							match s {
								[] => true,
								['и', 'р', 'р', 'а', 'б', 'а', s @ ..] => {
									super::push(out, 32289);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33140);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 30042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25954);
					match s {
						[] => true,
						['л', s @ ..] => super::t11::n31873(s, out),
						['с', s @ ..] => {
							super::push(out, 30044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t11::n27504(s, out),
				['о', s @ ..] => super::t11::n27505(s, out),
				['р', s @ ..] => super::t11::n27506(s, out),
				['у', s @ ..] => super::t11::n27507(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t11::n21583(s, out),
		['з', s @ ..] => super::t11::n21584(s, out),
		['и', s @ ..] => super::t11::n21585(s, out),
		['к', s @ ..] => super::t11::n21586(s, out),
		['л', s @ ..] => super::t11::n21587(s, out),
		['м', s @ ..] => super::t11::n21588(s, out),
		['н', s @ ..] => super::t11::n21589(s, out),
		['о', 'в', s @ ..] => super::t11::n21590(s, out),
		['п', s @ ..] => super::t11::n21591(s, out),
		['р', s @ ..] => super::t11::n21592(s, out),
		['с', s @ ..] => super::t11::n21593(s, out),
		['т', s @ ..] => super::t11::n21594(s, out),
		['у', s @ ..] => super::t11::n21595(s, out),
		['ф', s @ ..] => super::t11::n21596(s, out),
		['ц', s @ ..] => super::t11::n21597(s, out),
		['ч', s @ ..] => super::t11::n21598(s, out),
		['ш', s @ ..] => super::t11::n21599(s, out),
		['ь', s @ ..] => super::t11::n21600(s, out),
		_ => false,
	}
}

/// Words ending -рестуешься.
pub(crate) fn n27685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26117);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30254);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 30255);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тестуешься.
pub(crate) fn n27686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26118);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 30256);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 30257);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -устуешься.
pub(crate) fn n21659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20470);
	s.is_empty()
}

/// Words ending -ыстуешься.
pub(crate) fn n21660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20471);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 26119);
			match s {
				[] => true,
				['о', 'к', 'о', s @ ..] => {
					super::push(out, 30258);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утуешься.
pub(crate) fn n14856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14050);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 20472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фтуешься.
pub(crate) fn n14857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14051);
	s.is_empty()
}

/// Words ending -хтуешься.
pub(crate) fn n14858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14052);
	match s {
		[] => true,
		['а', 'р', 'ф', 'а', s @ ..] => {
			super::push(out, 20473);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'ш', 'е', s @ ..] => {
					super::push(out, 26120);
					s.is_empty()
				}
				['р', 'т', s @ ..] => {
					super::push(out, 26121);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 26122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютуешься.
pub(crate) fn n14859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14053);
	s.is_empty()
}

/// Words ending -ятуешься.
pub(crate) fn n14860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14054);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 20474);
			match s {
				[] => true,
				['а', 'п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30259);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -постесняешься.
pub(crate) fn n34476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32519);
	s.is_empty()
}

/// Words ending -ясняешься.
pub(crate) fn n21868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20656);
	s.is_empty()
}

/// Words ending -тняешься.
pub(crate) fn n14999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14190);
	s.is_empty()
}

/// Words ending -чняешься.
pub(crate) fn n15000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14191);
	s.is_empty()
}

/// Words ending -захороводишься.
pub(crate) fn n35543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33539);
	s.is_empty()
}

/// Words ending -родишься.
pub(crate) fn n15120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14309);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20888);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 20889);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26507);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 26508);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 30441);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32556);
									s.is_empty()
								}
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
			super::push(out, 20890);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26509);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26510);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26511);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26512);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 26513);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 26514);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 26515);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20891);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26516);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 30444);
							match s {
								[] => true,
								['у', 'з', s @ ..] => {
									super::push(out, 32557);
									match s {
										[] => true,
										['е', 'б', 'о', '\u{1}', s @ ..] => {
											super::push(out, 33312);
											s.is_empty()
										}
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
					super::push(out, 26517);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 20892);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 20893);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30446);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 30447);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32558);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 32559);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 33313);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33540);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 32560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 30448);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 30449);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 30450);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 30451);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 30452);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32564);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 30453);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 26518);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 26519);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 20894);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 20895);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 20896);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оходишься.
pub(crate) fn n22130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20900);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 26525);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 26526);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сходишься.
pub(crate) fn n22131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20901);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26527);
			match s {
				[] => true,
				['р', 'о', s @ ..] => {
					super::push(out, 30457);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыходишься.
pub(crate) fn n22133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20903);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 26528);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -телишься.
pub(crate) fn n15257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14446);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21197);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 21198);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26760);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32602);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32603);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33318);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 33319);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33542);
													s.is_empty()
												}
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
		['о', s @ ..] => {
			super::push(out, 21199);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26761);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21200);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26763);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26764);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -целишься.
pub(crate) fn n15258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14447);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21201);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21202);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21203);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21204);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26767);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шелишься.
pub(crate) fn n15259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14448);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21205);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26768);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щелишься.
pub(crate) fn n15260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14449);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 21206);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26769);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бсеменишься.
pub(crate) fn n32541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30673);
	s.is_empty()
}

/// Words ending -осеменишься.
pub(crate) fn n32542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30674);
	s.is_empty()
}

/// Words ending -ссеменишься.
pub(crate) fn n32543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30675);
	s.is_empty()
}

/// Words ending -ысеменишься.
pub(crate) fn n32544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30676);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зменишься.
pub(crate) fn n22702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21449);
	match s {
		[] => true,
		['и', 'о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30677);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -именишься.
pub(crate) fn n22703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21450);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 26985);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сменишься.
pub(crate) fn n22704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21451);
	s.is_empty()
}

/// Words ending -тменишься.
pub(crate) fn n22705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21452);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пенишься.
pub(crate) fn n15386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14572);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21453);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21454);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26987);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 26988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26989);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32632);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32633);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 32634);
									match s {
										[] => true,
										['з', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33543);
													match s {
														[] => true,
														['р', s @ ..] => super::t11::n35610(s, out),
														_ => false,
													}
												}
												['в', s @ ..] => {
													super::push(out, 33544);
													s.is_empty()
												}
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
					super::push(out, 26990);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30680);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32635);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32636);
									s.is_empty()
								}
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
			super::push(out, 21455);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21456);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренишься.
pub(crate) fn n15387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14573);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 21457);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26992);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26993);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21458);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30681);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 30682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сенишься.
pub(crate) fn n15388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14574);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тенишься.
pub(crate) fn n15389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14575);
	s.is_empty()
}

/// Words ending -ценишься.
pub(crate) fn n15390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14576);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21461);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21462);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26995);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21463);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26996);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26997);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 30684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченишься.
pub(crate) fn n15391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14577);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щенишься.
pub(crate) fn n15392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14578);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21465);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21466);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ронишься.
pub(crate) fn n15425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14611);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21529);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27063);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 27064);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 30733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 21530);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21531);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30734);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27067);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27068);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27069);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30737);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32652);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32653);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 32654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 27070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30738);
							s.is_empty()
						}
						['а', 'з', 'е', s @ ..] => {
							super::push(out, 30739);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 30740);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 30741);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32655);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 30742);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 30743);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 30744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21532);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 21533);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27071);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21534);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 21535);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27072);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тонишься.
pub(crate) fn n15427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14613);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21537);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21538);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щеришься.
pub(crate) fn n15553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14739);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21769);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьеришься.
pub(crate) fn n15554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14740);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 21771);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27244);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -купоришься.
pub(crate) fn n28915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27277);
	s.is_empty()
}

/// Words ending -шпоришься.
pub(crate) fn n23070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21809);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27278);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -соришься.
pub(crate) fn n15575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14761);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21810);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21811);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21812);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27279);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27280);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27281);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21813);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27283);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -торишься.
pub(crate) fn n15576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14762);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21814);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21815);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27284);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 21816);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27286);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21817);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21818);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21819);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27289);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 27290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 21820);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 21821);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27291);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -форишься.
pub(crate) fn n15577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14763);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 21822);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27292);
					match s {
						[] => true,
						['ф', 'с', s @ ..] => {
							super::push(out, 30848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хоришься.
pub(crate) fn n15578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14764);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 21823);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21824);
			match s {
				[] => true,
				['р', 'о', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30849);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шоришься.
pub(crate) fn n15579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14765);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21825);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окоротишься.
pub(crate) fn n32853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30976);
	s.is_empty()
}

/// Words ending -укоротишься.
pub(crate) fn n32854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30977);
	s.is_empty()
}

/// Words ending -соротишься.
pub(crate) fn n29137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27479);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 30978);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33331);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хотишься.
pub(crate) fn n15678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14864);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 22012);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27480);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 22013);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27482);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -истишься.
pub(crate) fn n15684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14870);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22029);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31004);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31005);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31006);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32718);
									s.is_empty()
								}
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
			super::push(out, 22030);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27522);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27523);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27524);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27525);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окстишься.
pub(crate) fn n23307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22031);
	s.is_empty()
}

/// Words ending -лстишься.
pub(crate) fn n15686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14872);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 22032);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мстишься.
pub(crate) fn n15687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14873);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22033);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 22034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -остишься.
pub(crate) fn n15688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14874);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22035);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 27526);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 22036);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27527);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27528);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 27529);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27530);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22037);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27531);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27532);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31010);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32719);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33338);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 32720);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22038);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27534);
					match s {
						[] => true,
						['х', 'ы', s @ ..] => {
							super::push(out, 31011);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32721);
									s.is_empty()
								}
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
			super::push(out, 22039);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27535);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27536);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27537);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 22040);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27538);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22041);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27540);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 27541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рстишься.
pub(crate) fn n15689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14875);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22042);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31014);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 31015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устишься.
pub(crate) fn n15690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14876);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 22043);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27542);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27543);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 27544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27545);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27546);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27547);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27548);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 22045);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27549);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31019);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32722);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27550);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 27551);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27552);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22046);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31021);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 31022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыстишься.
pub(crate) fn n15691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14877);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 22047);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31023);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31024);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32723);
									s.is_empty()
								}
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

/// Words ending -ьстишься.
pub(crate) fn n15692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14878);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 22048);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27554);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27555);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ридрочишься.
pub(crate) fn n32948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31070);
	s.is_empty()
}

/// Words ending -удрочишься.
pub(crate) fn n29303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27640);
	s.is_empty()
}

/// Words ending -орочишься.
pub(crate) fn n23443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22161);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 27641);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31071);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 31072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 27642);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31073);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27643);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27644);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31076);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31077);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 31078);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прочишься.
pub(crate) fn n23444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22162);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27645);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 27646);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -срочишься.
pub(crate) fn n23445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22163);
	s.is_empty()
}

/// Words ending -трочишься.
pub(crate) fn n23446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22164);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 27647);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31079);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 31080);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31081);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 31082);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 31083);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урочишься.
pub(crate) fn n23447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22165);
	s.is_empty()
}

/// Words ending -сочишься.
pub(crate) fn n15760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14945);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22166);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22167);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 22168);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 22169);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22170);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 22171);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27649);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -точишься.
pub(crate) fn n15761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14946);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22172);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22173);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 22174);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 22175);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 22176);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27650);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 27651);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27652);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22177);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27653);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27654);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31086);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 31087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 27655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22178);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 22179);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 22180);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27656);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31088);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32743);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 32744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27657);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олюбопытничаешь.
pub(crate) fn n33333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31446);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 32832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятничаешь.
pub(crate) fn n17465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16553);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23402);
			match s {
				[] => true,
				['е', 'с', 'о', s @ ..] => {
					super::push(out, 28379);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23403);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ольничаешь.
pub(crate) fn n24787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23431);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 28411);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 28412);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31483);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 31484);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31485);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['м', 'а', 'с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33358);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 32851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'ш', 'о', s @ ..] => {
			super::push(out, 28413);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 28414);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31487);
					match s {
						[] => true,
						['р', 'к', 'о', s @ ..] => {
							super::push(out, 32852);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31488);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 31489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 28415);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 28416);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 28417);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -переизбыточествуешь.
pub(crate) fn n35550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33546);
	s.is_empty()
}

/// Words ending -шествуешь.
pub(crate) fn n18112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17166);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23656);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28492);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 23657);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23658);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28494);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31533);
							match s {
								[] => true,
								['о', 'п', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33366);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 33547);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 33367);
											s.is_empty()
										}
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
					super::push(out, 28495);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31534);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33368);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33369);
											s.is_empty()
										}
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
					super::push(out, 28496);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31535);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32871);
									s.is_empty()
								}
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
			super::push(out, 23659);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23660);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28498);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31536);
							match s {
								[] => true,
								['о', 'р', 'о', s @ ..] => {
									super::push(out, 32872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28499);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 31537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28500);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31538);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 31539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23661);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 23662);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28501);
					match s {
						[] => true,
						['м', 'у', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32873);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32874);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32875);
									s.is_empty()
								}
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
			super::push(out, 23663);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28502);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 32876);
									match s {
										[] => true,
										['а', 'л', 'б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33548);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 33549);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33550);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 32877);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 32878);
									match s {
										[] => true,
										['а', 'м', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33551);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33552);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 33553);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 32879);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 32880);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 33370);
											s.is_empty()
										}
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
					super::push(out, 28503);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28504);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иществуешь.
pub(crate) fn n25030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23664);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 28505);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31541);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оществуешь.
pub(crate) fn n25031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23665);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 28506);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31542);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уществуешь.
pub(crate) fn n25032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23666);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 28507);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 28508);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32881);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -исходатайствуешь.
pub(crate) fn n35558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33554);
	s.is_empty()
}

/// Words ending -лонствуешь.
pub(crate) fn n25078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23710);
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31579);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 31580);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ронствуешь.
pub(crate) fn n25079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23711);
	s.is_empty()
}

/// Words ending -шонствуешь.
pub(crate) fn n25080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23712);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 28556);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унствуешь.
pub(crate) fn n18134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17188);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 23713);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23714);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 23715);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31581);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янствуешь.
pub(crate) fn n18135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17189);
	match s {
		[] => true,
		['ь', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28558);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28559);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 28560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31585);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 31586);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28561);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нерствуешь.
pub(crate) fn n25107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23737);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28593);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 31603);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31604);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32905);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31605);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 32907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 28594);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 28595);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 28596);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31607);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32908);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 33376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33377);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33555);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 33378);
									s.is_empty()
								}
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

/// Words ending -перствуешь.
pub(crate) fn n25108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23738);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28597);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -серствуешь.
pub(crate) fn n25109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23739);
	s.is_empty()
}

/// Words ending -терствуешь.
pub(crate) fn n25110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23740);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31608);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 31609);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 28598);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 28599);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 28600);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 28601);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 28602);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 28603);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 31612);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ферствуешь.
pub(crate) fn n25111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23741);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28604);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 31613);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -черствуешь.
pub(crate) fn n25112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23742);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 28605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шерствуешь.
pub(crate) fn n25113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23743);
	s.is_empty()
}

/// Words ending -ьерствуешь.
pub(crate) fn n25114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23744);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 28606);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 28607);
			match s {
				[] => true,
				['о', 'к', 'а', 'р', 'б', 'о', s @ ..] => {
					super::push(out, 31615);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирствуешь.
pub(crate) fn n18142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17196);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 23745);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 23746);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 23747);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 23748);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 23749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орствуешь.
pub(crate) fn n18143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17197);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23750);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28608);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 28609);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 23751);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28610);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32911);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 32912);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 33379);
											s.is_empty()
										}
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
			super::push(out, 23752);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28611);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 23753);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28612);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28613);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 28614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23754);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 23755);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28616);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31619);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32913);
									s.is_empty()
								}
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
			super::push(out, 23756);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 23757);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28617);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31620);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 31621);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 31622);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32915);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32916);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33380);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33556);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 32917);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 33381);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 32918);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 32919);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33382);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', 'а', 'о', s @ ..] => {
					super::push(out, 28618);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31624);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 31625);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32920);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 32921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28619);
					match s {
						[] => true,
						['а', 'ф', s @ ..] => {
							super::push(out, 31626);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32922);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 31627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28620);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 28621);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урствуешь.
pub(crate) fn n18144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17198);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 23758);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 23759);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ырствуешь.
pub(crate) fn n18145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17199);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 23760);
			match s {
				[] => true,
				['с', 'а', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31630);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 32923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 31631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эрствуешь.
pub(crate) fn n18146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17200);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 23761);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ярствуешь.
pub(crate) fn n18147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17201);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28622);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28623);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 23762);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пятствуешь.
pub(crate) fn n25162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23788);
	match s {
		[] => true,
		['е', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31651);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31652);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 31653);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тельствуешь.
pub(crate) fn n30384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28655);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31661);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32931);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 32932);
					match s {
						[] => true,
						['е', 'с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33557);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33605);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 33558);
									match s {
										[] => true,
										['е', 'р', 'п', 'о', s @ ..] => {
											super::push(out, 33606);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33616);
													s.is_empty()
												}
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
				['ж', 'я', s @ ..] => {
					super::push(out, 32933);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32934);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'е', 'ж', 'о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 33389);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'б', 'о', 'д', 'е', s @ ..] => {
							super::push(out, 33390);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32935);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'а', s @ ..] => {
					super::push(out, 32936);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 33392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31662);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['и', 'в', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33561);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33562);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33607);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 33563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33393);
							match s {
								[] => true,
								['г', 'а', 'л', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33608);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33609);
											s.is_empty()
										}
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
			super::push(out, 31663);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32937);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33395);
							s.is_empty()
						}
						['о', 'р', 'к', 'о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 33396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33397);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33398);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 33610);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33611);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 33564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', 'о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33399);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33565);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32938);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33402);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33403);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 33404);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32939);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32940);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 32941);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33406);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33407);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 33408);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33567);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 33568);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33612);
											s.is_empty()
										}
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
			super::push(out, 31664);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фильствуешь.
pub(crate) fn n30386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28657);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32942);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 32943);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 33409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32944);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ольствуешь.
pub(crate) fn n25169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23794);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 28658);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33410);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 33411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 32945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 28659);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 28660);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ульствуешь.
pub(crate) fn n25170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23795);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 28661);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 28662);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -селекционируешь.
pub(crate) fn n35575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33569);
	s.is_empty()
}

/// Words ending -зафункционируешь.
pub(crate) fn n35576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33570);
	s.is_empty()
}

/// Words ending -отонируешь.
pub(crate) fn n25530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24112);
	s.is_empty()
}

/// Words ending -фонируешь.
pub(crate) fn n18573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17583);
	match s {
		[] => true,
		['е', 'л', 'е', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28916);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28917);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шонируешь.
pub(crate) fn n18574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17584);
	s.is_empty()
}

/// Words ending -напереживаешься.
pub(crate) fn n35578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33572);
	s.is_empty()
}

/// Words ending -сопереживаешься.
pub(crate) fn n35579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33573);
	s.is_empty()
}

/// Words ending -выпереживаешься.
pub(crate) fn n35580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33574);
	s.is_empty()
}

/// Words ending -руживаешься.
pub(crate) fn n31091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29316);
	s.is_empty()
}

/// Words ending -ыживаешься.
pub(crate) fn n26410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24933);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 29317);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юживаешься.
pub(crate) fn n26411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24934);
	s.is_empty()
}

/// Words ending -киваешься.
pub(crate) fn n20641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19544);
	match s {
		[] => true,
		['с', 'и', s @ ..] => {
			super::push(out, 24935);
			match s {
				[] => true,
				['т', 'о', s @ ..] => {
					super::push(out, 29318);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33042);
									s.is_empty()
								}
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

/// Words ending -ливаешься.
pub(crate) fn n20642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19545);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24936);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29319);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 29320);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 29321);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29322);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 29323);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 29324);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 29325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 24937);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29326);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 29327);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29328);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31937);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33043);
									s.is_empty()
								}
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
			super::push(out, 24938);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 24939);
			match s {
				[] => true,
				['о', 'п', 'о', s @ ..] => {
					super::push(out, 29330);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31938);
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
				['б', s @ ..] => {
					super::push(out, 29331);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29332);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31939);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 31940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 24940);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29334);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29335);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 24941);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24942);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29336);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 29337);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31942);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 31943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 29338);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 29339);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29340);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31944);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 31945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 24943);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24944);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 24945);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29342);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 29343);
					match s {
						[] => true,
						['м', 'е', s @ ..] => {
							super::push(out, 31946);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33045);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33435);
											s.is_empty()
										}
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
					super::push(out, 29344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 24946);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'г', s @ ..] => {
			super::push(out, 24947);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 24948);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 24949);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29346);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29347);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 29348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 24950);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ниваешься.
pub(crate) fn n20643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19546);
	s.is_empty()
}

/// Words ending -пиваешься.
pub(crate) fn n20644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19547);
	s.is_empty()
}

/// Words ending -риваешься.
pub(crate) fn n20645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19548);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24951);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29349);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 29350);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 29351);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29352);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 29353);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 29354);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 24952);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29355);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31948);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 31949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 29356);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 29357);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29358);
					match s {
						[] => true,
						['д', 'а', s @ ..] => {
							super::push(out, 31952);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29359);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 29360);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29361);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 29362);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 24953);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29363);
					match s {
						[] => true,
						['е', 'п', 'ы', 'в', 'о', s @ ..] => {
							super::push(out, 31956);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24954);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 24955);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24956);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29365);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 29366);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31957);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 33048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 31958);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 31959);
							match s {
								[] => true,
								['к', 'у', s @ ..] => {
									super::push(out, 33049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 31960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31961);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 31962);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 33050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29367);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 29368);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 24957);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 24958);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29369);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31964);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 31965);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33051);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 33436);
											s.is_empty()
										}
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
			super::push(out, 24959);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хиваешься.
pub(crate) fn n20646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19549);
	s.is_empty()
}

/// Words ending -чиваешься.
pub(crate) fn n20647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19550);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24960);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24961);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24962);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29370);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 29371);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 31966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 29372);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31967);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29373);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31968);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31969);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 33053);
									s.is_empty()
								}
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
			super::push(out, 24963);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29374);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 31970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29375);
					match s {
						[] => true,
						['б', 'а', 'к', 'ы', 'в', 'о', s @ ..] => {
							super::push(out, 31971);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 29376);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31972);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33055);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 33056);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29377);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шиваешься.
pub(crate) fn n20648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19551);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24964);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29378);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 24965);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 24966);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 24967);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29381);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24968);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29383);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29384);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31973);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33057);
									s.is_empty()
								}
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
			super::push(out, 24969);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29385);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24970);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29386);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29387);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 29388);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31976);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31977);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 31978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24971);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 24972);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24973);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29390);
					match s {
						[] => true,
						['с', 'а', s @ ..] => {
							super::push(out, 31979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 24974);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щиваешься.
pub(crate) fn n20649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19552);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24975);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29393);
					match s {
						[] => true,
						['с', 'и', s @ ..] => {
							super::push(out, 31980);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33058);
									s.is_empty()
								}
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
			super::push(out, 24976);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29394);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31981);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 33059);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24977);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29396);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вкалываешься.
pub(crate) fn n33919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31997);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33066);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33439);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -икалываешься.
pub(crate) fn n33920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31998);
	match s {
		[] => true,
		['р', 'п', 'о', s @ ..] => {
			super::push(out, 33067);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33440);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елываешься.
pub(crate) fn n26487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25007);
	s.is_empty()
}

/// Words ending -плываешься.
pub(crate) fn n26488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25008);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 29428);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мываешься.
pub(crate) fn n20659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19562);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25009);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29429);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 29430);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 29431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 25010);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 25011);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 25012);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', s @ ..] => {
			super::push(out, 25013);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 25014);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29435);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 29436);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31999);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33068);
									s.is_empty()
								}
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
			super::push(out, 25015);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25016);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25017);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 25018);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25019);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29440);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32000);
							match s {
								[] => true,
								['р', 'п', 'а', s @ ..] => {
									super::push(out, 33069);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33441);
											s.is_empty()
										}
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
			super::push(out, 25020);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нываешься.
pub(crate) fn n20660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19563);
	s.is_empty()
}

/// Words ending -пываешься.
pub(crate) fn n20661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19564);
	match s {
		[] => true,
		['а', 'к', s @ ..] => {
			super::push(out, 25021);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29442);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 32001);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33070);
									s.is_empty()
								}
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
			super::push(out, 25022);
			match s {
				[] => true,
				['е', 'ч', 'с', s @ ..] => {
					super::push(out, 29443);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32002);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33071);
									s.is_empty()
								}
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

/// Words ending -рываешься.
pub(crate) fn n20662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19565);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25023);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29444);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29445);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 29446);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 29447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 25024);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 25025);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 25026);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29449);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 29450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 25027);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 25028);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 25029);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29452);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 29453);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32003);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33072);
									s.is_empty()
								}
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
			super::push(out, 25030);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 25031);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29455);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 32004);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33073);
									s.is_empty()
								}
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
			super::push(out, 25032);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 25033);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29456);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25034);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29457);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32006);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33074);
									s.is_empty()
								}
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
			super::push(out, 25035);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 25036);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29458);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сываешься.
pub(crate) fn n20663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19566);
	match s {
		[] => true,
		['а', 'р', 'б', 'ы', s @ ..] => {
			super::push(out, 25037);
			match s {
				[] => true,
				['в', 'о', s @ ..] => {
					super::push(out, 29459);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25038);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 25039);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29460);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 32008);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29461);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тываешься.
pub(crate) fn n20664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19567);
	s.is_empty()
}

/// The word консервируешься.
pub(crate) fn n35587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33581);
	s.is_empty()
}

/// Words ending -аконсервируешься.
pub(crate) fn n35588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33582);
	s.is_empty()
}

/// Words ending -сконсервируешься.
pub(crate) fn n35589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33583);
	s.is_empty()
}

/// Words ending -лидируешься.
pub(crate) fn n31873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30043);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32290);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ндируешься.
pub(crate) fn n27504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25955);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30045);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32291);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33141);
							match s {
								[] => true,
								['п', 'о', 'р', 'п', 'с', s @ ..] => {
									super::push(out, 33458);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33584);
											s.is_empty()
										}
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
					super::push(out, 32292);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32293);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33143);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 33459);
									s.is_empty()
								}
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
			super::push(out, 30046);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 30047);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 30048);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32294);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 32295);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 32296);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одируешься.
pub(crate) fn n27505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25956);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 30049);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдируешься.
pub(crate) fn n27506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25957);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30051);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30052);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удируешься.
pub(crate) fn n27507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25958);
	s.is_empty()
}

/// Words ending -жируешься.
pub(crate) fn n21583(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25959);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25960);
			match s {
				[] => true,
				['г', 'н', 'а', 'а', s @ ..] => {
					super::push(out, 30053);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30054);
					match s {
						[] => true,
						['и', 'т', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 33144);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 33145);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30055);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32301);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32302);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'е', 'х', s @ ..] => {
			super::push(out, 25961);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30056);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25962);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 25963);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зируешься.
pub(crate) fn n21584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20409);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25964);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30057);
					s.is_empty()
				}
				['г', 'з', s @ ..] => {
					super::push(out, 30058);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 30059);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30060);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30061);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32306);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33147);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'т', 'н', s @ ..] => {
			super::push(out, 25965);
			match s {
				[] => true,
				['и', 'с', 'е', s @ ..] => {
					super::push(out, 30062);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25966);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32309);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32310);
							match s {
								[] => true,
								['р', 'п', 'м', 'и', 'а', s @ ..] => {
									super::push(out, 33149);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33460);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', 'о', 'л', 'о', 'е', s @ ..] => {
					super::push(out, 30063);
					match s {
						[] => true,
						['д', 'и', 'а', s @ ..] => {
							super::push(out, 32311);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 30064);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 33151);
									match s {
										[] => true,
										['ц', 'р', s @ ..] => {
											super::push(out, 33461);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33585);
													match s {
														[] => true,
														['м', s @ ..] => super::t11::n35619(s, out),
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
									super::push(out, 33152);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33462);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33586);
													match s {
														[] => true,
														['р', s @ ..] => super::t11::n35620(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 33463);
											match s {
												[] => true,
												['м', 'о', s @ ..] => {
													super::push(out, 33587);
													match s {
														[] => true,
														['н', s @ ..] => super::t11::n35621(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 33464);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 33153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'б', s @ ..] => {
							super::push(out, 32312);
							match s {
								[] => true,
								['а', 'т', 'с', 'а', s @ ..] => {
									super::push(out, 33154);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33465);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 32313);
							match s {
								[] => true,
								['а', 'т', 'с', 'и', 'р', 'к', 'ы', s @ ..] => {
									super::push(out, 33155);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33466);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30065);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 30066);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32315);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32316);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33156);
									match s {
										[] => true,
										['х', 'н', 'и', 'с', 'с', s @ ..] => {
											super::push(out, 33467);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33588);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 33157);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33468);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 33589);
													s.is_empty()
												}
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
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32317);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32318);
							s.is_empty()
						}
						['о', 'р', s @ ..] => {
							super::push(out, 32319);
							match s {
								[] => true,
								['р', 'е', 'т', 'а', s @ ..] => {
									super::push(out, 33158);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33469);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t11::n31899(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t11::n27516(s, out),
		['о', 'д', s @ ..] => {
			super::push(out, 25968);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30070);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иируешься.
pub(crate) fn n21585(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 25969);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25970);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 25971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кируешься.
pub(crate) fn n21586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20410);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25972);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32327);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 32328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25973);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 25974);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 25975);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25976);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30074);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33171);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33172);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33173);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 33174);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 33175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 30075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25977);
			match s {
				[] => true,
				['а', 'м', 'е', s @ ..] => {
					super::push(out, 30076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25978);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32329);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32330);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32331);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 25979);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30077);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лируешься.
pub(crate) fn n21587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20411);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25980);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30078);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 30079);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 30080);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 32332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30081);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 32333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30082);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30083);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30084);
					match s {
						[] => true,
						['в', 'а', s @ ..] => {
							super::push(out, 32335);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33178);
									s.is_empty()
								}
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
			super::push(out, 25981);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30085);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32336);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33179);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 33180);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30086);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30087);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							super::push(out, 32338);
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
				['б', s @ ..] => {
					super::push(out, 30088);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 30089);
					match s {
						[] => true,
						['и', 'н', 'с', s @ ..] => {
							super::push(out, 32339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 30090);
					match s {
						[] => true,
						['о', 'м', 'с', s @ ..] => {
							super::push(out, 32340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30091);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32341);
							match s {
								[] => true,
								['н', 'т', s @ ..] => {
									super::push(out, 33182);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33478);
											s.is_empty()
										}
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
					super::push(out, 30092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25982);
			match s {
				[] => true,
				['п', 'м', 'о', 'к', 'с', s @ ..] => {
					super::push(out, 30093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'н', 'е', 'в', 'о', s @ ..] => {
					super::push(out, 30094);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 30095);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32344);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33479);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 33480);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 33481);
											s.is_empty()
										}
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
			super::push(out, 25983);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 25984);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30096);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30097);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32347);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32348);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32349);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 32350);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30099);
					match s {
						[] => true,
						['т', 'н', 'о', 'к', 'о', s @ ..] => {
							super::push(out, 32351);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33184);
									s.is_empty()
								}
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
			super::push(out, 25985);
			match s {
				[] => true,
				['н', 'а', 'р', 'т', 'т', s @ ..] => {
					super::push(out, 30100);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25986);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30101);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32353);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33186);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33187);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 33188);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 33189);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 33190);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 33191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', 'ь', 'л', 'а', 'к', 'с', s @ ..] => {
					super::push(out, 30102);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32354);
							s.is_empty()
						}
						['р', 'о', 'ф', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33192);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33482);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 33193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', 'к', 'к', 'а', 'с', s @ ..] => {
							super::push(out, 32355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30103);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 30104);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 25987);
			match s {
				[] => true,
				['у', 'м', 'а', 'к', 'а', s @ ..] => {
					super::push(out, 30106);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мируешься.
pub(crate) fn n21588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20412);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25988);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30107);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 30108);
					match s {
						[] => true,
						['ь', 'л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33194);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'к', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['д', 'о', s @ ..] => {
									super::push(out, 33196);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33483);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'з', s @ ..] => {
									super::push(out, 33197);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33484);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 25989);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 25990);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32358);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 32359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25991);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30109);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30110);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32361);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33198);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 32362);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 33200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30111);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 25992);
			match s {
				[] => true,
				['а', 'р', 'г', 'о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32363);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32365);
							s.is_empty()
						}
						['с', 'о', s @ ..] => {
							super::push(out, 32366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25993);
			match s {
				[] => true,
				['р', 'х', 'т', s @ ..] => {
					super::push(out, 30113);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25994);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30114);
					s.is_empty()
				}
				['о', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32368);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32369);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33485);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33591);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 33486);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 33592);
													s.is_empty()
												}
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
							super::push(out, 32370);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 33201);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33487);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33593);
													s.is_empty()
												}
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
							super::push(out, 32371);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33202);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 33203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32372);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33204);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 33205);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33488);
											s.is_empty()
										}
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
			super::push(out, 25995);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 25996);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 25997);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 25998);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30115);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30116);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нируешься.
pub(crate) fn n21589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20413);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25999);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30117);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33206);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33207);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33208);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 33209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 26000);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 26001);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30119);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 30120);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30121);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32374);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 32375);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33211);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33212);
									s.is_empty()
								}
								['е', 'д', s @ ..] => {
									super::push(out, 33213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 30122);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26002);
			match s {
				[] => true,
				['б', 'м', 'о', 'к', 'с', s @ ..] => {
					super::push(out, 30123);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 30124);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32379);
							s.is_empty()
						}
						['р', 'о', 'о', 'к', 'с', s @ ..] => {
							super::push(out, 32380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32381);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 32382);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30125);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 30126);
					s.is_empty()
				}
				['ц', 'к', 'а', 'в', 'о', s @ ..] => {
					super::push(out, 30127);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26003);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30128);
					match s {
						[] => true,
						['ц', 'и', 'з', s @ ..] => {
							super::push(out, 32384);
							match s {
								[] => true,
								['о', 'п', 'е', s @ ..] => {
									super::push(out, 33216);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33489);
											s.is_empty()
										}
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
					super::push(out, 30129);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 30130);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30131);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32385);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33217);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33490);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 33218);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33491);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 32386);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30132);
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33219);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33492);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 33220);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'и', 'о', s @ ..] => {
							super::push(out, 32388);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33221);
									s.is_empty()
								}
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
			super::push(out, 26004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -воируешься.
pub(crate) fn n21590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20414);
	match s {
		[] => true,
		['н', 'о', 'к', 'т', s @ ..] => {
			super::push(out, 26005);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30133);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пируешься.
pub(crate) fn n21591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20415);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26006);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30134);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 30135);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30136);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33222);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30137);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 32389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 26007);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 26008);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 26009);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 30138);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26010);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30139);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32391);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 32392);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', 'е', 'с', 'к', 'т', s @ ..] => {
									super::push(out, 33225);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33493);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 33226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30140);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30141);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 26011);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 30142);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33227);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 33228);
									s.is_empty()
								}
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
			super::push(out, 26012);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 26013);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26014);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 26015);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рируешься.
pub(crate) fn n21592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20416);
	match s {
		[] => true,
		['а', 'п', 'е', 'р', 'п', 'т', s @ ..] => {
			super::push(out, 26016);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 26017);
			s.is_empty()
		}
		['в', 'а', 'т', 'с', 'е', 'р', 'т', s @ ..] => {
			super::push(out, 26018);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 26019);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 26020);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 26021);
			match s {
				[] => true,
				['н', 'е', 'г', 'с', s @ ..] => {
					super::push(out, 30146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30147);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32396);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33229);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'п', 'с', 'н', 'о', 'к', 'а', s @ ..] => {
			super::push(out, 26022);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26023);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30150);
					match s {
						[] => true,
						['е', 'д', 'а', s @ ..] => {
							super::push(out, 32397);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'г', 'и', 'о', s @ ..] => {
					super::push(out, 30151);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 26024);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30152);
					match s {
						[] => true,
						['м', 'о', 'н', 'о', 'р', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33232);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33233);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 30153);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 30154);
					match s {
						[] => true,
						['е', 'ц', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33235);
									s.is_empty()
								}
								['н', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33494);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 33495);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 33236);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 33237);
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
							super::push(out, 32399);
							s.is_empty()
						}
						['и', 'г', s @ ..] => {
							super::push(out, 32400);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33496);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 33497);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 33594);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 33498);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'о', 'м', 'е', 'д', 'о', s @ ..] => {
							super::push(out, 32401);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33238);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', 'л', 'л', 'ы', s @ ..] => {
							super::push(out, 32402);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 33239);
									s.is_empty()
								}
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
			super::push(out, 26025);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 26026);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30155);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 32403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30156);
					match s {
						[] => true,
						['г', 'а', s @ ..] => {
							super::push(out, 32404);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33240);
									s.is_empty()
								}
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

/// Words ending -сируешься.
pub(crate) fn n21593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20417);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26027);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30157);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26028);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30159);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 26029);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30161);
					s.is_empty()
				}
				['е', 'д', 'н', 'и', 'о', s @ ..] => {
					super::push(out, 30162);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'ф', 'а', s @ ..] => {
					super::push(out, 30163);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30164);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 33241);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33499);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 33242);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33500);
											s.is_empty()
										}
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
			super::push(out, 26030);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30165);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32408);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33243);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33244);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33501);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', '\u{1}', s @ ..] => {
									super::push(out, 33245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'и', 'ф', 'о', 'р', s @ ..] => {
							super::push(out, 32409);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33246);
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
						['д', 'н', 'о', 'к', 'с', s @ ..] => {
							super::push(out, 32410);
							s.is_empty()
						}
						['п', 'м', 'о', 'к', 'с', s @ ..] => {
							super::push(out, 32411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30166);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32412);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 32413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 26031);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 26032);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30167);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32414);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 33247);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33502);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 32415);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 32416);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 32417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30168);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32418);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33248);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33503);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 33504);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 33505);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 33506);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33595);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 33249);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33507);
											s.is_empty()
										}
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
					super::push(out, 30169);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 32419);
							match s {
								[] => true,
								['е', 'р', 'т', s @ ..] => {
									super::push(out, 33250);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33508);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 32420);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30170);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 30171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26033);
			match s {
				[] => true,
				['к', 'о', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32421);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тируешься.
pub(crate) fn n21594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20418);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26034);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30172);
					match s {
						[] => true,
						['е', 'д', 'о', s @ ..] => {
							super::push(out, 32423);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 30173);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 30174);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32424);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 32425);
							match s {
								[] => true,
								['о', 'ф', '\u{1}', s @ ..] => {
									super::push(out, 33253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 32426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30175);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 30176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', 'м', 'с', s @ ..] => {
							super::push(out, 32427);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 32428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'о', 'р', 'п', 'м', 'о', 'к', 'с', s @ ..] => {
					super::push(out, 30177);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 30178);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33254);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 33255);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 33256);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30179);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32429);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 32430);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 32431);
							match s {
								[] => true,
								['р', 'е', 'т', 'н', 'и', 'е', s @ ..] => {
									super::push(out, 33257);
									match s {
										[] => true,
										['р', 'е', s @ ..] => {
											super::push(out, 33509);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33596);
													s.is_empty()
												}
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
							super::push(out, 32432);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33258);
									s.is_empty()
								}
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
			super::push(out, 26035);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30180);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 30181);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 30182);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32433);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32434);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32435);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33259);
									s.is_empty()
								}
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
			super::push(out, 26036);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32436);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33510);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 33511);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 33512);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33513);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 33514);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 32437);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 32438);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32439);
							s.is_empty()
						}
						['т', 'н', 'о', 'к', 'с', s @ ..] => {
							super::push(out, 32440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30183);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 32441);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 32442);
							s.is_empty()
						}
						['о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33260);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33515);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 33261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', 'с', 'н', s @ ..] => {
							match s {
								[] => true,
								['и', 'о', s @ ..] => {
									super::push(out, 33262);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33516);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'к', 'а', s @ ..] => {
									super::push(out, 33263);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33517);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32443);
							match s {
								[] => true,
								['р', 'о', 'к', '\u{1}', s @ ..] => {
									super::push(out, 33264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32444);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 32445);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 32446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30184);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 32447);
							match s {
								[] => true,
								['п', 'а', s @ ..] => {
									super::push(out, 33265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'р', 'т', 'с', 'н', 'и', 'о', s @ ..] => {
					super::push(out, 30185);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t12::n27588(s, out),
		['о', s @ ..] => {
			super::push(out, 26038);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30188);
					match s {
						[] => true,
						['л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33275);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33523);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => super::t12::n35268(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t12::n27590(s, out),
		['р', s @ ..] => super::t12::n27591(s, out),
		['с', s @ ..] => super::t12::n27592(s, out),
		['у', s @ ..] => super::t12::n27593(s, out),
		['ь', s @ ..] => super::t12::n27594(s, out),
		['ю', s @ ..] => super::t12::n27595(s, out),
		_ => false,
	}
}

/// Words ending -уируешься.
pub(crate) fn n21595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20419);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 26045);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 26046);
			match s {
				[] => true,
				['т', 'с', 'н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', s @ ..] => {
							super::push(out, 32470);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33288);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30199);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33289);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33528);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 33290);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 33291);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33529);
											s.is_empty()
										}
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
					super::push(out, 30200);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фируешься.
pub(crate) fn n21596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20420);
	match s {
		[] => true,
		['а', 'р', 'г', 'о', s @ ..] => {
			match s {
				[] => true,
				['н', 'е', 'т', 'с', 'а', s @ ..] => {
					super::push(out, 30201);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30202);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32473);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 32474);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32475);
							match s {
								[] => true,
								['ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33530);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 33531);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33532);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 33533);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цируешься.
pub(crate) fn n21597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20421);
	match s {
		[] => true,
		['е', 'о', s @ ..] => {
			super::push(out, 26047);
			match s {
				[] => true,
				['р', 'п', 'с', s @ ..] => {
					super::push(out, 30203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'ф', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32477);
							match s {
								[] => true,
								['а', 'в', 'к', s @ ..] => {
									match s {
										[] => true,
										['е', 'р', s @ ..] => {
											super::push(out, 33534);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33601);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 33535);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 33602);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'а', 'т', s @ ..] => {
							super::push(out, 32478);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33292);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33536);
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
								['с', 'а', s @ ..] => {
									super::push(out, 33293);
									match s {
										[] => true,
										['л', 'к', 'с', s @ ..] => {
											super::push(out, 33537);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33603);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', 'р', 'е', s @ ..] => {
									super::push(out, 33294);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 33538);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'и', 'з', s @ ..] => {
					super::push(out, 30204);
					match s {
						[] => true,
						['е', 'д', 'о', s @ ..] => {
							super::push(out, 32480);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33295);
									s.is_empty()
								}
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
			super::push(out, 26048);
			match s {
				[] => true,
				['в', 'о', 'р', 'п', 'с', s @ ..] => {
					super::push(out, 30205);
					s.is_empty()
				}
				['л', 'с', 'и', 'д', 'е', s @ ..] => {
					super::push(out, 30206);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26049);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чируешься.
pub(crate) fn n21598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20422);
	match s {
		[] => true,
		['ь', 'л', 'у', 'м', 'а', s @ ..] => {
			super::push(out, 26050);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шируешься.
pub(crate) fn n21599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20423);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26051);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 26052);
			match s {
				[] => true,
				['л', 'к', 'а', s @ ..] => {
					super::push(out, 30208);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 30209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 26053);
			match s {
				[] => true,
				['и', 'м', 'с', s @ ..] => {
					super::push(out, 30210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 26054);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32483);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 32484);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 33296);
									s.is_empty()
								}
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
			super::push(out, 26055);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30211);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33297);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33298);
									s.is_empty()
								}
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
			super::push(out, 26056);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30212);
					s.is_empty()
				}
				['т', 'е', s @ ..] => {
					super::push(out, 30213);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 32485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 26057);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьируешься.
pub(crate) fn n21600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20424);
	s.is_empty()
}

/// Words ending -разъерепенишься.
pub(crate) fn n35610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33604);
	s.is_empty()
}

/// Words ending -мерциализируешься.
pub(crate) fn n35619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33613);
	s.is_empty()
}

/// Words ending -роанализируешься.
pub(crate) fn n35620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33614);
	s.is_empty()
}

/// Words ending -номинализируешься.
pub(crate) fn n35621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33615);
	s.is_empty()
}

/// Words ending -тизируешься.
pub(crate) fn n31899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30067);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32320);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33159);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33160);
					match s {
						[] => true,
						['к', 'о', 'р', 'ю', 'б', 'а', s @ ..] => {
							super::push(out, 33470);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33590);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['м', 'р', 'е', 'г', 'а', s @ ..] => {
					super::push(out, 33161);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'о', 'е', 'т', 'а', s @ ..] => {
					super::push(out, 33162);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'л', 'о', 'п', 'а', s @ ..] => {
			super::push(out, 32321);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 32322);
			match s {
				[] => true,
				['к', 'р', 'а', 'н', 'а', s @ ..] => {
					super::push(out, 33164);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'п', 'и', 'г', 'а', s @ ..] => {
					super::push(out, 33165);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'о', 'м', 'а', 'с', s @ ..] => {
			super::push(out, 32323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 32324);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33167);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нзируешься.
pub(crate) fn n27516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25967);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 30068);
			match s {
				[] => true,
				['ц', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'т', s @ ..] => {
							super::push(out, 33168);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'л', 'т', s @ ..] => {
							super::push(out, 33169);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33477);
									s.is_empty()
								}
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
			super::push(out, 30069);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32325);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 33170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

