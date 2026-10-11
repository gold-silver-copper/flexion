use flexion::Path;

/// Words ending -cal.
pub(crate) fn n893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 882);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3050);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 3051);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3052);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3053);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6233);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9031);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9032);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11701);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11702);
											s.is_empty()
										}
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
					super::push(out, 6234);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3054);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6235);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eal.
pub(crate) fn n894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 883);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6237);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9034);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9035);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10811);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6238);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nal.
pub(crate) fn n897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 886);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3057);
			match s {
				[] => true,
				['r', 'i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9036);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3058);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6240);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3059);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3060);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3061);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3062);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6242);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9038);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6243);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3063);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3064);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oal.
pub(crate) fn n898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 887);
	s.is_empty()
}

/// Words ending -pal.
pub(crate) fn n899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 888);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3065);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3066);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9040);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6245);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qal.
pub(crate) fn n900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 889);
	s.is_empty()
}

/// Words ending -ral.
pub(crate) fn n901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 890);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 3067);
			match s {
				[] => true,
				['i', 't', s @ ..] => {
					super::push(out, 6246);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10813);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10814);
									s.is_empty()
								}
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
			super::push(out, 3068);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6247);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sal.
pub(crate) fn n902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 891);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3069);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3070);
			match s {
				[] => true,
				['i', 'v', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9042);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9043);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 10815);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10816);
									s.is_empty()
								}
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
			super::push(out, 3071);
			match s {
				[] => true,
				['r', 'p', 'i', 'c', 'i', 'a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9044);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3072);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6248);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9046);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6249);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tal.
pub(crate) fn n903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 892);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3073);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6251);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6252);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6253);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3074);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9048);
							s.is_empty()
						}
						['o', 'r', 'n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 9049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 6255);
					s.is_empty()
				}
				['f', 'o', s @ ..] => {
					super::push(out, 6256);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6257);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6258);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6259);
					s.is_empty()
				}
				['r', 'o', 's', 't', 'r', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6260);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6261);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6262);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 10817);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10818);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10819);
									match s {
										[] => true,
										['s', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 11703);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10820);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 9050);
							match s {
								[] => true,
								['o', 'r', 'a', 'x', '\u{1}', s @ ..] => {
									super::push(out, 10821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3075);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6264);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9051);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9052);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10822);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3076);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 3077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ual.
pub(crate) fn n904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 893);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3078);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9053);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9054);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3079);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -val.
pub(crate) fn n905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 894);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6266);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 6267);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xal.
pub(crate) fn n906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 895);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
			super::push(out, 3081);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9056);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -el.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6268);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6269);
							match s {
								[] => true,
								['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9058);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 9059);
									s.is_empty()
								}
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
			super::push(out, 897);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6270);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6271);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 9060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3083);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6272);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6273);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10824);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 10825);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3085);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6275);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 898);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 899);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 900);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 901);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6277);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10826);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10827);
											s.is_empty()
										}
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
					super::push(out, 3087);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6278);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9061);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6279);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 9063);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 9064);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 9065);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9066);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6281);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6282);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6283);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 903);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 904);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 905);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3088);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 6285);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10828);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10829);
											s.is_empty()
										}
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
					super::push(out, 3089);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9067);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 9068);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 906);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 907);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3091);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6286);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 9069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6287);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10830);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10831);
											s.is_empty()
										}
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
					super::push(out, 3092);
					match s {
						[] => true,
						['c', 'o', 'l', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 6288);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6289);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -il.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3093);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 909);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3095);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6291);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 9070);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 9071);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9072);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6292);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10832);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10833);
											s.is_empty()
										}
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
					super::push(out, 3096);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6293);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6294);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 9074);
									match s {
										[] => true,
										['l', 'i', 's', s @ ..] => {
											super::push(out, 10834);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11704);
													match s {
														[] => true,
														['i', 'm', 'u', 's', '\u{1}', s @ ..] => super::t4::n14083(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 9075);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 10835);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 10836);
											match s {
												[] => true,
												['r', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3097);
					match s {
						[] => true,
						['d', 'i', 'n', 'o', 's', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10837);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10838);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 9076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3098);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9077);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 9078);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9079);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11705);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11706);
													s.is_empty()
												}
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
					super::push(out, 3099);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 910);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 3101);
					match s {
						[] => true,
						['c', 'e', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 6295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3102);
					match s {
						[] => true,
						['e', 'n', 'i', 'f', 'o', 'r', 'm', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 6296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 911);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3103);
					match s {
						[] => true,
						['m', 'e', 'n', 't', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6297);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6298);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3104);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6300);
							match s {
								[] => true,
								['u', s @ ..] => super::t4::n10083(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t4::n3214(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t4::n923(s, out),
		['m', s @ ..] => super::t4::n924(s, out),
		['n', s @ ..] => super::t4::n925(s, out),
		['p', s @ ..] => super::t4::n926(s, out),
		['q', s @ ..] => super::t4::n927(s, out),
		['r', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 917);
			s.is_empty()
		}
		['t', s @ ..] => super::t4::n929(s, out),
		['v', s @ ..] => super::t4::n930(s, out),
		['x', s @ ..] => super::t4::n931(s, out),
		_ => false,
	}
}

/// Words ending -ol.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 921);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3130);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6324);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6325);
							match s {
								[] => true,
								['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9095);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10851);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10852);
											s.is_empty()
										}
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
					super::push(out, 3131);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3132);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3133);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6326);
							match s {
								[] => true,
								['l', 'e', 't', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 11709);
													s.is_empty()
												}
												['t', 'u', 'r', s @ ..] => {
													super::push(out, 11710);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 's', s @ ..] => {
											super::push(out, 10853);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6327);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 10854);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 10855);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10856);
											s.is_empty()
										}
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
			super::push(out, 922);
			match s {
				[] => true,
				['g', 'i', 's', s @ ..] => {
					super::push(out, 3134);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
							super::push(out, 6329);
							match s {
								[] => true,
								['i', 'o', s @ ..] => {
									super::push(out, 9096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 924);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6331);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 925);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ul.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3138);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6333);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3139);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3140);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6335);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9097);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9098);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 10857);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3141);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6337);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9099);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6338);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6339);
							match s {
								[] => true,
								['o', 's', 'i', 's', s @ ..] => {
									super::push(out, 9101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3143);
					match s {
						[] => true,
						['v', 'i', 'c', 'i', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 930);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 931);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 932);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3144);
					match s {
						[] => true,
						['r', 'i', 'c', 'o', 'i', 'd', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 6341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 933);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6342);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6343);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 9102);
									s.is_empty()
								}
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
			super::push(out, 934);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6344);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6345);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10858);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10859);
											s.is_empty()
										}
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
			super::push(out, 935);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3146);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6346);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10860);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10861);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6347);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 9103);
									match s {
										[] => true,
										['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10862);
											s.is_empty()
										}
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
					super::push(out, 3147);
					match s {
						[] => true,
						['c', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6348);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 936);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3149);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3150);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9104);
									s.is_empty()
								}
								['i', 'v', s @ ..] => {
									super::push(out, 9105);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 937);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3152);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3153);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6350);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6351);
							match s {
								[] => true,
								['l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9106);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6352);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6353);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6354);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9107);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 938);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3156);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yl.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	s.is_empty()
}

/// The word sudnarecam.
pub(crate) fn n13522(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word snarecam.
pub(crate) fn n13523(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word surutarecam.
pub(crate) fn n13524(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word sutarecam.
pub(crate) fn n13525(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -tcam.
pub(crate) fn n3276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3162);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
			super::push(out, 6361);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10863);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ucam.
pub(crate) fn n3277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3163);
	s.is_empty()
}

/// Words ending -dam.
pub(crate) fn n952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 941);
	match s {
		[] => true,
		['e', 's', s @ ..] => {
			super::push(out, 3164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eam.
pub(crate) fn n953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 942);
	s.is_empty()
}

/// Words ending -gam.
pub(crate) fn n954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 943);
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6362);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9110);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6363);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iam.
pub(crate) fn n955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 944);
	match s {
		[] => true,
		['o', 'r', 'a', s @ ..] => {
			super::push(out, 3165);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3166);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lam.
pub(crate) fn n956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 945);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3167);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3168);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6364);
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10865);
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11711);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6365);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3169);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 6367);
					match s {
						[] => true,
						['a', 't', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6368);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mam.
pub(crate) fn n957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 946);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3171);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9113);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nam.
pub(crate) fn n958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 947);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3173);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6370);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3174);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3175);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6372);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9115);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11712);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11713);
											s.is_empty()
										}
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
					super::push(out, 6373);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9116);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6374);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 10867);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10868);
									s.is_empty()
								}
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
			super::push(out, 3176);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3177);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6375);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6376);
					match s {
						[] => true,
						['s', 't', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3178);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3179);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6377);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9119);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 9120);
							s.is_empty()
						}
						['u', 'l', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6378);
					match s {
						[] => true,
						['e', 's', 't', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12109);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12110);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 11714);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 11715);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11716);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 10869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6379);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6380);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3180);
			match s {
				[] => true,
				['u', 'e', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'e', 'n', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 9123);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 'd', s @ ..] => {
											super::push(out, 11717);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 11718);
											s.is_empty()
										}
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
			super::push(out, 3181);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6382);
					match s {
						[] => true,
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 6383);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6384);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6385);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oam.
pub(crate) fn n959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	match s {
		[] => true,
		['r', 'i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6387);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 6388);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ram.
pub(crate) fn n960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 949);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3182);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3183);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6389);
					match s {
						[] => true,
						['s', 'c', 'e', 'n', 's', s @ ..] => {
							super::push(out, 9126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6390);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3184);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3185);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6391);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6392);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6393);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6394);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9128);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9129);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3186);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3187);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6395);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3188);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6397);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6398);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 9131);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sam.
pub(crate) fn n961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 950);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3190);
			match s {
				[] => true,
				['i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tam.
pub(crate) fn n962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 951);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3192);
			match s {
				[] => true,
				['r', 'i', 'a', 'l', s @ ..] => {
					super::push(out, 6400);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10870);
									s.is_empty()
								}
								['t', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3193);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3194);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3195);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6401);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6402);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6403);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9133);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3196);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3197);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9135);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6405);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uam.
pub(crate) fn n963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 952);
	s.is_empty()
}

/// Words ending -xam.
pub(crate) fn n964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 953);
	match s {
		[] => true,
		['i', 'l', s @ ..] => {
			super::push(out, 3198);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -em.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 954);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3199);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3200);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 955);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 956);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6406);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9138);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9139);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11719);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11720);
													s.is_empty()
												}
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
							super::push(out, 6407);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9140);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 10871);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 9141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6408);
							match s {
								[] => true,
								['v', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 9142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6409);
							match s {
								[] => true,
								['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 9143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6410);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 9144);
									match s {
										[] => true,
										['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10872);
											s.is_empty()
										}
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
			super::push(out, 957);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 958);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3202);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3203);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6411);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6412);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9145);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6413);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10873);
											s.is_empty()
										}
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t4::n14088(s, out),
														['s', s @ ..] => super::t4::n14089(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 11721);
													match s {
														[] => true,
														['u', s @ ..] => super::t4::n14090(s, out),
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
							super::push(out, 6414);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3204);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6416);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 9146);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11722);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11723);
													s.is_empty()
												}
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
							super::push(out, 6417);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 9147);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9148);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 11724);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11725);
													s.is_empty()
												}
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
		['m', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3205);
					match s {
						[] => true,
						['r', 'a', 'n', s @ ..] => super::t4::n6874(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t4::n3325(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t4::n972(s, out),
		['r', s @ ..] => super::t4::n973(s, out),
		['s', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['t', s @ ..] => super::t4::n3337(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t4::n975(s, out),
		['x', s @ ..] => super::t4::n976(s, out),
		_ => false,
	}
}

/// Words ending -im.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 966);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3225);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3226);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6451);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6452);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 9160);
									s.is_empty()
								}
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
			super::push(out, 968);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3228);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6453);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3229);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 6455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 969);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 970);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3231);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6456);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6457);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 9161);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3232);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3233);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3235);
					match s {
						[] => true,
						['t', 'a', 'b', s @ ..] => {
							super::push(out, 6458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3236);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9162);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 9163);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 3237);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6459);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6460);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6461);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6462);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9164);
									s.is_empty()
								}
								['u', 's', s @ ..] => {
									super::push(out, 9165);
									s.is_empty()
								}
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
			super::push(out, 971);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3238);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6463);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6464);
							match s {
								[] => true,
								['d', 'i', s @ ..] => {
									super::push(out, 9166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9167);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 972);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6465);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9169);
									match s {
										[] => true,
										['i', 'l', 'i', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 11732);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11733);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n14091(s, out),
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
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'u', s @ ..] => super::t4::n10210(s, out),
								_ => false,
							}
						}
						['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t4::n984(s, out),
		['x', s @ ..] => super::t4::n985(s, out),
		_ => false,
	}
}

/// Words ending -om.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 975);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 976);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 977);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 978);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3244);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 9170);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3245);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3246);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6466);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6467);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9172);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10891);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3247);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6468);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6469);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 9174);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10892);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11734);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11735);
													s.is_empty()
												}
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
					super::push(out, 3248);
					match s {
						[] => true,
						['a', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 979);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 980);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3250);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3251);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6471);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3252);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3253);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3254);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6474);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3255);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3256);
					match s {
						[] => true,
						['a', 'n', 'i', s @ ..] => {
							super::push(out, 6476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 981);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3257);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6477);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6478);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9175);
									s.is_empty()
								}
								['u', 'r', s @ ..] => {
									super::push(out, 9176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 3258);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3259);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3260);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3261);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9177);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3262);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3263);
					match s {
						[] => true,
						['i', 'f', 'e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 982);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3265);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6480);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6481);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3266);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 984);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -um.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3268);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3269);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3270);
					match s {
						[] => true,
						['s', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 986);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 987);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3272);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6484);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10893);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10894);
											s.is_empty()
										}
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
					super::push(out, 3273);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6485);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6486);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3274);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6488);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9179);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9180);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10895);
											s.is_empty()
										}
										['u', 's', s @ ..] => {
											super::push(out, 10896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6489);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6490);
							match s {
								[] => true,
								['p', 'u', s @ ..] => {
									super::push(out, 9181);
									match s {
										[] => true,
										['n', 'c', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10897);
											s.is_empty()
										}
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
			super::push(out, 988);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3275);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6491);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10898);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10899);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3276);
					match s {
						[] => true,
						['r', 'a', 'b', s @ ..] => {
							super::push(out, 6493);
							match s {
								[] => true,
								['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3277);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 6494);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10900);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10901);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10902);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10903);
											s.is_empty()
										}
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
			super::push(out, 989);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3278);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3279);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6495);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6496);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9183);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 9184);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3280);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 990);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3282);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6498);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3283);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3284);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6500);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10904);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10905);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3285);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							super::push(out, 6502);
							match s {
								[] => true,
								['l', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 991);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3286);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6504);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6505);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 9186);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10906);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10907);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t4::n3411(s, out),
				['t', s @ ..] => super::t4::n3412(s, out),
				['u', s @ ..] => super::t4::n3413(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ym.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 992);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 993);
			match s {
				[] => true,
				['o', 'h', 'y', 'o', 'i', 'd', 'e', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 994);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 995);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 996);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -en.
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1009);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1010);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1011);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3304);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6525);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1012);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6527);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6528);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3306);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6530);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6531);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 9197);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3307);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3308);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3309);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 9198);
									match s {
										[] => true,
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11736);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11737);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 9199);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1015);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1016);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3311);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1017);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3313);
					match s {
						[] => true,
						['a', 'l', 'e', 'd', 'o', 'n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3314);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 1019);
			match s {
				[] => true,
				['u', 'i', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6533);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6534);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3316);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1021);
			match s {
				[] => true,
				['c', 'i', 't', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9200);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9201);
									s.is_empty()
								}
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
			super::push(out, 1022);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1023);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 1025);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -in.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3319);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3320);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3321);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9202);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9203);
									s.is_empty()
								}
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
			super::push(out, 1027);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3322);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3323);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3325);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
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
							super::push(out, 6536);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9204);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6537);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9206);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6538);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6539);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 9208);
									match s {
										[] => true,
										['u', 'n', 'c', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10912);
											s.is_empty()
										}
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
		['h', s @ ..] => {
			super::push(out, 1029);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3326);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3327);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3328);
					match s {
						[] => true,
						['t', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 6540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3329);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1033);
			s.is_empty()
		}
		['s', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1034);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3331);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 9209);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6541);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 3332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 1037);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6543);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6544);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3333);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -on.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6546);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9211);
									s.is_empty()
								}
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
			super::push(out, 1039);
			match s {
				[] => true,
				['t', 'u', s @ ..] => {
					super::push(out, 3334);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6548);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6549);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1041);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1042);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['i', 'n', 'a', 't', 'o', s @ ..] => {
					super::push(out, 3335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1044);
			s.is_empty()
		}
		['o', 't', 'k', 'a', 't', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 1045);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3336);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6551);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 9212);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9213);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6552);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10913);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10914);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'e', 'g', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3338);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3340);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6553);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6554);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9215);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3341);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6555);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3342);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3343);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3344);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3346);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 6558);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10915);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10916);
											s.is_empty()
										}
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
					super::push(out, 3347);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6559);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3348);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 1050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -un.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1051);
			match s {
				[] => true,
				['i', 'c', s @ ..] => {
					super::push(out, 3349);
					match s {
						[] => true,
						['o', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1052);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3350);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6562);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3351);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1054);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6564);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6565);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 9218);
									s.is_empty()
								}
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
			super::push(out, 1055);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1056);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3353);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6566);
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									match s {
										[] => true,
										['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10917);
											s.is_empty()
										}
										['o', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10918);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'p', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9219);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 9220);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10919);
											s.is_empty()
										}
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
					super::push(out, 3354);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3355);
					match s {
						[] => true,
						['i', 'a', 't', 'o', s @ ..] => {
							super::push(out, 6567);
							match s {
								[] => true,
								['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9221);
									s.is_empty()
								}
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
			super::push(out, 1057);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3356);
					match s {
						[] => true,
						['b', 'u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6568);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6569);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3357);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10920);
											match s {
												[] => true,
												['n', 's', s @ ..] => {
													super::push(out, 11738);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 11739);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 10921);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 9222);
									s.is_empty()
								}
								['m', 'e', 'n', 't', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 9223);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10922);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 10923);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11740);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11741);
													s.is_empty()
												}
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

/// Words ending -ebo.
pub(crate) fn n1074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1063);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3368);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9232);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 3369);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3370);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3371);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3372);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sufbo.
pub(crate) fn n3507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3373);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9234);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9235);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ibo.
pub(crate) fn n1076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1065);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3374);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3375);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3376);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3377);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3378);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3379);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lbo.
pub(crate) fn n1077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1066);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3380);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9236);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9237);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10926);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10927);
									s.is_empty()
								}
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
			super::push(out, 3381);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3382);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6577);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6578);
					match s {
						[] => true,
						['a', 't', 'o', s @ ..] => {
							super::push(out, 9238);
							match s {
								[] => true,
								['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6579);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6580);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 6581);
					s.is_empty()
				}
				['s', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9239);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6582);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							super::push(out, 9241);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6583);
					match s {
						[] => true,
						['i', 's', 'c', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 9242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nbo.
pub(crate) fn n1079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1068);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3384);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6584);
					s.is_empty()
				}
				['x', 'u', 's', s @ ..] => {
					super::push(out, 6585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3385);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3386);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rbo.
pub(crate) fn n1082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1071);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3387);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3388);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3389);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sbo.
pub(crate) fn n1083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1072);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3390);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3391);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6586);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10930);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6587);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11742);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11743);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'o', s @ ..] => {
									super::push(out, 10932);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10933);
									s.is_empty()
								}
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
			super::push(out, 3392);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6588);
					match s {
						[] => true,
						['r', 'i', 'c', 'a', 't', 'u', 'r', s @ ..] => {
							super::push(out, 9243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3393);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3394);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6589);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6590);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 9244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3395);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6592);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9245);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6593);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 9247);
							match s {
								[] => true,
								['c', 't', 'u', 'r', s @ ..] => {
									super::push(out, 10934);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10935);
									s.is_empty()
								}
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

/// Words ending -tbo.
pub(crate) fn n1084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1073);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3396);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6594);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3397);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3398);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3399);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6596);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6597);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6598);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10936);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6599);
					match s {
						[] => true,
						['d', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 9248);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9249);
							s.is_empty()
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 9250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3400);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6600);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9251);
							s.is_empty()
						}
						['s', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9253);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 9254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9255);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9256);
							match s {
								[] => true,
								['f', 'o', 'l', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 9257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ubo.
pub(crate) fn n1085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1074);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 3401);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vbo.
pub(crate) fn n1086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1075);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3403);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3404);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3405);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 6601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -co.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1076);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3406);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6602);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6603);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10939);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10940);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6604);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10941);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10942);
											s.is_empty()
										}
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
					super::push(out, 3407);
					match s {
						[] => true,
						['d', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 9258);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6605);
							s.is_empty()
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 6606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3408);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['d', 'e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10943);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10944);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10945);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10946);
											s.is_empty()
										}
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
					super::push(out, 3409);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6607);
							match s {
								[] => true,
								['a', 't', 'i', s @ ..] => {
									super::push(out, 9260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'a', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1078);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1079);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1080);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3411);
					match s {
						[] => true,
						['v', 'a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3412);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6609);
							match s {
								[] => true,
								['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'a', 'r', 'i', 'n', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6610);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6611);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6612);
							s.is_empty()
						}
						['p', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6613);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -do.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1082);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3413);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3414);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3415);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3416);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9262);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1083);
			match s {
				[] => true,
				['r', 'a', 't', 'u', 'r', s @ ..] => {
					super::push(out, 3418);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eo.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['c', 'u', s @ ..] => {
			super::push(out, 1084);
			match s {
				[] => true,
				['m', 'e', 'n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3419);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fo.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3420);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6614);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6615);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 9264);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3421);
					match s {
						[] => true,
						['c', 'i', 'o', s @ ..] => {
							super::push(out, 6617);
							match s {
								[] => true,
								['s', 'i', 's', s @ ..] => {
									super::push(out, 9266);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3422);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3423);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 6618);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10947);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10948);
											s.is_empty()
										}
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

/// Words ending -lo.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1085);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3424);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6619);
							s.is_empty()
						}
						['r', 'i', 's', s @ ..] => {
							super::push(out, 6620);
							s.is_empty()
						}
						['t', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3425);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3426);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1086);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1087);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3428);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3429);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3430);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3431);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1088);
			match s {
				[] => true,
				['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1089);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3434);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3435);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1090);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3436);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9267);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9268);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11744);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11745);
													s.is_empty()
												}
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
					super::push(out, 3437);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1092);
			match s {
				[] => true,
				['i', 'p', s @ ..] => {
					super::push(out, 3439);
					match s {
						[] => true,
						['o', 't', 'e', 'n', 't', s @ ..] => {
							super::push(out, 6622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3440);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -no.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['e', 'r', 'a', 't', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6623);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1095);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', 'f', 'r', 'o', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 6625);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1096);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6627);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9269);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9270);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 10949);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6628);
							match s {
								[] => true,
								['s', 'i', 's', s @ ..] => {
									super::push(out, 9271);
									s.is_empty()
								}
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
			super::push(out, 1097);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3442);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3443);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3444);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3445);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3446);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9272);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9273);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3447);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1098);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6629);
							s.is_empty()
						}
						['u', 'n', 'i', 's', s @ ..] => {
							super::push(out, 6630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1099);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3449);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 9275);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3450);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3451);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6631);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3452);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9277);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3453);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6633);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6634);
							match s {
								[] => true,
								['t', 'u', 'n', 'i', 's', s @ ..] => {
									super::push(out, 9279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3454);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1100);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1101);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3456);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6636);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6637);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9280);
									s.is_empty()
								}
								['u', 's', s @ ..] => {
									super::push(out, 9281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3457);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1103);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3459);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1104);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 3461);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9282);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9283);
									s.is_empty()
								}
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
			super::push(out, 1105);
			match s {
				[] => true,
				['i', 'n', 'i', s @ ..] => {
					super::push(out, 3462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1106);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6639);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6640);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10950);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10951);
											s.is_empty()
										}
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
			super::push(out, 1107);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1108);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1109);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3463);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3464);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3465);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1110);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6641);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6642);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9284);
									s.is_empty()
								}
								['u', 's', s @ ..] => {
									super::push(out, 9285);
									s.is_empty()
								}
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
			super::push(out, 1111);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1112);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1113);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3467);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['i', 't', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6643);
							match s {
								[] => true,
								['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6644);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10952);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10953);
											s.is_empty()
										}
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
					super::push(out, 3469);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3470);
					match s {
						[] => true,
						['l', 'a', 'b', s @ ..] => {
							super::push(out, 6645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1116);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1117);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3471);
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 6646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3472);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6647);
							s.is_empty()
						}
						['l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 6648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1118);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3474);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3475);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1119);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3476);
					s.is_empty()
				}
				['o', 's', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6649);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vo.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1121);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3477);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 6651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3478);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1122);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3480);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9287);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3481);
					match s {
						[] => true,
						['l', 'i', s @ ..] => {
							super::push(out, 6652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3482);
					s.is_empty()
				}
				['p', 'a', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3483);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xo.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['i', 'a', s @ ..] => {
			super::push(out, 1123);
			match s {
				[] => true,
				['n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1124);
			match s {
				[] => true,
				['n', 'i', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1125);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tacap.
pub(crate) fn n7160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6657);
	match s {
		[] => true,
		['u', 's', s @ ..] => {
			super::push(out, 9289);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sullyhpyhcap.
pub(crate) fn n3630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3488);
	s.is_empty()
}

/// Words ending -icap.
pub(crate) fn n3631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3489);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 6658);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tcap.
pub(crate) fn n3632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3490);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9290);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9291);
					s.is_empty()
				}
				['t', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 9292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eap.
pub(crate) fn n1140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1128);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3491);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6660);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9293);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11746);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11747);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 9294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3492);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9295);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3493);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3494);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gap.
pub(crate) fn n1141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1129);
	match s {
		[] => true,
		['a', 'n', 'i', s @ ..] => {
			super::push(out, 3495);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3496);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kap.
pub(crate) fn n1142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1130);
	s.is_empty()
}

/// Words ending -lap.
pub(crate) fn n1143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1131);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3497);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6661);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6662);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6663);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6664);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6665);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9297);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9298);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10954);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3498);
			match s {
				[] => true,
				['a', 'r', 'i', 'c', s @ ..] => {
					super::push(out, 6667);
					match s {
						[] => true,
						['i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6668);
					match s {
						[] => true,
						['t', 'i', 'n', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 9300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3499);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3500);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6669);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9301);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3501);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6671);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9303);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11748);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11749);
											s.is_empty()
										}
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
					super::push(out, 6672);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3502);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 9304);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9305);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11750);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11751);
											s.is_empty()
										}
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
			super::push(out, 3503);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6674);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6675);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -map.
pub(crate) fn n1144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1132);
	s.is_empty()
}

/// Words ending -nap.
pub(crate) fn n1145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1133);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3504);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6676);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3505);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6678);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9306);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3506);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3507);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3508);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 6679);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pap.
pub(crate) fn n1146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1134);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3510);
			match s {
				[] => true,
				['l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3511);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3512);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6682);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6683);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6684);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3513);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rap.
pub(crate) fn n1147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1135);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3515);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9308);
							match s {
								[] => true,
								['l', 'i', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 9309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6686);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6687);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6688);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6689);
					match s {
						[] => true,
						['d', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6690);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9311);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['b', 'e', 'r', 'c', 'u', 'l', 'o', 's', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 10956);
									s.is_empty()
								}
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10957);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 3516);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3517);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9312);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9313);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 9314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3518);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6691);
					match s {
						[] => true,
						['u', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'i', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6692);
					match s {
						[] => true,
						['u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'u', s @ ..] => {
			match s {
				[] => true,
				['r', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3519);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'e', 'p', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 9316);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11752);
											match s {
												[] => true,
												['u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 11753);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					match s {
						[] => true,
						['r', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 3520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sap.
pub(crate) fn n1148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1136);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3521);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6693);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6694);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6695);
					s.is_empty()
				}
				['u', 'a', s @ ..] => {
					super::push(out, 6696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3522);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3523);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3524);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6697);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6698);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6699);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10958);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 11754);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11755);
											s.is_empty()
										}
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
					super::push(out, 6700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tap.
pub(crate) fn n1149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1137);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3525);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3526);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3527);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3528);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6703);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9317);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6704);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6705);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6706);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uap.
pub(crate) fn n1150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1138);
	match s {
		[] => true,
		['p', 'e', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 't', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'o', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3529);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9319);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vap.
pub(crate) fn n1151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1139);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3530);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 6707);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6708);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3531);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6710);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6711);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6712);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6713);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9321);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3532);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9323);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ep.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1140);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3533);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 9325);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9326);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3534);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3535);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6714);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6715);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6716);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6717);
							match s {
								[] => true,
								['i', 'o', 's', 'i', 's', s @ ..] => {
									super::push(out, 9328);
									s.is_empty()
								}
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
			super::push(out, 1141);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3536);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6718);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6719);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3537);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6721);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3538);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9329);
									s.is_empty()
								}
								['u', 'l', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10959);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 10960);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6723);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9330);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9331);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3539);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6725);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3540);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3541);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9332);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 9333);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 3542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1143);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'c', 't', 'u', 'r', s @ ..] => {
							super::push(out, 6727);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6728);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', 's', '\u{1}', s @ ..] => {
											super::push(out, 10961);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 10962);
											s.is_empty()
										}
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
			super::push(out, 1144);
			match s {
				[] => true,
				['b', 'a', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3543);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1145);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3545);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6729);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6730);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3546);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3547);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3548);
					match s {
						[] => true,
						['c', 'i', 'l', 'l', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6732);
							s.is_empty()
						}
						['f', 'o', 'r', 'm', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 6733);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6734);
							match s {
								[] => true,
								['s', 'u', 'l', 'a', 'r', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 9334);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', s @ ..] => {
							super::push(out, 6735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3549);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3550);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6736);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9335);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3551);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6738);
							s.is_empty()
						}
						['l', 't', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t4::n1158(s, out),
		['s', s @ ..] => {
			super::push(out, 1147);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1148);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3572);
					match s {
						[] => true,
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hp.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1149);
			match s {
				[] => true,
				['r', 'o', s @ ..] => {
					super::push(out, 3573);
					match s {
						[] => true,
						['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 6822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1150);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3574);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1151);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3576);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1152);
			match s {
				[] => true,
				['s', 'o', s @ ..] => {
					super::push(out, 3578);
					match s {
						[] => true,
						['d', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 6823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ip.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3579);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1154);
			match s {
				[] => true,
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 3581);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6824);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1155);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1156);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3582);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3583);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3584);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3585);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9430);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9431);
									s.is_empty()
								}
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
			super::push(out, 1157);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1158);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1159);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6826);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6827);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6828);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11004);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11005);
											s.is_empty()
										}
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
					super::push(out, 3586);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3587);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1160);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1161);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3589);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6829);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3590);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3591);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9432);
									match s {
										[] => true,
										['s', 'c', 'e', 'n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11817);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11818);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3592);
					match s {
						[] => true,
						['c', 'o', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3593);
					match s {
						[] => true,
						['a', 't', 'i', s @ ..] => {
							super::push(out, 6832);
							match s {
								[] => true,
								['f', 'i', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3594);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6833);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11006);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11007);
											s.is_empty()
										}
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
			super::push(out, 1162);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3595);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3596);
					match s {
						[] => true,
						['a', 't', 'u', 'r', s @ ..] => {
							super::push(out, 6834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1163);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1164);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3598);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6835);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9434);
									s.is_empty()
								}
								['r', 'i', 'c', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9435);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9436);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 11008);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6836);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6837);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9437);
									s.is_empty()
								}
								['v', 'o', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6838);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3599);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3600);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6840);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9439);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6841);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9441);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9442);
									s.is_empty()
								}
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
			super::push(out, 1165);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1166);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lp.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1167);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3601);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6842);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9443);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9444);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11009);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11010);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6843);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3602);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6845);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6846);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9445);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 9446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3603);
					match s {
						[] => true,
						['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6848);
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 9447);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 6849);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6850);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 9449);
									match s {
										[] => true,
										['r', 'o', 'n', 's', '\u{1}', s @ ..] => {
											super::push(out, 11011);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6851);
							match s {
								[] => true,
								['a', 'g', s @ ..] => {
									super::push(out, 9450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3604);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3605);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1168);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3607);
					s.is_empty()
				}
				['c', 't', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9451);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6855);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6856);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6857);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6858);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3608);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3609);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3610);
					s.is_empty()
				}
				['u', 'r', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3611);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 3612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1169);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6860);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6861);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 9453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3613);
					match s {
						[] => true,
						['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1170);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3614);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3615);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9454);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9455);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1171);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3617);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3618);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6863);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9457);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9458);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 11012);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 6864);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6865);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 9459);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6866);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 9461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3619);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6868);
							s.is_empty()
						}
						['c', 'u', 'l', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3621);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -np.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	s.is_empty()
}

/// Words ending -op.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1172);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1173);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3622);
					match s {
						[] => true,
						['i', 't', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1174);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3624);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6870);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6871);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6872);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9462);
									s.is_empty()
								}
								['u', 's', s @ ..] => {
									super::push(out, 9463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3625);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6873);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6874);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 9464);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9465);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 11013);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 11014);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6875);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11819);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11820);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 11015);
											s.is_empty()
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11821);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11822);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 9466);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11016);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11017);
											s.is_empty()
										}
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
					super::push(out, 3626);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3627);
					match s {
						[] => true,
						['l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1175);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3629);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3630);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3631);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3632);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6877);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9467);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11018);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11019);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1176);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3633);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3634);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3635);
					s.is_empty()
				}
				['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1177);
			match s {
				[] => true,
				['u', 'l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6879);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1178);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3637);
					match s {
						[] => true,
						['a', 'r', 'i', 'c', s @ ..] => {
							super::push(out, 6881);
							match s {
								[] => true,
								['i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3638);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9470);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3639);
					match s {
						[] => true,
						['e', 'c', 't', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9472);
									s.is_empty()
								}
								['u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3640);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6883);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9474);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9475);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1179);
			match s {
				[] => true,
				['t', s @ ..] => super::t4::n3793(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t4::n1192(s, out),
		_ => false,
	}
}

/// Words ending -rp.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1181);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3646);
					s.is_empty()
				}
				['e', s @ ..] => super::t4::n3800(s, out),
				['g', s @ ..] => {
					super::push(out, 3648);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3649);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6912);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3650);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n1194(s, out),
		['i', s @ ..] => super::t4::n1195(s, out),
		['o', s @ ..] => super::t4::n1196(s, out),
		['u', s @ ..] => {
			super::push(out, 1185);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3683);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3684);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3685);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sp.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['y', s @ ..] => {
			super::push(out, 1186);
			match s {
				[] => true,
				['c', 'h', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6975);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tp.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	s.is_empty()
}

/// Words ending -up.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1187);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3686);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 9618);
									match s {
										[] => true,
										['e', 'n', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1188);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3688);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6977);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3689);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6979);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9619);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9620);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1189);
			match s {
				[] => true,
				['r', 'i', 'l', 'i', 's', s @ ..] => {
					super::push(out, 3691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6980);
							match s {
								[] => true,
								['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1190);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3692);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							super::push(out, 6981);
							match s {
								[] => true,
								['t', 'o', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9621);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'a', s @ ..] => {
					super::push(out, 3693);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6982);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6983);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 9622);
									s.is_empty()
								}
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
			super::push(out, 1191);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3694);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3695);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3696);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6984);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3697);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3698);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6986);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11120);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11121);
											s.is_empty()
										}
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
					super::push(out, 3699);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6987);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1192);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1193);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3700);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3701);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3702);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6989);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6990);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6991);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6992);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11122);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11123);
											s.is_empty()
										}
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
			super::push(out, 1194);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1195);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3703);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9623);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9624);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 11124);
											s.is_empty()
										}
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
					super::push(out, 3704);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3705);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1196);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3707);
					match s {
						[] => true,
						['l', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1197);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3708);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6994);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3709);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6996);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6997);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3710);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6999);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3711);
					match s {
						[] => true,
						['e', s @ ..] => super::t4::n7552(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yp.
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1198);
			match s {
				[] => true,
				['i', 'd', 'i', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1199);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3713);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7001);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3714);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3715);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1200);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7003);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7004);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -edirdauq.
pub(crate) fn n13756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11888);
	s.is_empty()
}

/// Words ending -udirdauq.
pub(crate) fn n13757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11889);
	s.is_empty()
}

/// Words ending -ragirdauq.
pub(crate) fn n13758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11890);
	s.is_empty()
}

/// Words ending -iuq.
pub(crate) fn n1215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1203);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3727);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7011);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9634);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 9635);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9636);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9637);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9638);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 9639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3728);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7012);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9640);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 7013);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							super::push(out, 9642);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 11133);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11134);
									s.is_empty()
								}
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
			super::push(out, 3729);
			match s {
				[] => true,
				['i', 'n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9643);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3730);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3731);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ouq.
pub(crate) fn n1216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1204);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7014);
					match s {
						[] => true,
						['d', 'i', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'u', 'm', 'q', 'u', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -er.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1218);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3750);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3751);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3752);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9648);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1219);
			match s {
				[] => true,
				['o', 'a', 't', s @ ..] => {
					super::push(out, 3754);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9650);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9651);
									s.is_empty()
								}
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
			super::push(out, 1220);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3755);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7023);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7024);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11138);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11139);
											s.is_empty()
										}
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
					super::push(out, 3756);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7025);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 9652);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11140);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 11141);
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', 'u', 's', '\u{1}', s @ ..] => super::t4::n14167(s, out),
														['s', '\u{1}', s @ ..] => super::t4::n14168(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['i', 't', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11142);
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 9653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 7026);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3757);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7028);
							match s {
								[] => true,
								['e', 'n', 'd', s @ ..] => {
									super::push(out, 9654);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 9655);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 7029);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7030);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11143);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11144);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3758);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7032);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', s @ ..] => {
											super::push(out, 11145);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11891);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11892);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 11146);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7033);
							match s {
								[] => true,
								['n', s @ ..] => super::t4::n10800(s, out),
								_ => false,
							}
						}
						['u', s @ ..] => super::t4::n7597(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t4::n3916(s, out),
				['r', s @ ..] => super::t4::n3917(s, out),
				['t', s @ ..] => {
					super::push(out, 3761);
					s.is_empty()
				}
				['u', s @ ..] => super::t4::n3919(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t4::n1233(s, out),
		['e', s @ ..] => {
			super::push(out, 1222);
			s.is_empty()
		}
		['f', s @ ..] => super::t4::n1235(s, out),
		['g', s @ ..] => super::t4::n1236(s, out),
		['i', s @ ..] => super::t4::n1237(s, out),
		['l', s @ ..] => super::t4::n1238(s, out),
		['m', s @ ..] => super::t4::n1239(s, out),
		['n', s @ ..] => super::t4::n1240(s, out),
		['p', s @ ..] => super::t4::n1241(s, out),
		['q', s @ ..] => super::t4::n1242(s, out),
		['s', s @ ..] => super::t4::n1243(s, out),
		['t', s @ ..] => super::t4::n1244(s, out),
		['u', s @ ..] => super::t4::n1245(s, out),
		['v', s @ ..] => super::t4::n1246(s, out),
		_ => false,
	}
}

/// The word sisnegir.
pub(crate) fn n13789(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -amir.
pub(crate) fn n3996(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 7167);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7168);
			s.is_empty()
		}
		['t', 'u', 'r', s @ ..] => {
			super::push(out, 7169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pir.
pub(crate) fn n1254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1242);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3838);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3839);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sir.
pub(crate) fn n1255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1243);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3840);
			match s {
				[] => true,
				['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7170);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vir.
pub(crate) fn n1257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1245);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'i', 'c', s @ ..] => {
					super::push(out, 7171);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7172);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 7173);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xir.
pub(crate) fn n1258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1246);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7174);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7175);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11191);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11192);
									s.is_empty()
								}
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

/// Words ending -or.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1247);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3841);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3842);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3843);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3844);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1248);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1249);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1250);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 3845);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9732);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1251);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7177);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1252);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7179);
							match s {
								[] => true,
								['m', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 9734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7180);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11193);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11194);
											s.is_empty()
										}
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
					super::push(out, 3847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1253);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3848);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7181);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7182);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7183);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3849);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3850);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3851);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3852);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1254);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3853);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7186);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3854);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7188);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3855);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1255);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3856);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3857);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7191);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7192);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'o', 'r', 'm', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 7193);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7194);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3858);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 9736);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 11912);
													match s {
														[] => true,
														['d', s @ ..] => super::t4::n14177(s, out),
														['s', s @ ..] => super::t4::n14178(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 11913);
													match s {
														[] => true,
														['u', s @ ..] => super::t4::n14179(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['e', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 11195);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 11196);
											match s {
												[] => true,
												['s', 't', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 11914);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 11197);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 9737);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 9738);
									s.is_empty()
								}
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
			super::push(out, 1256);
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7196);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7197);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9739);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9740);
									s.is_empty()
								}
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
			super::push(out, 1257);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3859);
					match s {
						[] => true,
						['r', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 7198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7199);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7200);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11198);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11199);
											s.is_empty()
										}
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
			super::push(out, 1258);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1259);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3860);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'o', 'r', s @ ..] => {
							super::push(out, 7201);
							match s {
								[] => true,
								['n', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 9741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'e', 's', s @ ..] => {
							super::push(out, 7202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1260);
			match s {
				[] => true,
				['i', 'c', s @ ..] => {
					super::push(out, 3862);
					match s {
						[] => true,
						['o', 'l', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 7203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1261);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1262);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1263);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3864);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1264);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7204);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1265);
			match s {
				[] => true,
				['e', 's', 't', 'r', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3866);
					s.is_empty()
				}
				['i', 'c', 'o', 'l', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1266);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1267);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3868);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1268);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3870);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3871);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ilas.
pub(crate) fn n4048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3880);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7218);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7219);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7220);
			s.is_empty()
		}
		['t', 'u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9747);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 7221);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -llas.
pub(crate) fn n4049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3881);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9749);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 9750);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olas.
pub(crate) fn n4050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3882);
	s.is_empty()
}

/// Words ending -slas.
pub(crate) fn n4051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3883);
	s.is_empty()
}

/// Words ending -tlas.
pub(crate) fn n4052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3884);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', 'u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 9751);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9752);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9753);
					match s {
						[] => true,
						['u', 's', s @ ..] => {
							super::push(out, 11200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7222);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulas.
pub(crate) fn n4053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3885);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7223);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11201);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11202);
							match s {
								[] => true,
								['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7224);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11203);
							match s {
								[] => true,
								['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 11915);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'o', s @ ..] => {
							super::push(out, 11204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vlas.
pub(crate) fn n4054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3886);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7225);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9754);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 9755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mas.
pub(crate) fn n1287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1275);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3887);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7227);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nas.
pub(crate) fn n1288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1276);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3888);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7228);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7229);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['o', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9756);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11205);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11206);
									s.is_empty()
								}
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
			super::push(out, 3889);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7230);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3890);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3891);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7232);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3892);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pas.
pub(crate) fn n1289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1277);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3894);
			match s {
				[] => true,
				['r', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7234);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ras.
pub(crate) fn n1290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1278);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3895);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3896);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7235);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 7236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3897);
			match s {
				[] => true,
				['i', 't', 'u', 'r', s @ ..] => {
					super::push(out, 7237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tas.
pub(crate) fn n1291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1279);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3898);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7238);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9757);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9758);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3899);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 9760);
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9761);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uas.
pub(crate) fn n1292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1280);
	s.is_empty()
}

/// Words ending -vas.
pub(crate) fn n1293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1281);
	match s {
		[] => true,
		['i', 'a', 't', 'u', 'r', s @ ..] => {
			super::push(out, 3900);
			match s {
				[] => true,
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7240);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xas.
pub(crate) fn n1294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1282);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3901);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3902);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cs.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1283);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3903);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7241);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3904);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7243);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3905);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7245);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1284);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3906);
					match s {
						[] => true,
						['e', 'r', 'a', 't', 'i', s @ ..] => {
							super::push(out, 7247);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 9762);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3907);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1285);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1286);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3909);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3910);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3911);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7248);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3912);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7250);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3913);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 7252);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11207);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11208);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1287);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1288);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3915);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7254);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9764);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7255);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3916);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1289);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3918);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3919);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3920);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1290);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3922);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3923);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -es.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1291);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3924);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3925);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7258);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7259);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3926);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3927);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3928);
					match s {
						[] => true,
						['e', 't', 'u', 'r', s @ ..] => {
							super::push(out, 7261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3929);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 7262);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11209);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11210);
											s.is_empty()
										}
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
					super::push(out, 3930);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7263);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7264);
							s.is_empty()
						}
						['p', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7265);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7266);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9766);
									match s {
										[] => true,
										['g', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 11211);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 9767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7267);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 11212);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 11213);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11214);
											s.is_empty()
										}
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
		['d', s @ ..] => {
			super::push(out, 1292);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3931);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7268);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9768);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11215);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11216);
											s.is_empty()
										}
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
					super::push(out, 3932);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7269);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3933);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7271);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7272);
							match s {
								[] => true,
								['i', 'o', 's', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11217);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11218);
											s.is_empty()
										}
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
					super::push(out, 3934);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9769);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9770);
									s.is_empty()
								}
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
			super::push(out, 1293);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1294);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3935);
					match s {
						[] => true,
						['e', 'r', 'i', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7274);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 9771);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 9772);
									s.is_empty()
								}
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
			super::push(out, 1297);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3936);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3937);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3939);
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 7275);
							s.is_empty()
						}
						['s', 't', 'r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 7276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3940);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7277);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9773);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 9774);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 11219);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 9775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7278);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7279);
							match s {
								[] => true,
								['u', 'n', s @ ..] => {
									super::push(out, 9776);
									match s {
										[] => true,
										['c', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11220);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7280);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3941);
					match s {
						[] => true,
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7281);
							s.is_empty()
						}
						['v', s @ ..] => super::t4::n7872(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t4::n4114(s, out),
				['u', s @ ..] => super::t4::n4115(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t4::n1311(s, out),
		['o', s @ ..] => super::t4::n1312(s, out),
		['p', s @ ..] => super::t4::n1313(s, out),
		['q', s @ ..] => super::t4::n1314(s, out),
		['r', s @ ..] => super::t4::n1315(s, out),
		['s', s @ ..] => super::t4::n1316(s, out),
		['t', s @ ..] => super::t4::n1317(s, out),
		['u', s @ ..] => super::t4::n1318(s, out),
		['v', s @ ..] => super::t4::n1319(s, out),
		['x', s @ ..] => super::t4::n1320(s, out),
		['y', 'c', 'h', 'e', 'l', 'l', s @ ..] => super::t4::n1321(s, out),
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1309);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3980);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7303);
							match s {
								[] => true,
								['i', 'c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1310);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7304);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9798);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['e', 'r', 'a', 't', s @ ..] => {
					super::push(out, 3982);
					match s {
						[] => true,
						['i', 'c', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1312);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3983);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3984);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3985);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 7307);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11228);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11229);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1313);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3986);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7309);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7310);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 9801);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 9802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3987);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7311);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9803);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9804);
									s.is_empty()
								}
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
			super::push(out, 1314);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3988);
					match s {
						[] => true,
						['l', 'a', 'g', s @ ..] => {
							super::push(out, 7312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3989);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3990);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7313);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3991);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3992);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 9805);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 11230);
											match s {
												[] => true,
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
					super::push(out, 3993);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3994);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7315);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3995);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 7317);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11231);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11232);
											s.is_empty()
										}
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
			super::push(out, 1316);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3996);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1318);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3998);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3999);
					match s {
						[] => true,
						['b', 'u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7318);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 7319);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7320);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4000);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 7322);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9806);
									s.is_empty()
								}
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
			super::push(out, 1319);
			match s {
				[] => true,
				['c', 'e', 'u', 'v', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 4001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ls.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['o', 'v', 'a', 'c', 'i', s @ ..] => {
			super::push(out, 1320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ms.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1321);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -os.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1322);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4002);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9807);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9808);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4003);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1324);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4005);
					match s {
						[] => true,
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4006);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7324);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4007);
					match s {
						[] => true,
						['c', 'i', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7326);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 7327);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7328);
							s.is_empty()
						}
						['s', 'e', 'q', 'u', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7329);
							s.is_empty()
						}
						['t', 'a', s @ ..] => {
							super::push(out, 7330);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4008);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4009);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4010);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4011);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7332);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7333);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7334);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9810);
									s.is_empty()
								}
								['u', 'r', s @ ..] => {
									super::push(out, 9811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1326);
			match s {
				[] => true,
				['n', 'i', 'c', s @ ..] => {
					super::push(out, 4013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4014);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7335);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4015);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 7337);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1328);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4017);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4018);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7339);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4019);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9812);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9813);
									s.is_empty()
								}
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
			super::push(out, 1329);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4020);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4021);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7341);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9814);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9815);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4022);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4023);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4024);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							super::push(out, 7343);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11233);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11234);
											s.is_empty()
										}
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

/// Words ending -ps.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1330);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4025);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7344);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4026);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1331);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4028);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7346);
							match s {
								[] => true,
								['o', 's', 'i', s @ ..] => {
									super::push(out, 9816);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 11235);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7347);
							match s {
								[] => true,
								['a', 't', 'i', s @ ..] => {
									super::push(out, 9817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4029);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7349);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9818);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9819);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7350);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1332);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4030);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4032);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7352);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7353);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4033);
					match s {
						[] => true,
						['i', 'p', s @ ..] => {
							super::push(out, 7355);
							match s {
								[] => true,
								['e', 'n', 'n', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 9820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4034);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9821);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9822);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1334);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7357);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7358);
							match s {
								[] => true,
								['t', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9824);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'u', s @ ..] => {
							super::push(out, 7359);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9825);
									s.is_empty()
								}
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
			super::push(out, 1335);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4036);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4037);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7360);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7361);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7362);
							match s {
								[] => true,
								['a', 'l', 'i', 'c', s @ ..] => {
									super::push(out, 9826);
									match s {
										[] => true,
										['i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11237);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['e', 't', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7364);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1337);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4038);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4039);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7366);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9827);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9828);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4040);
					match s {
						[] => true,
						['c', 'a', 't', s @ ..] => {
							super::push(out, 7368);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9830);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4041);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 7369);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11238);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11239);
											s.is_empty()
										}
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

/// Words ending -qs.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', 'i', 'f', 'a', 'c', 'i', 'e', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 7370);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4042);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

