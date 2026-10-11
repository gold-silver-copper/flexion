use flexion::Path;

/// Words ending -ожный.
pub(crate) fn n1220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1189);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7677);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3279);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 3280);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3281);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3282);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7681);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7682);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7683);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14163);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 21016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 14164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7684);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14165);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 14166);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21017);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26488);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 14167);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 26489);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 29917);
													s.is_empty()
												}
												_ => false,
											}
										}
										['я', s @ ..] => {
											super::push(out, 26490);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14168);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 21018);
									s.is_empty()
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26491);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 29918);
													match s {
														[] => true,
														['р', s @ ..] => super::t7::n33383(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 26492);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 14169);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 14170);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 26493);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 26494);
											s.is_empty()
										}
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
					super::push(out, 7685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3283);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3284);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3285);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3286);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14171);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 21019);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 21020);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 21021);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21022);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 26495);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 21023);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 26496);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 29919);
													match s {
														[] => true,
														['е', s @ ..] => super::t7::n33384(s, out),
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
							super::push(out, 14172);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 21024);
									s.is_empty()
								}
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
			super::push(out, 3287);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7686);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 7687);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржный.
pub(crate) fn n1221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1190);
	match s {
		[] => true,
		['е', 'д', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7688);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14175);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 14176);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 21025);
									s.is_empty()
								}
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
			super::push(out, 3288);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7689);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужный.
pub(crate) fn n1222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1191);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3289);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7690);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3290);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7692);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3291);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7694);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7695);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3292);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7696);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7697);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3293);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3294);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7698);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7699);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14181);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 14182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7700);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 14183);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 21026);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26497);
											s.is_empty()
										}
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
					super::push(out, 7701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3295);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3296);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 3297);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7702);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7703);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 14184);
							match s {
								[] => true,
								['б', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 21027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7704);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7705);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 7706);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыжный.
pub(crate) fn n1223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1192);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3298);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7707);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21028);
									s.is_empty()
								}
								['р', 'о', 'г', '\u{1}', s @ ..] => {
									super::push(out, 21029);
									s.is_empty()
								}
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
			super::push(out, 3299);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7708);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 7709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эжный.
pub(crate) fn n1224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1193);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3300);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7710);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -южный.
pub(crate) fn n1225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1194);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3301);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3302);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7711);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3303);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3304);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 7714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 3305);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7715);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжный.
pub(crate) fn n1226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1195);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3306);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3307);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7718);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 7719);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 7720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3308);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7721);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14186);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 14187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3309);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7722);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7723);
					match s {
						[] => true,
						['р', 'о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 14189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3310);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3311);
			s.is_empty()
		}
		['ь', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7724);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14190);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 14191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7725);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -разный.
pub(crate) fn n3441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3316);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7730);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21033);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 21034);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21035);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 26500);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 29920);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 31823);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', 'е', s @ ..] => {
									super::push(out, 26501);
									match s {
										[] => true,
										['щ', s @ ..] => {
											super::push(out, 29921);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31824);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									super::push(out, 26502);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 29922);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', 'о', s @ ..] => {
									super::push(out, 26503);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 29923);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 31825);
													s.is_empty()
												}
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
							super::push(out, 21036);
							s.is_empty()
						}
						['о', s @ ..] => super::t7::n22108(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7731);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14198);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7732);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14199);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14200);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лезный.
pub(crate) fn n3447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3322);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7738);
			match s {
				[] => true,
				['ж', 'з', s @ ..] => {
					super::push(out, 14206);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7739);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7740);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14207);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7741);
			match s {
				[] => true,
				['к', 'р', 'е', 'б', 'у', 'т', 'и', s @ ..] => {
					super::push(out, 14208);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'а', s @ ..] => {
					super::push(out, 14209);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -незный.
pub(crate) fn n3448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3323);
	s.is_empty()
}

/// Words ending -пезный.
pub(crate) fn n3449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3324);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7742);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14210);
					match s {
						[] => true,
						['т', 'а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 21047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -резный.
pub(crate) fn n3450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3325);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7744);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14211);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 14212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7745);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14213);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 14214);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14215);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14216);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14217);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тезный.
pub(crate) fn n3451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3326);
	s.is_empty()
}

/// Words ending -ьезный.
pub(crate) fn n3452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3327);
	s.is_empty()
}

/// Words ending -изный.
pub(crate) fn n1229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1198);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3328);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3329);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3330);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7747);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7748);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14219);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 21051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14220);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 14221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3331);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3332);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3333);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14222);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 14223);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 7750);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3334);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7751);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 7752);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3335);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 3336);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7753);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3337);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7754);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озный.
pub(crate) fn n1230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1199);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3338);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7755);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14227);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 21053);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26513);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 14228);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 21054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 14229);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21055);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26514);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3339);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3340);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14231);
							s.is_empty()
						}
						['и', 'л', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 21056);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26515);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21057);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 26516);
											s.is_empty()
										}
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
					super::push(out, 7756);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7757);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14232);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7758);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 7759);
					match s {
						[] => true,
						['а', 'м', '\u{1}', s @ ..] => {
							super::push(out, 14233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 7760);
					match s {
						[] => true,
						['и', 'б', 'м', 'а', 'з', s @ ..] => {
							super::push(out, 14234);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 21059);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 26517);
											s.is_empty()
										}
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
			super::push(out, 3341);
			match s {
				[] => true,
				['и', 'р', 'а', 'в', 'и', s @ ..] => {
					super::push(out, 7761);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7762);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7763);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3342);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7764);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7765);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', 'л', 'л', 'е', 'ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14238);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 14239);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3343);
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7766);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7767);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21060);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 21061);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26518);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21062);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 26519);
											s.is_empty()
										}
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
					super::push(out, 7768);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 14241);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', 'у', 'н', s @ ..] => {
							super::push(out, 14242);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21064);
									s.is_empty()
								}
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
			super::push(out, 3344);
			match s {
				[] => true,
				['п', 'и', 'р', 'г', '\u{1}', s @ ..] => {
					super::push(out, 7769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3345);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7770);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 14243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 7771);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 7772);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 7773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7774);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7775);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21065);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26520);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 26521);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21066);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 26522);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 14246);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 14247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7776);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7777);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t7::n3472(s, out),
		['у', s @ ..] => {
			super::push(out, 3347);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 3348);
			match s {
				[] => true,
				['и', 'т', 'о', s @ ..] => {
					super::push(out, 7782);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 3349);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7783);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7784);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рзный.
pub(crate) fn n1231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1200);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3350);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7785);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узный.
pub(crate) fn n1232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1201);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3351);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7786);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 14252);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 14253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7787);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3352);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7788);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 14255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 7789);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7790);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14257);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21068);
									s.is_empty()
								}
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
			super::push(out, 3353);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3354);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3355);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7793);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14258);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 14259);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3356);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 3357);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 3358);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзный.
pub(crate) fn n1233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1202);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3359);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7797);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 14261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 7798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3360);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14262);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 21069);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 26526);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 21070);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											super::push(out, 26527);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 14263);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 14264);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14265);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 14266);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 21073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 14267);
							match s {
								[] => true,
								['о', 'р', 'п', 'с', s @ ..] => {
									super::push(out, 21074);
									s.is_empty()
								}
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

/// Words ending -язный.
pub(crate) fn n1234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1203);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3361);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7799);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14268);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7800);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14269);
							match s {
								[] => true,
								['е', 'б', 'у', s @ ..] => {
									super::push(out, 21076);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 26528);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 14270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7801);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 14271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3362);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7802);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлейный.
pub(crate) fn n8204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7828);
	match s {
		[] => true,
		['е', 'в', 'о', '\u{1}', s @ ..] => {
			super::push(out, 14292);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улейный.
pub(crate) fn n8206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7830);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -семейный.
pub(crate) fn n3512(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7831);
			match s {
				[] => true,
				['н', 'в', s @ ..] => {
					super::push(out, 14294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7832);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нейный.
pub(crate) fn n3513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3384);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7834);
			match s {
				[] => true,
				['л', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 14296);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пейный.
pub(crate) fn n3514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3385);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7835);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14298);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14299);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14300);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7836);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					super::push(out, 14301);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14302);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рейный.
pub(crate) fn n3515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3386);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7837);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7838);
			match s {
				[] => true,
				['и', 'л', '\u{1}', s @ ..] => {
					super::push(out, 14304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7839);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7840);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14306);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14308);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14309);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 14310);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7841);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 7842);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 14312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сейный.
pub(crate) fn n3516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3387);
	s.is_empty()
}

/// Words ending -тейный.
pub(crate) fn n3517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3388);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7843);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14313);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 21085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7844);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					super::push(out, 14314);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 14315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7845);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7846);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фейный.
pub(crate) fn n3518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3389);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 7847);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шейный.
pub(crate) fn n3519(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7849);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 7850);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7851);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14318);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14319);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ийный.
pub(crate) fn n1247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1215);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3390);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7852);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 14320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3391);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7853);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7854);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14321);
							match s {
								[] => true,
								['л', 'у', s @ ..] => {
									super::push(out, 21087);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26529);
											s.is_empty()
										}
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
					super::push(out, 7855);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3392);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7856);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14323);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 14324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 14325);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 21088);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 26530);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 14326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7857);
					match s {
						[] => true,
						['р', 'о', '-', s @ ..] => {
							super::push(out, 14327);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21089);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 14328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3393);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3394);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7858);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3395);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3396);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7860);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7861);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 14330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3397);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14331);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 14332);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14333);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 21091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7862);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3398);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7863);
					match s {
						[] => true,
						['в', 'а', 'и', s @ ..] => {
							super::push(out, 14335);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 21092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7864);
					match s {
						[] => true,
						['с', 'х', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 21093);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 21094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7865);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14336);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 14337);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 21095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7866);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14338);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 21096);
									s.is_empty()
								}
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
			super::push(out, 3399);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7867);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7868);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 21097);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26531);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21098);
									match s {
										[] => true,
										['а', 'г', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29951);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 29952);
													s.is_empty()
												}
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
					super::push(out, 7869);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21099);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 21100);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26532);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ж', s @ ..] => {
									super::push(out, 21101);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 21102);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 26533);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 21103);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26534);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21104);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 21105);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 21106);
									match s {
										[] => true,
										['е', 'р', 'т', '\u{1}', s @ ..] => {
											super::push(out, 26535);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t7::n8250(s, out),
				['ы', s @ ..] => super::t7::n8251(s, out),
				['я', s @ ..] => super::t7::n8252(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t7::n3530(s, out),
		_ => false,
	}
}

/// Words ending -ойный.
pub(crate) fn n1248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1216);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3401);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7873);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7874);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 7875);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14342);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7876);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14344);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21109);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26536);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21110);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 21111);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', s @ ..] => {
							super::push(out, 14345);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 21112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7877);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7878);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3402);
			match s {
				[] => true,
				['е', '\u{1}', s @ ..] => {
					super::push(out, 7880);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 7881);
					s.is_empty()
				}
				['н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14347);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 14348);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14350);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3403);
			match s {
				[] => true,
				['е', '\u{1}', s @ ..] => {
					super::push(out, 7882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3404);
			match s {
				[] => true,
				['у', 'о', s @ ..] => {
					super::push(out, 7883);
					match s {
						[] => true,
						['к', 'о', s @ ..] => {
							super::push(out, 14352);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 21113);
									s.is_empty()
								}
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
			super::push(out, 3405);
			match s {
				[] => true,
				['н', 'е', 'б', 'о', s @ ..] => {
					super::push(out, 7884);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3406);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3407);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7886);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7887);
					match s {
						[] => true,
						['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 14354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7888);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3408);
			match s {
				[] => true,
				['о', 'д', s @ ..] => {
					super::push(out, 7889);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3409);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3410);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3411);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7890);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 7891);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7892);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21114);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 21115);
									s.is_empty()
								}
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
			super::push(out, 3412);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14357);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26537);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 29953);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 26538);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 29954);
													s.is_empty()
												}
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
							super::push(out, 14358);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14359);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21117);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 21118);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 26539);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 21119);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 26540);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 14360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уйный.
pub(crate) fn n1249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1217);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3413);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3414);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3415);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 14361);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21120);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 14362);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 21121);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 26541);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 29955);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 26542);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 21122);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 26543);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 29956);
													s.is_empty()
												}
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

/// Words ending -хемный.
pub(crate) fn n3566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3433);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14378);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14379);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чемный.
pub(crate) fn n3567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3434);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 7922);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7923);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14380);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъемный.
pub(crate) fn n3568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3435);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7924);
			match s {
				[] => true,
				['о', '-', s @ ..] => {
					super::push(out, 14381);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7925);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							super::push(out, 21133);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 26545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'т', s @ ..] => {
							super::push(out, 21134);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26546);
									s.is_empty()
								}
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
			super::push(out, 7926);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7927);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14382);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21135);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 21136);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 21137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыемный.
pub(crate) fn n3569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3436);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -имный.
pub(crate) fn n1256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1224);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3437);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 3438);
			match s {
				[] => true,
				['е', 'р', 'о', s @ ..] => {
					super::push(out, 7929);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14383);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21138);
									s.is_empty()
								}
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
			super::push(out, 3439);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7930);
					match s {
						[] => true,
						['э', '\u{1}', s @ ..] => {
							super::push(out, 14384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3440);
			match s {
				[] => true,
				['р', 'п', 'е', 'т', 'с', 'о', 'г', 'е', s @ ..] => {
					super::push(out, 7931);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3441);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14386);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 14387);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21139);
									s.is_empty()
								}
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
			super::push(out, 3442);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3443);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 3444);
			match s {
				[] => true,
				['н', 'е', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14388);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 21140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 14389);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 21141);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 26547);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 21142);
									s.is_empty()
								}
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

/// Words ending -лмный.
pub(crate) fn n1257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1225);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3445);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7933);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ммный.
pub(crate) fn n1258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1226);
	match s {
		[] => true,
		['а', 'р', 'г', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					super::push(out, 7934);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14391);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 14392);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 21144);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14393);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 14394);
							match s {
								[] => true,
								['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 21145);
									s.is_empty()
								}
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
			super::push(out, 3446);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7935);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омный.
pub(crate) fn n1259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1227);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3447);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 7936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3448);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7937);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 14395);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21146);
									s.is_empty()
								}
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
			super::push(out, 3449);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3450);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3451);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3452);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7940);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 21147);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 21148);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 26548);
											s.is_empty()
										}
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
					super::push(out, 7941);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14396);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 21149);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 21150);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26549);
											s.is_empty()
										}
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
					super::push(out, 7942);
					match s {
						[] => true,
						['и', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21151);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 21152);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 26550);
											s.is_empty()
										}
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
			super::push(out, 3453);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7943);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 14397);
							match s {
								[] => true,
								['э', '\u{1}', s @ ..] => {
									super::push(out, 21153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 14398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3454);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7944);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3455);
			match s {
				[] => true,
				['а', 'п', 'а', s @ ..] => {
					super::push(out, 7945);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 7946);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14401);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7947);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14403);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 21154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7948);
					match s {
						[] => true,
						['а', 'т', 'а', s @ ..] => {
							super::push(out, 14405);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 21155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'т', s @ ..] => {
							super::push(out, 14406);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21157);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 26551);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'в', s @ ..] => {
									super::push(out, 21158);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26552);
											s.is_empty()
										}
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
					super::push(out, 7949);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14407);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21159);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 21160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 14408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 7950);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21161);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 21162);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 26553);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'а', 'б', s @ ..] => {
									super::push(out, 21163);
									s.is_empty()
								}
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
			super::push(out, 3456);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7951);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14409);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 21164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 14410);
							match s {
								[] => true,
								['о', 'р', 'х', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26554);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 26555);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 29957);
													s.is_empty()
												}
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
							super::push(out, 14411);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 21165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t7::n3591(s, out),
		['х', s @ ..] => {
			super::push(out, 3458);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 3459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рмный.
pub(crate) fn n1260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1228);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3460);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7961);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7962);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14431);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 21179);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26562);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 7963);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3461);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7964);
					s.is_empty()
				}
				['ф', s @ ..] => {
					match s {
						[] => true,
						['и', 'н', s @ ..] => {
							super::push(out, 14433);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 21181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'н', s @ ..] => {
							super::push(out, 14434);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 21182);
									s.is_empty()
								}
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

/// Words ending -умный.
pub(crate) fn n1261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1229);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3462);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3463);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 3464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3465);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7966);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14435);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21183);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 14436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7967);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14437);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3466);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7969);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14438);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21185);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 21186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3467);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3468);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7972);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7973);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7974);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7975);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14442);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21187);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 21188);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 26563);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 14443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3469);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3470);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3471);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7976);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14444);
							match s {
								[] => true,
								['б', 'х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 21189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7977);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 14445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3472);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3473);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7979);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21190);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21191);
									s.is_empty()
								}
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
			super::push(out, 3474);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7980);
					match s {
						[] => true,
						['а', 'в', 'о', s @ ..] => {
							super::push(out, 14446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 3475);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 3476);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7981);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7982);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7983);
					match s {
						[] => true,
						['е', 'б', '\u{1}', s @ ..] => {
							super::push(out, 14447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ымный.
pub(crate) fn n1262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1230);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3477);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юмный.
pub(crate) fn n1263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1231);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 3478);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ямный.
pub(crate) fn n1264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1232);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 3479);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7984);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -анный.
pub(crate) fn n1265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1233);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3480);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7985);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7986);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7987);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14449);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 14450);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14451);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 14452);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 7988);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 7989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t7::n3615(s, out),
		['г', s @ ..] => {
			super::push(out, 3482);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8001);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14494);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 21286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14495);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 21287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 14496);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21288);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 21289);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26657);
											s.is_empty()
										}
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
					super::push(out, 8002);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8003);
					match s {
						[] => true,
						['ы', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 21290);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26658);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 30119);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21291);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8004);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8005);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8006);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14497);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21292);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26659);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 26660);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 26661);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 30120);
													s.is_empty()
												}
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
							super::push(out, 14498);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 14499);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 21293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8007);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14500);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 21294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14501);
							match s {
								[] => true,
								['т', 'с', 'ы', s @ ..] => {
									super::push(out, 21295);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26662);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 21296);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26663);
											s.is_empty()
										}
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
					super::push(out, 8008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t7::n3617(s, out),
		['е', s @ ..] => {
			super::push(out, 3484);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8024);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3485);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8025);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14536);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 21332);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 14537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8026);
					match s {
						[] => true,
						['л', 'б', s @ ..] => {
							super::push(out, 14538);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8027);
					match s {
						[] => true,
						['е', 'д', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21334);
									match s {
										[] => true,
										['о', 'п', 'е', s @ ..] => {
											super::push(out, 26696);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 30128);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => super::t7::n22416(s, out),
								['о', s @ ..] => {
									super::push(out, 21336);
									s.is_empty()
								}
								['с', s @ ..] => super::t7::n22418(s, out),
								['у', s @ ..] => {
									super::push(out, 21338);
									s.is_empty()
								}
								['ы', s @ ..] => super::t7::n22420(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t7::n3620(s, out),
		['и', s @ ..] => super::t7::n3621(s, out),
		['к', s @ ..] => super::t7::n3622(s, out),
		['л', s @ ..] => super::t7::n3623(s, out),
		['м', s @ ..] => super::t7::n3624(s, out),
		['н', s @ ..] => super::t7::n3625(s, out),
		['п', s @ ..] => super::t7::n3626(s, out),
		['р', s @ ..] => super::t7::n3627(s, out),
		['с', s @ ..] => super::t7::n3628(s, out),
		['т', s @ ..] => super::t7::n3629(s, out),
		['у', s @ ..] => super::t7::n3630(s, out),
		['ф', s @ ..] => super::t7::n3631(s, out),
		['х', s @ ..] => super::t7::n3632(s, out),
		['ц', s @ ..] => {
			super::push(out, 3499);
			s.is_empty()
		}
		['ч', s @ ..] => super::t7::n3634(s, out),
		['ш', s @ ..] => super::t7::n3635(s, out),
		['щ', s @ ..] => super::t7::n3636(s, out),
		_ => false,
	}
}

/// Words ending -енный.
pub(crate) fn n1266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1234);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3503);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8138);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8139);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14685);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 14686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3504);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8140);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14687);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 21508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8141);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14688);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 21509);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14689);
							match s {
								[] => true,
								['г', 'ы', s @ ..] => {
									super::push(out, 21510);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26831);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 14690);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 14691);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21512);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 21513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8142);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8143);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8144);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t7::n3639(s, out),
		['г', s @ ..] => {
			super::push(out, 3506);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8155);
					s.is_empty()
				}
				['а', 'т', 'у', 'м', 'и', s @ ..] => {
					super::push(out, 8156);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8157);
					match s {
						[] => true,
						['р', 'р', 'е', 'т', '\u{1}', s @ ..] => {
							super::push(out, 14723);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14724);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 21547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'у', s @ ..] => {
							super::push(out, 14725);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 21548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8158);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14726);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 14727);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 14728);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 21549);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26875);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 26876);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21550);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 21551);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 26877);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 14729);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 21552);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26878);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', 'а', s @ ..] => {
							super::push(out, 14730);
							match s {
								[] => true,
								['г', '\u{1}', s @ ..] => {
									super::push(out, 21553);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => super::t7::n15481(s, out),
						['н', s @ ..] => super::t7::n15482(s, out),
						['п', s @ ..] => super::t7::n15483(s, out),
						['р', s @ ..] => super::t7::n15484(s, out),
						['с', s @ ..] => super::t7::n15485(s, out),
						['т', s @ ..] => super::t7::n15486(s, out),
						['ф', s @ ..] => super::t7::n15487(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t7::n8550(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t7::n3641(s, out),
		['е', s @ ..] => super::t7::n3642(s, out),
		['ж', s @ ..] => super::t7::n3643(s, out),
		['з', s @ ..] => super::t7::n3644(s, out),
		['и', s @ ..] => super::t7::n3645(s, out),
		['к', s @ ..] => super::t7::n3646(s, out),
		['л', s @ ..] => super::t7::n3647(s, out),
		['м', s @ ..] => super::t7::n3648(s, out),
		['н', s @ ..] => super::t7::n3649(s, out),
		['о', s @ ..] => super::t7::n3650(s, out),
		['п', s @ ..] => super::t7::n3651(s, out),
		['р', s @ ..] => super::t7::n3652(s, out),
		['с', s @ ..] => super::t7::n3653(s, out),
		['т', s @ ..] => super::t7::n3654(s, out),
		['у', s @ ..] => super::t7::n3655(s, out),
		['ф', s @ ..] => super::t7::n3656(s, out),
		['ц', s @ ..] => super::t7::n3657(s, out),
		['ч', s @ ..] => super::t7::n3658(s, out),
		['ш', s @ ..] => super::t7::n3659(s, out),
		['щ', s @ ..] => super::t7::n3660(s, out),
		_ => false,
	}
}

/// Words ending -рбинный.
pub(crate) fn n8735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8343);
	match s {
		[] => true,
		['у', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22342);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22343);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -винный.
pub(crate) fn n3663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3529);
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15247);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 8345);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8346);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8347);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8348);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15251);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15252);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22344);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 22345);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 22346);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15253);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15254);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 22348);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27563);
									s.is_empty()
								}
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

/// Words ending -гинный.
pub(crate) fn n3664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3530);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 8349);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8350);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -динный.
pub(crate) fn n3665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3531);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8351);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8352);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8353);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15257);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15258);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8354);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 8355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жинный.
pub(crate) fn n3666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3532);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8356);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15259);
					match s {
						[] => true,
						['к', 'с', 'и', s @ ..] => {
							super::push(out, 22351);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 15260);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8357);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 8358);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15262);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зинный.
pub(crate) fn n3667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3533);
	match s {
		[] => true,
		['а', 'г', 'а', 'м', '\u{1}', s @ ..] => {
			super::push(out, 8359);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 8360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -линный.
pub(crate) fn n3669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3535);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8361);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15263);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22353);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 27565);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 27566);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 27567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8362);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 8363);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8364);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15266);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15267);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22354);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 22355);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15268);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8365);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 15269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8366);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15270);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27569);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 30659);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 27570);
									s.is_empty()
								}
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
			super::push(out, 8367);
			match s {
				[] => true,
				['д', 'н', s @ ..] => {
					super::push(out, 15271);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8368);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15272);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 22358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8369);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 8370);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15273);
					match s {
						[] => true,
						['п', 'с', 'е', 'б', '\u{1}', s @ ..] => {
							super::push(out, 22359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8371);
			match s {
				[] => true,
				['б', 'о', s @ ..] => {
					super::push(out, 15274);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -минный.
pub(crate) fn n3670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3536);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8372);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8373);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15275);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15276);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 15277);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['и', 'в', '\u{1}', s @ ..] => {
							super::push(out, 22360);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8374);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8375);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15278);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8376);
			s.is_empty()
		}
		['с', 'а', 'ж', '\u{1}', s @ ..] => {
			super::push(out, 8377);
			s.is_empty()
		}
		['т', '\u{1}', s @ ..] => {
			super::push(out, 8378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нинный.
pub(crate) fn n3671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3537);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8379);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 8380);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8381);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15279);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							super::push(out, 22363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8382);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 8383);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 8384);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', 'м', 'и', '\u{1}', s @ ..] => {
			super::push(out, 8385);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ринный.
pub(crate) fn n3673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3539);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8386);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15281);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8387);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 8388);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					super::push(out, 15283);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8389);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15284);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 15285);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15286);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8390);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -синный.
pub(crate) fn n3674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3540);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8391);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15287);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'а', s @ ..] => {
			super::push(out, 8392);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15289);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тинный.
pub(crate) fn n3675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3541);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8393);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8394);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 8395);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 8396);
			s.is_empty()
		}
		['о', 'к', s @ ..] => {
			super::push(out, 8397);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 8398);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8399);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15291);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 15292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8400);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15293);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15294);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уинный.
pub(crate) fn n3676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3542);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 8401);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -финный.
pub(crate) fn n3677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3543);
	s.is_empty()
}

/// Words ending -хинный.
pub(crate) fn n3678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3544);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15295);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чинный.
pub(crate) fn n3679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3545);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8403);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 8404);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15297);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 22367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8405);
			match s {
				[] => true,
				['р', 'п', 'с', s @ ..] => {
					super::push(out, 15299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8406);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8407);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15301);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 22368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15302);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шинный.
pub(crate) fn n3680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3546);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8408);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8409);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15304);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22370);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 22371);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27571);
									match s {
										[] => true,
										['р', 'т', '\u{1}', s @ ..] => {
											super::push(out, 30660);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 27572);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30661);
											s.is_empty()
										}
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
					super::push(out, 15305);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['и', 'р', s @ ..] => {
									super::push(out, 27573);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30662);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 27574);
									s.is_empty()
								}
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
			super::push(out, 8410);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щинный.
pub(crate) fn n3681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3547);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8411);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 22372);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27575);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 22373);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 27576);
									s.is_empty()
								}
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
			super::push(out, 8412);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8413);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 8414);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онный.
pub(crate) fn n1268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1236);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3548);
			s.is_empty()
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8415);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8416);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8417);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3549);
			match s {
				[] => true,
				['а', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15310);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15311);
							s.is_empty()
						}
						['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 15312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8419);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15314);
							s.is_empty()
						}
						['ч', 'л', s @ ..] => {
							super::push(out, 15315);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8420);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15316);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8421);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 15317);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 15318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8422);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'и', s @ ..] => {
					super::push(out, 8423);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 15320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3550);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8424);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8425);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15321);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 15322);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8426);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 22377);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 22378);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27577);
											s.is_empty()
										}
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
					super::push(out, 8427);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8428);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 8429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3551);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 15324);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 15325);
							match s {
								[] => true,
								['а', 'и', 'д', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 22379);
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
						['м', s @ ..] => {
							super::push(out, 15326);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15327);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22380);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 27578);
											s.is_empty()
										}
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
		['и', s @ ..] => super::t7::n3687(s, out),
		['к', s @ ..] => {
			super::push(out, 3553);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8438);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22416);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 22417);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27657);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 15353);
							match s {
								[] => true,
								['ф', s @ ..] => {
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
					super::push(out, 8439);
					s.is_empty()
				}
				['л', 'а', 'б', '\u{1}', s @ ..] => {
					super::push(out, 8440);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8441);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 15354);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 15355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22420);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 22421);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 15356);
							match s {
								[] => true,
								['о', 'в', 'о', 'т', s @ ..] => {
									super::push(out, 22422);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 27658);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 22423);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 22424);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 27659);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							super::push(out, 15357);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 15358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8442);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15359);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 22425);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15360);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 8443);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t7::n3689(s, out),
		['м', s @ ..] => super::t7::n3690(s, out),
		['н', s @ ..] => super::t7::n3691(s, out),
		['п', s @ ..] => super::t7::n3692(s, out),
		['р', s @ ..] => super::t7::n3693(s, out),
		['с', s @ ..] => super::t7::n3694(s, out),
		['т', s @ ..] => super::t7::n3695(s, out),
		['ф', s @ ..] => super::t7::n3696(s, out),
		['х', s @ ..] => super::t7::n3697(s, out),
		['ь', s @ ..] => super::t7::n3698(s, out),
		['ю', 'л', 'г', '\u{1}', s @ ..] => super::t7::n3699(s, out),
		_ => false,
	}
}

/// Words ending -унный.
pub(crate) fn n1269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1237);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3565);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8479);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 15409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3566);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 3567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8481);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8483);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8484);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8485);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8486);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3568);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3569);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8488);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8489);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3570);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8490);
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15413);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15414);
							match s {
								[] => true,
								['т', 'я', s @ ..] => {
									super::push(out, 22462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15415);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 15416);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22463);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 30760);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t7::n33941(s, out),
														_ => false,
													}
												}
												['ы', s @ ..] => {
													super::push(out, 30761);
													match s {
														[] => true,
														['т', s @ ..] => super::t7::n33942(s, out),
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
		['т', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8491);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15417);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 15418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 3571);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8493);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынный.
pub(crate) fn n1270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1238);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3572);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3573);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8494);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8495);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8496);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -энный.
pub(crate) fn n1271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1239);
	s.is_empty()
}

/// Words ending -юнный.
pub(crate) fn n1272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1240);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3574);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8497);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янный.
pub(crate) fn n1273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1241);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3575);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15422);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8499);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8500);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15425);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8501);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 15427);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 8502);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 8503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15428);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15429);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 15430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3576);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8504);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3577);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8506);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8507);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15431);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 22467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8508);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22468);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 22469);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 27682);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15433);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15434);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22470);
									s.is_empty()
								}
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
			super::push(out, 3578);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8509);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8510);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3579);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8511);
					match s {
						[] => true,
						['в', 'ы', s @ ..] => {
							super::push(out, 15437);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8512);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15438);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15439);
							match s {
								[] => true,
								['т', 'с', 'т', s @ ..] => {
									super::push(out, 22473);
									match s {
										[] => true,
										['о', 'е', s @ ..] => {
											super::push(out, 27683);
											match s {
												[] => true,
												['н', s @ ..] => {
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
				['к', s @ ..] => {
					super::push(out, 8513);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15440);
							match s {
								[] => true,
								['т', 'с', 'г', s @ ..] => {
									super::push(out, 22474);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27684);
											s.is_empty()
										}
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
					super::push(out, 8514);
					match s {
						[] => true,
						['г', 'ы', s @ ..] => {
							super::push(out, 15441);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 8515);
					match s {
						[] => true,
						['а', 'к', 'ы', s @ ..] => {
							super::push(out, 15442);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22476);
									s.is_empty()
								}
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
			super::push(out, 3580);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8516);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15443);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22477);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 27685);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 27686);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 30763);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 27687);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 22478);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27688);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 22479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8517);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3581);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8518);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22480);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22481);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 22482);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 27689);
											s.is_empty()
										}
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
		['о', s @ ..] => super::t7::n3719(s, out),
		['р', s @ ..] => super::t7::n3720(s, out),
		['у', s @ ..] => super::t7::n3721(s, out),
		['ь', s @ ..] => super::t7::n3722(s, out),
		_ => false,
	}
}

/// Words ending -упный.
pub(crate) fn n1283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1251);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 3600);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3601);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3602);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'п', 'г', s @ ..] => {
							super::push(out, 15468);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22504);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27699);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15469);
							match s {
								[] => true,
								['д', 'е', 'с', s @ ..] => {
									super::push(out, 22505);
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
						['у', s @ ..] => {
							super::push(out, 15470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22506);
									s.is_empty()
								}
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

/// Words ending -ерный.
pub(crate) fn n1285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1253);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3619);
			match s {
				[] => true,
				['а', 'ж', 'о', s @ ..] => {
					super::push(out, 8566);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15492);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8567);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8569);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8570);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15493);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 8571);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15495);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22513);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22514);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 22515);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 27702);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 15496);
							match s {
								[] => true,
								['е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 22516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8572);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15497);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22517);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 15498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15499);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15500);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22519);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 15501);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 22520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8573);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8574);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8575);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22521);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 22522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8576);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15504);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 22523);
									match s {
										[] => true,
										['е', 'л', '\u{1}', s @ ..] => {
											super::push(out, 27703);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 22524);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27704);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8577);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8578);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 8579);
					match s {
						[] => true,
						['л', 'о', 'в', 'е', 'р', '\u{1}', s @ ..] => {
							super::push(out, 15508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3621);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8580);
					match s {
						[] => true,
						['л', 'р', s @ ..] => {
							super::push(out, 15509);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22525);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 8581);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 8582);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8583);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15511);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 27705);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 27706);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 30764);
													s.is_empty()
												}
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
					super::push(out, 8584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3622);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8585);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15512);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 15513);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22526);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 22527);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 27707);
											s.is_empty()
										}
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
					super::push(out, 8586);
					match s {
						[] => true,
						['е', 'г', 'с', s @ ..] => {
							super::push(out, 15514);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22528);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 27708);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 22529);
									match s {
										[] => true,
										['а', 'р', 'т', '\u{1}', s @ ..] => {
											super::push(out, 27709);
											s.is_empty()
										}
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
					super::push(out, 8587);
					match s {
						[] => true,
						['м', s @ ..] => super::t7::n16297(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t7::n9008(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t7::n3762(s, out),
		['ж', s @ ..] => super::t7::n3763(s, out),
		['з', s @ ..] => super::t7::n3764(s, out),
		['й', s @ ..] => super::t7::n3765(s, out),
		['к', s @ ..] => super::t7::n3766(s, out),
		['л', s @ ..] => super::t7::n3767(s, out),
		['м', s @ ..] => super::t7::n3768(s, out),
		['н', s @ ..] => super::t7::n3769(s, out),
		['п', s @ ..] => super::t7::n3770(s, out),
		['с', s @ ..] => super::t7::n3771(s, out),
		['т', s @ ..] => super::t7::n3772(s, out),
		['у', s @ ..] => super::t7::n3773(s, out),
		['ф', s @ ..] => super::t7::n3774(s, out),
		['х', s @ ..] => super::t7::n3775(s, out),
		['ц', s @ ..] => super::t7::n3776(s, out),
		['ч', s @ ..] => super::t7::n3777(s, out),
		['ш', s @ ..] => super::t7::n3778(s, out),
		['щ', s @ ..] => super::t7::n3779(s, out),
		['ь', s @ ..] => super::t7::n3780(s, out),
		_ => false,
	}
}

/// Words ending -ирный.
pub(crate) fn n1286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1254);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3642);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 3643);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3644);
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					super::push(out, 8668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3645);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8669);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8670);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8671);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 8672);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3646);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3647);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8674);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8675);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3648);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8677);
					match s {
						[] => true,
						['в', 'у', 'с', '-', s @ ..] => {
							super::push(out, 15588);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8678);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8679);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 22561);
									s.is_empty()
								}
								['ш', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27731);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 27732);
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
								['т', 'о', s @ ..] => {
									super::push(out, 22562);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 22563);
									s.is_empty()
								}
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
			super::push(out, 3649);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3650);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3651);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8680);
					s.is_empty()
				}
				['р', 'а', 'в', 'к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15589);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 15590);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 27733);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 30774);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 27734);
											s.is_empty()
										}
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
		['ф', s @ ..] => {
			super::push(out, 3652);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8681);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 8682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3653);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8683);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 3654);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орный.
pub(crate) fn n1287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1255);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3655);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8684);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15592);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 15593);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8685);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8686);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8687);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8688);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8689);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8690);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15595);
							match s {
								[] => true,
								['-', 'о', 'н', 'т', 'е', 'ч', 'т', 'о', '\u{1}', s @ ..] => {
									super::push(out, 22565);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22566);
									s.is_empty()
								}
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
			super::push(out, 3656);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8691);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15596);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15597);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8692);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22568);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 27735);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 27736);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 22569);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 22570);
									match s {
										[] => true,
										['а', 'р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 30775);
													s.is_empty()
												}
												['-', s @ ..] => {
													super::push(out, 30776);
													match s {
														[] => true,
														['о', s @ ..] => super::t7::n33944(s, out),
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
									super::push(out, 22571);
									match s {
										[] => true,
										['д', '-', s @ ..] => {
											super::push(out, 27737);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 30777);
													s.is_empty()
												}
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
							super::push(out, 15598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8693);
					match s {
						[] => true,
						['о', 'д', s @ ..] => {
							super::push(out, 15599);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 22572);
									s.is_empty()
								}
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
			super::push(out, 3657);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8694);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8695);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3658);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8697);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 15600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8698);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8699);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 15601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3659);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 3660);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 3661);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8700);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8701);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8702);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15602);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8703);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15604);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 22574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8704);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15605);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 15606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8705);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 8706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3662);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8707);
					match s {
						[] => true,
						['е', 'т', 'с', 'о', s @ ..] => {
							super::push(out, 15607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3663);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8708);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15608);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 15609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8709);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8710);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 8711);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 15611);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 22575);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 22576);
									s.is_empty()
								}
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
			super::push(out, 3664);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8712);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 8713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3665);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8714);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15612);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 22577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8715);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8716);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15614);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 15615);
							match s {
								[] => true,
								['у', s @ ..] => super::t7::n23732(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t7::n3805(s, out),
		['п', s @ ..] => super::t7::n3806(s, out),
		['р', s @ ..] => super::t7::n3807(s, out),
		['с', s @ ..] => super::t7::n3808(s, out),
		['т', s @ ..] => super::t8::n3809(s, out),
		['ф', s @ ..] => super::t8::n3810(s, out),
		['х', s @ ..] => super::t8::n3811(s, out),
		['ш', s @ ..] => super::t8::n3812(s, out),
		_ => false,
	}
}

/// Words ending -урный.
pub(crate) fn n1288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1256);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3674);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8744);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 15658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3675);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8745);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 8746);
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							super::push(out, 15659);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22626);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 15660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3676);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 3677);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 3678);
			match s {
				[] => true,
				['а', 'б', s @ ..] => {
					super::push(out, 8747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3679);
			match s {
				[] => true,
				['н', 'е', 'ц', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15661);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15662);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 22627);
									s.is_empty()
								}
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
			super::push(out, 3680);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8748);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3681);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8749);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8750);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 8751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3682);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8752);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8753);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3683);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8754);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15667);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 15668);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22628);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 22629);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 30797);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 30798);
													match s {
														[] => true,
														['у', s @ ..] => super::t8::n33951(s, out),
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
							super::push(out, 15669);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['п', 'м', 'е', 'т', 'о', s @ ..] => {
											super::push(out, 27778);
											s.is_empty()
										}
										['т', s @ ..] => {
											match s {
												[] => true,
												['и', 'л', 'е', 'щ', s @ ..] => {
													super::push(out, 30799);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 30800);
													match s {
														[] => true,
														['и', s @ ..] => super::t8::n33952(s, out),
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
									super::push(out, 22630);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8755);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 8756);
					match s {
						[] => true,
						['а', 'ф', 'у', s @ ..] => {
							super::push(out, 15670);
							s.is_empty()
						}
						['е', 'т', s @ ..] => {
							super::push(out, 15671);
							s.is_empty()
						}
						['н', 'у', 'п', 'у', s @ ..] => {
							super::push(out, 15672);
							s.is_empty()
						}
						['у', 'р', 'т', 'с', 'о', s @ ..] => {
							super::push(out, 15673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8757);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15674);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 22631);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 22632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15675);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8758);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 8759);
					match s {
						[] => true,
						['л', 'у', 'к', s @ ..] => {
							match s {
								[] => true,
								['о', 'и', s @ ..] => {
									super::push(out, 22634);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 27779);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 30801);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 22635);
									s.is_empty()
								}
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
			super::push(out, 3684);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8760);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 3685);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 8762);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярный.
pub(crate) fn n1291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1259);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8765);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 8766);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8767);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15676);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 15677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8768);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 15678);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 15679);
							match s {
								[] => true,
								['н', 'а', 'л', 'г', 'и', s @ ..] => {
									super::push(out, 22636);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 27780);
											s.is_empty()
										}
										['р', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 30802);
													s.is_empty()
												}
												['ю', s @ ..] => {
													super::push(out, 30803);
													s.is_empty()
												}
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
							super::push(out, 15680);
							match s {
								[] => true,
								['е', 'л', 'о', 'м', 'и', s @ ..] => {
									super::push(out, 22637);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27781);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 30804);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 22638);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 27782);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 30805);
													match s {
														[] => true,
														['к', s @ ..] => super::t8::n33953(s, out),
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
									super::push(out, 22639);
									match s {
										[] => true,
										['а', 'в', '\u{1}', s @ ..] => {
											super::push(out, 27783);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 15681);
							match s {
								[] => true,
								['о', 'п', 'х', s @ ..] => {
									super::push(out, 22640);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27784);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 30806);
													s.is_empty()
												}
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

/// Words ending -рко-красный.
pub(crate) fn n32329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30807);
	s.is_empty()
}

/// Words ending -мно-красный.
pub(crate) fn n32330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30808);
	match s {
		[] => true,
		['е', 'т', '\u{1}', s @ ..] => {
			super::push(out, 32345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нно-красный.
pub(crate) fn n32331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30809);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 32346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рно-красный.
pub(crate) fn n32332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30810);
	match s {
		[] => true,
		['е', 'ч', '\u{1}', s @ ..] => {
			super::push(out, 32347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -желто-красный.
pub(crate) fn n32333(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32348);
			s.is_empty()
		}
		['-', 'о', 'н', 'р', 'е', 'ч', '\u{1}', s @ ..] => {
			super::push(out, 32349);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лесный.
pub(crate) fn n3844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3703);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8781);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8782);
			match s {
				[] => true,
				['к', 'х', s @ ..] => {
					super::push(out, 15695);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22644);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 30811);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32350);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 30812);
											match s {
												[] => true,
												['т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 32351);
													s.is_empty()
												}
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
							super::push(out, 22645);
							match s {
								[] => true,
								['в', 'д', '\u{1}', s @ ..] => {
									super::push(out, 27788);
									s.is_empty()
								}
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

/// Words ending -месный.
pub(crate) fn n3845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3704);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8783);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ресный.
pub(crate) fn n3846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3705);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 8785);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8786);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15697);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 15698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8787);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15699);
					match s {
						[] => true,
						['о', 'в', 'е', 'ж', s @ ..] => {
							super::push(out, 22647);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27789);
									s.is_empty()
								}
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
			super::push(out, 8788);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тесный.
pub(crate) fn n3847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3706);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8789);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8790);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -исный.
pub(crate) fn n1294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1262);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3707);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3708);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8792);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8793);
					match s {
						[] => true,
						['р', 'к', 'о', s @ ..] => {
							super::push(out, 15701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3709);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 8794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3710);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8795);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 8796);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3711);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8797);
					match s {
						[] => true,
						['н', 'и', 'ш', s @ ..] => {
							super::push(out, 15703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8798);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 8799);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3712);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8800);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15706);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22648);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 15707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8801);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 8802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3713);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 3714);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8804);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8805);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15708);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22649);
									s.is_empty()
								}
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

/// Words ending -ксный.
pub(crate) fn n1296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1264);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3715);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3716);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8806);
					match s {
						[] => true,
						['н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22650);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8807);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15709);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 15710);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22652);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 27790);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', 'о', 'к', 'а', s @ ..] => {
									super::push(out, 22653);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27791);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 15711);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8808);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3717);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3718);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нсный.
pub(crate) fn n1297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1265);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3719);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8809);
					match s {
						[] => true,
						['а', 'б', 'м', s @ ..] => {
							super::push(out, 15713);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									super::push(out, 22655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3720);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8811);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3721);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8812);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3722);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 8813);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15716);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осный.
pub(crate) fn n1298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1266);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 3723);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8814);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15717);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22656);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8815);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22657);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'о', 'н', 'е', 'л', 'е', 'з', '\u{1}', s @ ..] => {
					super::push(out, 8816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3724);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8817);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8818);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22658);
									match s {
										[] => true,
										['к', 'о', s @ ..] => {
											super::push(out, 27792);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 22659);
									s.is_empty()
								}
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
			super::push(out, 3725);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8819);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3726);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8820);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8821);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15722);
							match s {
								[] => true,
								['о', 'в', s @ ..] => {
									super::push(out, 22660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 15723);
							match s {
								[] => true,
								['а', 'м', s @ ..] => {
									super::push(out, 22661);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 15724);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 22662);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 27793);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15725);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22663);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27794);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 30813);
													s.is_empty()
												}
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
							super::push(out, 15726);
							match s {
								[] => true,
								['е', 'л', 'о', 'т', 'р', 'е', 'в', '\u{1}', s @ ..] => {
									super::push(out, 22664);
									s.is_empty()
								}
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
			super::push(out, 3727);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8822);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8823);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8824);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15727);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 15728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'о', 'в', s @ ..] => {
					super::push(out, 8825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3729);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8826);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22666);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22667);
									match s {
										[] => true,
										['р', s @ ..] => {
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
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 3730);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 3731);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 15729);
							s.is_empty()
						}
						['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 15730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -псный.
pub(crate) fn n1299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1267);
	match s {
		[] => true,
		['о', 'л', 'ф', '\u{1}', s @ ..] => {
			super::push(out, 3732);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рсный.
pub(crate) fn n1300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1268);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8828);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22668);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 22669);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 27796);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15731);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 15732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8829);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 15733);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22670);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 27797);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 8830);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 15735);
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
				['б', s @ ..] => {
					super::push(out, 8831);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 8832);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15736);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22671);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8833);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссный.
pub(crate) fn n1301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1269);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3733);
			match s {
				[] => true,
				['л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15738);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15739);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15740);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 15741);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22672);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 27798);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 15742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3734);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					super::push(out, 8835);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15743);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22673);
									s.is_empty()
								}
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
			super::push(out, 3735);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -усный.
pub(crate) fn n1302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1270);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3736);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 3737);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8836);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15744);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 22674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 8837);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3738);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8839);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22675);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 27799);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 30814);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 22676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8840);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15745);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 22677);
									s.is_empty()
								}
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
			super::push(out, 3739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8841);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 15746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3740);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8842);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 8843);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 8844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3741);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15747);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15748);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 22678);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 27800);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 22679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 15749);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8846);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 15751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8847);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15752);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15753);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 15754);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 22681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 15755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3742);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3743);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8848);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8849);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15757);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22682);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22683);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 27801);
											s.is_empty()
										}
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
					super::push(out, 8850);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15758);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 15759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3744);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3745);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3746);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 15760);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 15761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8852);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22684);
									match s {
										[] => true,
										['н', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 30815);
													match s {
														[] => true,
														['р', s @ ..] => super::t8::n33961(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 30816);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 27802);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 22685);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 27803);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22686);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 27804);
											s.is_empty()
										}
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
					super::push(out, 8853);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15762);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22687);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 8854);
					match s {
						[] => true,
						['ъ', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 15763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3747);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8855);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8856);
					match s {
						[] => true,
						['у', 'о', 'н', 'с', s @ ..] => {
							super::push(out, 15765);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22688);
									s.is_empty()
								}
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
			super::push(out, 3748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьсный.
pub(crate) fn n1304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1272);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юсный.
pub(crate) fn n1305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1273);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3750);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3751);
			match s {
				[] => true,
				['о', 'п', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 8857);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясный.
pub(crate) fn n1306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1274);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3752);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3753);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3754);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 8859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3755);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иевопрокатный.
pub(crate) fn n32340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30817);
	s.is_empty()
}

/// Words ending -хматный.
pub(crate) fn n9303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8876);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15773);
			match s {
				[] => true,
				['ш', 'е', s @ ..] => {
					super::push(out, 22698);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -натный.
pub(crate) fn n3907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3764);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8877);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15774);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15775);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 22699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 8878);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 8879);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15776);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8880);
			match s {
				[] => true,
				['д', 'р', 'о', 'о', 'к', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 15778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 8881);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15779);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27809);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 27810);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27811);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 27812);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30818);
											match s {
												[] => true,
												['р', 'т', '\u{1}', s @ ..] => {
													super::push(out, 32353);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', 'в', 'д', '\u{1}', s @ ..] => {
											super::push(out, 30819);
											s.is_empty()
										}
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
			super::push(out, 8882);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 8883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ратный.
pub(crate) fn n3908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3765);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8884);
			match s {
				[] => true,
				['п', 'п', s @ ..] => {
					super::push(out, 15780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8885);
			match s {
				[] => true,
				['о', 'о', s @ ..] => {
					super::push(out, 15781);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 22700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8886);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8887);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15782);
					match s {
						[] => true,
						['в', 'к', 'и', s @ ..] => {
							super::push(out, 22701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8888);
			match s {
				[] => true,
				['е', 'о', s @ ..] => {
					super::push(out, 15783);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 15784);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15785);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 15786);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27813);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30820);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 27814);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15787);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 15788);
					match s {
						[] => true,
						['н', 'з', s @ ..] => {
							super::push(out, 22702);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27815);
									s.is_empty()
								}
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

/// Words ending -татный.
pub(crate) fn n3910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3767);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 8889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 8890);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15790);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 15791);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22703);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 27816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 15792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фатный.
pub(crate) fn n3911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3768);
	s.is_empty()
}

/// Words ending -хатный.
pub(crate) fn n3912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3769);
	match s {
		[] => true,
		['р', 'а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15793);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 15794);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чатный.
pub(crate) fn n3913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3770);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8891);
			match s {
				[] => true,
				['п', 'о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22704);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 22705);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27818);
									s.is_empty()
								}
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
			super::push(out, 8892);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15795);
					match s {
						[] => true,
						['в', 'а', s @ ..] => {
							super::push(out, 22706);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 27819);
									s.is_empty()
								}
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

/// Words ending -етный.
pub(crate) fn n1308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1276);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3771);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3772);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8894);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 8895);
					match s {
						[] => true,
						['х', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 22707);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 22708);
									s.is_empty()
								}
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
			super::push(out, 3773);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3774);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 3775);
			match s {
				[] => true,
				['д', 'ю', 'б', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 15797);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 8897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3776);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 3777);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8898);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 15799);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 15800);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22710);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22711);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 22712);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22713);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 27820);
											s.is_empty()
										}
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
					super::push(out, 8899);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15801);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8900);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8901);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15803);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 22715);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27821);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 22716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8902);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22717);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 22718);
									s.is_empty()
								}
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
			super::push(out, 3778);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8903);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8904);
					match s {
						[] => true,
						['б', 'р', s @ ..] => {
							super::push(out, 15804);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8905);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8906);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15805);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 15806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8907);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15807);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22720);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 15808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8908);
					match s {
						[] => true,
						['м', 'а', s @ ..] => {
							super::push(out, 15809);
							s.is_empty()
						}
						['п', 'д', s @ ..] => {
							super::push(out, 15810);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15811);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 27822);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 27823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 15813);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 15814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8909);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3779);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8911);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8912);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8913);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15815);
							match s {
								[] => true,
								['у', 'п', '-', s @ ..] => {
									super::push(out, 22721);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27824);
											match s {
												[] => true,
												['в', 'о', 'к', 'л', 'е', 'р', 'т', 'с', '\u{1}', s @ ..] => {
													super::push(out, 30821);
													s.is_empty()
												}
												['н', 'т', 'а', 'м', 'о', 'т', 'в', 'а', '\u{1}', s @ ..] => {
													super::push(out, 30822);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => super::t8::n16622(s, out),
						['р', s @ ..] => {
							super::push(out, 15817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8914);
					match s {
						[] => true,
						['р', s @ ..] => super::t8::n16624(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t8::n9347(s, out),
				['с', s @ ..] => super::t8::n9348(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t8::n3923(s, out),
		['п', s @ ..] => super::t8::n3924(s, out),
		['р', s @ ..] => super::t8::n3925(s, out),
		['с', s @ ..] => super::t8::n3926(s, out),
		['т', s @ ..] => super::t8::n3927(s, out),
		['у', s @ ..] => super::t8::n3928(s, out),
		['ф', s @ ..] => super::t8::n3929(s, out),
		['ч', s @ ..] => super::t8::n3930(s, out),
		['ш', s @ ..] => super::t8::n3931(s, out),
		_ => false,
	}
}

/// Words ending -итный.
pub(crate) fn n1309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1277);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3789);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8933);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8934);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8935);
					match s {
						[] => true,
						['м', 'е', s @ ..] => {
							super::push(out, 15842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', 'к', s @ ..] => {
			super::push(out, 3790);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 3791);
			match s {
				[] => true,
				['н', 'и', s @ ..] => {
					super::push(out, 8937);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3792);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8938);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'и', s @ ..] => {
			super::push(out, 3793);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 3794);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8939);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8940);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 8941);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15846);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 22737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', 'а', s @ ..] => {
			super::push(out, 3795);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3796);
			match s {
				[] => true,
				['г', 'а', 'м', 'о', 'р', 'т', s @ ..] => {
					super::push(out, 8942);
					match s {
						[] => true,
						['к', 'е', 'л', 'э', '\u{1}', s @ ..] => {
							super::push(out, 15847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3797);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3798);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 3799);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8944);
					match s {
						[] => true,
						['р', 'п', 'н', s @ ..] => {
							super::push(out, 15848);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3800);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8945);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8946);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15849);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 22739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 3801);
			match s {
				[] => true,
				['а', 'з', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15850);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 22740);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 27832);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15851);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 22741);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27833);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 30825);
													s.is_empty()
												}
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

/// Words ending -йтный.
pub(crate) fn n1310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1278);
	match s {
		[] => true,
		['а', 'б', '\u{1}', s @ ..] => {
			super::push(out, 3802);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ктный.
pub(crate) fn n1311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1279);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3803);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8947);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8948);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8949);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15852);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22742);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 27834);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 15853);
							match s {
								[] => true,
								['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 22744);
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
							super::push(out, 15854);
							s.is_empty()
						}
						['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 15855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3804);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8950);
					s.is_empty()
				}
				['ф', 'е', s @ ..] => {
					super::push(out, 8951);
					match s {
						[] => true,
						['д', 'з', s @ ..] => {
							super::push(out, 15856);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22745);
									s.is_empty()
								}
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
			super::push(out, 3805);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15857);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 22746);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 15858);
							match s {
								[] => true,
								['н', 'о', 'к', 'с', s @ ..] => {
									super::push(out, 22747);
									s.is_empty()
								}
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

/// Words ending -лтный.
pub(crate) fn n1312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1280);
	s.is_empty()
}

/// Words ending -нтный.
pub(crate) fn n1313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1281);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3806);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8952);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 8953);
					s.is_empty()
				}
				['н', 'и', s @ ..] => {
					super::push(out, 8954);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8955);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8956);
					match s {
						[] => true,
						['е', 'д', '-', s @ ..] => {
							super::push(out, 15860);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'к', s @ ..] => {
					super::push(out, 8957);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3807);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8958);
					match s {
						[] => true,
						['р', 'о', 'с', 'б', s @ ..] => {
							super::push(out, 15862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8959);
					match s {
						[] => true,
						['к', 'е', 'с', 'б', s @ ..] => {
							super::push(out, 15863);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 22749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 8960);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15864);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 8961);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8962);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15865);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 15866);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15867);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 22751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8963);
					match s {
						[] => true,
						['а', 'в', 'х', s @ ..] => {
							super::push(out, 15868);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 27835);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 27836);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 30826);
													match s {
														[] => true,
														['е', 'ч', '\u{1}', s @ ..] => super::t8::n33963(s, out),
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
				['м', s @ ..] => {
					super::push(out, 8964);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15869);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 22752);
									s.is_empty()
								}
								['л', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 27837);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 27838);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 30827);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 22753);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 22754);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27839);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22755);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 27840);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 15870);
							match s {
								[] => true,
								['е', 'с', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 27841);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 30828);
													s.is_empty()
												}
												_ => false,
											}
										}
										['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
											super::push(out, 27842);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15871);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15872);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15873);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15874);
							match s {
								[] => true,
								['е', 'ф', 'о', s @ ..] => {
									super::push(out, 22756);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 27843);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 15875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8965);
					match s {
						[] => true,
						['о', 'п', 'м', 'о', 'к', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 15876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15877);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15878);
							match s {
								[] => true,
								['ф', 'ф', 'и', s @ ..] => {
									super::push(out, 22757);
									match s {
										[] => true,
										['д', 'н', 'и', '\u{1}', s @ ..] => {
											super::push(out, 27844);
											s.is_empty()
										}
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
					super::push(out, 8967);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							super::push(out, 15879);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15880);
							s.is_empty()
						}
						['о', 'п', 'м', s @ ..] => {
							super::push(out, 15881);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => super::t8::n9403(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3808);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8969);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 15884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t8::n3953(s, out),
		['э', s @ ..] => super::t8::n3954(s, out),
		['я', s @ ..] => super::t8::n3955(s, out),
		_ => false,
	}
}

/// Words ending -отный.
pub(crate) fn n1314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1282);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3812);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8971);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8972);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15886);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15887);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22767);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 27853);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 22768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8973);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8974);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 15888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3813);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 3814);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8975);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 8976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3815);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3816);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8977);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3817);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8978);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8979);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8980);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 8981);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15890);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15891);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 22769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8982);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 8983);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3818);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8984);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8985);
					match s {
						[] => true,
						['б', 'и', s @ ..] => {
							super::push(out, 15893);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22770);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15894);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15895);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8987);
					match s {
						[] => true,
						['и', 'к', 'о', 'н', s @ ..] => {
							super::push(out, 15897);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22771);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 27854);
											s.is_empty()
										}
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
			super::push(out, 3819);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8988);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15898);
							match s {
								[] => true,
								['г', 'у', s @ ..] => {
									super::push(out, 22772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 15899);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8989);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 15900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3820);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3821);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8991);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8992);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15901);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 15902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8993);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15903);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3822);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8995);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8996);
					s.is_empty()
				}
				['х', 'е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 15905);
							s.is_empty()
						}
						['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 15906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3823);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3824);
			match s {
				[] => true,
				['с', 'а', 'ч', 'о', 'и', s @ ..] => {
					super::push(out, 8997);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15907);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22774);
									s.is_empty()
								}
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
			super::push(out, 3825);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 3826);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8998);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15908);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8999);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9000);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15910);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15911);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22776);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27855);
											s.is_empty()
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 22777);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 22778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 9001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртный.
pub(crate) fn n1315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1283);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3827);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9002);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 9003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3828);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9004);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 9005);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 9006);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 9007);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15912);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9008);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 9009);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15914);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 9010);
					match s {
						[] => true,
						['н', 'о', 'к', 'о', s @ ..] => {
							super::push(out, 15915);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 22781);
									s.is_empty()
								}
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
			super::push(out, 3829);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9011);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15916);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22782);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 22783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 15917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9012);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 9013);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15918);
							match s {
								[] => true,
								['н', 'а', 'р', 'т', '-', s @ ..] => {
									super::push(out, 22784);
									match s {
										[] => true,
										['о', 'н', s @ ..] => {
											match s {
												[] => true,
												['ж', 'о', 'р', 'о', 'д', '\u{1}', s @ ..] => {
													super::push(out, 30832);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 30833);
													s.is_empty()
												}
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
					super::push(out, 9014);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9015);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9016);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 9017);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 15921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3830);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9018);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стный.
pub(crate) fn n1316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1284);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3831);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9019);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22785);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22786);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 22787);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9020);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15922);
							match s {
								[] => true,
								['л', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 22789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9021);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15923);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22790);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 27856);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9022);
					match s {
						[] => true,
						['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 15925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3832);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9023);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15926);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15927);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 22791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9024);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15928);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9025);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15930);
							s.is_empty()
						}
						['х', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 22793);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 22794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9026);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9027);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15931);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22795);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 22796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15932);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 22797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9028);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 9029);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15933);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 22798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15934);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15935);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 22799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3833);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9031);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 9032);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15937);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 22800);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27857);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 15938);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15939);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 22802);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 27858);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 15940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9033);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15941);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 9034);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15942);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t8::n3978(s, out),
		['р', s @ ..] => {
			super::push(out, 3835);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15973);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 22827);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27866);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 30834);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 27867);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 15974);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22828);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 27868);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 15975);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22829);
									s.is_empty()
								}
								['у', s @ ..] => super::t8::n23999(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t8::n3980(s, out),
		['ы', s @ ..] => super::t8::n3981(s, out),
		_ => false,
	}
}

/// Words ending -ттный.
pub(crate) fn n1317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1285);
	s.is_empty()
}

/// Words ending -утный.
pub(crate) fn n1318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1286);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3838);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15978);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 9049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3839);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 3840);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 3841);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3842);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9050);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9051);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3843);
			match s {
				[] => true,
				['и', 'м', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15981);
							match s {
								[] => true,
								['т', 'я', 'п', s @ ..] => {
									super::push(out, 22832);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 27870);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15982);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 22833);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27871);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 15983);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 30836);
													s.is_empty()
												}
												['а', 'к', 'о', 'р', 'о', 'с', '\u{1}', s @ ..] => {
													super::push(out, 30837);
													s.is_empty()
												}
												['и', 'т', 'а', 'ц', 'д', s @ ..] => {
													match s {
														[] => true,
														['а', 'в', 'д', '\u{1}', s @ ..] => super::t8::n33966(s, out),
														['и', 'р', 'т', '\u{1}', s @ ..] => super::t8::n33967(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ы', 'т', 'е', 'ч', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 30838);
													s.is_empty()
												}
												['а', 'к', 'о', 'р', 'о', 'с', '\u{1}', s @ ..] => {
													super::push(out, 30839);
													s.is_empty()
												}
												['и', 'т', 'а', 'ц', 'д', 'а', 'в', 'д', '\u{1}', s @ ..] => {
													super::push(out, 30840);
													s.is_empty()
												}
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
			super::push(out, 3844);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9053);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 15984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3845);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3846);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3847);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фтный.
pub(crate) fn n1319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1287);
	s.is_empty()
}

/// Words ending -хтный.
pub(crate) fn n1320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1288);
	s.is_empty()
}

/// Words ending -штный.
pub(crate) fn n1321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1289);
	match s {
		[] => true,
		['о', 'к', 'о', 'н', 'н', 'е', 'з', 'а', 'к', '\u{1}', s @ ..] => {
			super::push(out, 3848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытный.
pub(crate) fn n1322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1290);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3849);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3850);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3851);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15985);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 22835);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 15986);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22836);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15987);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 22837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 15988);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22838);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 27872);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 22839);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27873);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 22840);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 27874);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15989);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 22841);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22842);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 15990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3852);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3853);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9055);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9056);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтный.
pub(crate) fn n1323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1291);
	s.is_empty()
}

/// Words ending -этный.
pub(crate) fn n1324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1292);
	s.is_empty()
}

/// Words ending -ютный.
pub(crate) fn n1325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1293);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3854);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3855);
			match s {
				[] => true,
				['а', 'в', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22843);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 22844);
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 27875);
											s.is_empty()
										}
										['ь', s @ ..] => {
											super::push(out, 27876);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 15991);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22845);
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27877);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 27878);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 15992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9057);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3856);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9058);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9059);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', 'а', 'р', 'а', 'п', 'с', s @ ..] => {
			super::push(out, 3857);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9060);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятный.
pub(crate) fn n1326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1294);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3858);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3859);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9061);
					match s {
						[] => true,
						['п', 'о', 'т', s @ ..] => {
							super::push(out, 15995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9062);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3860);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3861);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3862);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9063);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 15997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3863);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word трехзначный.
pub(crate) fn n32366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30841);
	s.is_empty()
}

/// Words ending -новобрачный.
pub(crate) fn n32367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30842);
	s.is_empty()
}

/// Words ending -диопрозрачный.
pub(crate) fn n32368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30843);
	s.is_empty()
}

/// Words ending -ечный.
pub(crate) fn n1335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1303);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3882);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 3883);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3884);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9089);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 9090);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9091);
					match s {
						[] => true,
						['е', 'с', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22852);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27883);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'т', 'с', 'о', s @ ..] => {
									super::push(out, 22853);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27884);
											s.is_empty()
										}
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
			super::push(out, 3885);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9092);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['и', 'т', 'я', s @ ..] => {
									super::push(out, 22854);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 27885);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 22855);
									s.is_empty()
								}
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
			super::push(out, 3886);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9093);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16020);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9094);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9095);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22857);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 22858);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 27886);
											s.is_empty()
										}
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
					super::push(out, 9096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3887);
			match s {
				[] => true,
				['а', 'г', s @ ..] => {
					super::push(out, 9097);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9098);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9099);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9100);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3888);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3889);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9103);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9104);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 9105);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16025);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 16026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9106);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16027);
							match s {
								[] => true,
								['с', 'д', s @ ..] => {
									super::push(out, 22859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9107);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16028);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 22860);
									s.is_empty()
								}
								['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 22861);
									s.is_empty()
								}
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
			super::push(out, 3890);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9108);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 9109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16029);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22862);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22863);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 27887);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 22864);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 22865);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 22866);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 27888);
											s.is_empty()
										}
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
					super::push(out, 9110);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 9111);
					match s {
						[] => true,
						['с', 'у', s @ ..] => {
							super::push(out, 16030);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22867);
									s.is_empty()
								}
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
			super::push(out, 3891);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3892);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9112);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9113);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 16031);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 22868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 16032);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22869);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 16033);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 22871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9114);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 9115);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 16034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9116);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16035);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', 'о', '\u{1}', s @ ..] => {
							super::push(out, 16036);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 16037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t8::n4038(s, out),
		['т', s @ ..] => super::t8::n4039(s, out),
		['ч', s @ ..] => super::t8::n4040(s, out),
		['ш', s @ ..] => super::t8::n4041(s, out),
		['щ', s @ ..] => super::t8::n4042(s, out),
		_ => false,
	}
}

/// Words ending -ичный.
pub(crate) fn n1336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1304);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3898);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 3899);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9133);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16058);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 22892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 16059);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 16060);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 9134);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 16061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3900);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9135);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9136);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 9137);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9138);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 16062);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 22894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 16063);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 22895);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27892);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 22896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 16064);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 22897);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27893);
											s.is_empty()
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 22898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 16065);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 16066);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9139);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3901);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9140);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9141);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							super::push(out, 16068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3902);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9142);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16069);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9143);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 16070);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22901);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 22902);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27894);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 30844);
													match s {
														[] => true,
														['т', '\u{1}', s @ ..] => super::t8::n33968(s, out),
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
									super::push(out, 22903);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 27895);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 16071);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 16072);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 16073);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22904);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 27896);
											s.is_empty()
										}
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
			super::push(out, 3903);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 9144);
					match s {
						[] => true,
						['о', 'н', 'н', 'е', s @ ..] => {
							match s {
								[] => true,
								['л', 'е', 'д', 'е', 'р', 'п', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 27897);
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											super::push(out, 27898);
											s.is_empty()
										}
										_ => false,
									}
								}
								['щ', 'б', 'о', 'б', 'о', '\u{1}', s @ ..] => {
									super::push(out, 22905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 9145);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16074);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22906);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 16075);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 16076);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22908);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', 'у', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16077);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 16078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9146);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16079);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9147);
					s.is_empty()
				}
				['к', s @ ..] => super::t8::n9594(s, out),
				['л', s @ ..] => super::t8::n9595(s, out),
				['о', s @ ..] => super::t8::n9596(s, out),
				['с', s @ ..] => super::t8::n9597(s, out),
				['т', s @ ..] => super::t8::n9598(s, out),
				['у', s @ ..] => super::t8::n9599(s, out),
				['я', s @ ..] => super::t8::n9600(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t8::n4049(s, out),
		['н', s @ ..] => super::t8::n4050(s, out),
		['о', s @ ..] => super::t8::n4051(s, out),
		['п', s @ ..] => super::t8::n4052(s, out),
		['р', s @ ..] => super::t8::n4053(s, out),
		['с', s @ ..] => super::t8::n4054(s, out),
		['т', s @ ..] => super::t8::n4055(s, out),
		['ф', s @ ..] => super::t8::n4056(s, out),
		['х', s @ ..] => super::t8::n4057(s, out),
		['ч', s @ ..] => super::t8::n4058(s, out),
		['щ', s @ ..] => super::t8::n4059(s, out),
		['ь', s @ ..] => super::t8::n4060(s, out),
		_ => false,
	}
}

