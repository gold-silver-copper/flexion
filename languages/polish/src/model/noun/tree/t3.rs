use flexion::Path;

/// Words ending -yna.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 10938);
							s.is_empty()
						}
						['l', 'l', s @ ..] => {
							super::push(out, 10939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7089);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7090);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10940);
							s.is_empty()
						}
						['m', 'o', 'e', s @ ..] => {
							super::push(out, 10941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 10942);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 7091);
					s.is_empty()
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['r', 'i', 'p', 'o', s @ ..] => {
							super::push(out, 10944);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 10945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2917);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7092);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 7093);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['k', 'o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10946);
							s.is_empty()
						}
						['t', 'i', s @ ..] => {
							super::push(out, 10947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2918);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 7096);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7097);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7098);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 7099);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2919);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7101);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7102);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10948);
							match s {
								[] => true,
								['d', 'e', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 13336);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 13337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 10949);
							match s {
								[] => true,
								['ł', 'z', 'c', '\u{1}', s @ ..] => {
									super::push(out, 13338);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'n', 'd', 'ę', 'z', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 10950);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 10951);
							match s {
								[] => true,
								['e', 'z', s @ ..] => {
									super::push(out, 13339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 10952);
							match s {
								[] => true,
								['s', 'o', s @ ..] => {
									super::push(out, 13340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7103);
					match s {
						[] => true,
						['l', 'y', s @ ..] => {
							super::push(out, 10953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7104);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7105);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7106);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10954);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 13341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'w', s @ ..] => {
									super::push(out, 13342);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 13343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', 'k', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10955);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 2920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zna.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2921);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2922);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					super::push(out, 7108);
					match s {
						[] => true,
						['g', 'o', 'r', s @ ..] => {
							super::push(out, 10957);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 10958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'b', s @ ..] => {
							super::push(out, 10959);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 10960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 7109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2923);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 7110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2924);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2925);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7112);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 13344);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 13345);
									s.is_empty()
								}
								['j', s @ ..] => {
									super::push(out, 13346);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13347);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 13348);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 15267);
													match s {
														[] => true,
														['ł', s @ ..] => super::t4::n16522(s, out),
														_ => false,
													}
												}
												['r', 'o', 'm', s @ ..] => {
													super::push(out, 15268);
													s.is_empty()
												}
												['w', 'ó', s @ ..] => {
													super::push(out, 15269);
													s.is_empty()
												}
												['ń', s @ ..] => {
													super::push(out, 15270);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ż', s @ ..] => {
									super::push(out, 13349);
									match s {
										[] => true,
										['ę', s @ ..] => {
											super::push(out, 14672);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'w', s @ ..] => {
							super::push(out, 10961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', 'u', s @ ..] => {
					super::push(out, 7113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łna.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -źna.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 613);
	match s {
		[] => true,
		['o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 2927);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żna.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 614);
	match s {
		[] => true,
		['e', 'z', 'r', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2928);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2929);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2930);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 2931);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7114);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7115);
					match s {
						[] => true,
						['a', 'z', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 10962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 7116);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -atra.
pub(crate) fn n3069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2981);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7152);
			s.is_empty()
		}
		['w', '\u{1}', s @ ..] => {
			super::push(out, 7153);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 7154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -etra.
pub(crate) fn n3070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2982);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 7155);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7156);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7157);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stra.
pub(crate) fn n3072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2984);
	match s {
		[] => true,
		['j', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 7158);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7159);
			match s {
				[] => true,
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 10975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word mutra.
pub(crate) fn n7496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7160);
	s.is_empty()
}

/// Words ending -ura.
pub(crate) fn n635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 632);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2986);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2987);
			match s {
				[] => true,
				['c', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2988);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7162);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7163);
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 10977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2989);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2990);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2991);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7164);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2992);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10979);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 13350);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13351);
									s.is_empty()
								}
								['y', 'd', s @ ..] => {
									super::push(out, 13352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 13353);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 13354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'r', s @ ..] => {
							super::push(out, 10980);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', s @ ..] => {
					super::push(out, 7165);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10982);
							s.is_empty()
						}
						['e', 't', s @ ..] => {
							super::push(out, 10983);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10984);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 13355);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 13356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 10985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10986);
							s.is_empty()
						}
						['u', 'k', s @ ..] => {
							match s {
								[] => true,
								['a', 'w', s @ ..] => {
									super::push(out, 13357);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 13358);
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
						['a', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 13359);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 13360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 't', s @ ..] => {
							super::push(out, 10987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7166);
					s.is_empty()
				}
				['s', 'o', 'g', s @ ..] => {
					super::push(out, 7167);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2993);
			s.is_empty()
		}
		['z', 'a', 'l', s @ ..] => {
			super::push(out, 2994);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -óra.
pub(crate) fn n636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 633);
	match s {
		[] => true,
		['k', 's', 'o', 'n', 'r', s @ ..] => {
			super::push(out, 2995);
			match s {
				[] => true,
				['a', 'z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7169);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kita.
pub(crate) fn n3126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3035);
	match s {
		[] => true,
		['o', 'ł', 'o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 7204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lita.
pub(crate) fn n3127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3036);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11004);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11005);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 13363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7205);
			match s {
				[] => true,
				['p', 's', 'o', 'p', 'z', 'c', 'e', 'z', 'r', '\u{1}', s @ ..] => {
					super::push(out, 11006);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mita.
pub(crate) fn n3128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3037);
	s.is_empty()
}

/// Words ending -nita.
pub(crate) fn n3129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3038);
	s.is_empty()
}

/// Words ending -pita.
pub(crate) fn n3130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3039);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rita.
pub(crate) fn n3131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3040);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 7207);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sita.
pub(crate) fn n3132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3041);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			super::push(out, 7208);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -suita.
pub(crate) fn n3133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3042);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -świta.
pub(crate) fn n3134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3043);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jta.
pub(crate) fn n653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 649);
	match s {
		[] => true,
		['a', 'w', s @ ..] => {
			super::push(out, 3044);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kta.
pub(crate) fn n654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7211);
					s.is_empty()
				}
				['d', 'y', 'd', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3046);
			match s {
				[] => true,
				['l', 'a', s @ ..] => {
					super::push(out, 7213);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -lta.
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 651);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3047);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 7214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nta.
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 652);
	match s {
		[] => true,
		['a', 'm', 'o', s @ ..] => {
			super::push(out, 3048);
			match s {
				[] => true,
				['r', 'k', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3049);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7216);
					s.is_empty()
				}
				['t', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3050);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3051);
			match s {
				[] => true,
				['d', 'o', 't', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7218);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ota.
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3052);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3053);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3054);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3055);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3056);
			match s {
				[] => true,
				['d', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7220);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 3057);
			s.is_empty()
		}
		['k', 'a', s @ ..] => {
			super::push(out, 3058);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3059);
			match s {
				[] => true,
				['b', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7222);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7223);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 7224);
					s.is_empty()
				}
				['g', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7225);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 7226);
					s.is_empty()
				}
				['u', 'i', 'k', 'n', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 7227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3060);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7228);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7229);
					match s {
						[] => true,
						['i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3061);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11009);
							s.is_empty()
						}
						['e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3062);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					super::push(out, 7231);
					match s {
						[] => true,
						['ś', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13364);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3063);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 7233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 's', 'o', s @ ..] => {
			super::push(out, 3064);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 3065);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3066);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 3067);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pta.
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 654);
	s.is_empty()
}

/// Words ending -rta.
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['a', 'w', s @ ..] => {
			super::push(out, 3068);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 7235);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 11013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sta.
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3069);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7236);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 11014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 7237);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 7238);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7239);
					s.is_empty()
				}
				['m', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7240);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7241);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11015);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 13366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7242);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7243);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3070);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 7245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3071);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7246);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 11016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3072);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 7248);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 11017);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7249);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11018);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 11019);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 13368);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14673);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 11020);
							match s {
								[] => true,
								['j', 'n', 'a', 'w', 'e', '\u{1}', s @ ..] => {
									super::push(out, 13369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 11021);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11022);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 13371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 11023);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 13372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7250);
					match s {
						[] => true,
						['o', 'g', s @ ..] => {
							super::push(out, 11024);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 13373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7251);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'k', s @ ..] => {
					super::push(out, 7252);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3073);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 7253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3074);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3075);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7254);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7255);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7256);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11027);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 11028);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 11029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 7257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3076);
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7258);
					s.is_empty()
				}
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7259);
					s.is_empty()
				}
				['o', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3077);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7261);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7262);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7263);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3078);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11030);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11031);
							match s {
								[] => true,
								['t', 's', 'a', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 13374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7266);
					match s {
						[] => true,
						['a', 'f', s @ ..] => {
							super::push(out, 11032);
							match s {
								[] => true,
								['o', 'j', 'y', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 13375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'f', 'a', 'j', 'y', s @ ..] => {
							super::push(out, 11033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7267);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'c', s @ ..] => {
					super::push(out, 7268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 3079);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 7269);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tta.
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	s.is_empty()
}

/// Words ending -uta.
pub(crate) fn n662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3080);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7270);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 11036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3081);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3082);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3083);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 3084);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3085);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3086);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3087);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3088);
			match s {
				[] => true,
				['o', 'b', s @ ..] => {
					super::push(out, 7272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yta.
pub(crate) fn n663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7273);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 11038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'f', 'a', 'm', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 7274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3089);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7275);
					s.is_empty()
				}
				['y', 'ż', 'r', 'a', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 7276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3090);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 7277);
					s.is_empty()
				}
				['o', 'w', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', '\u{1}', s @ ..] => {
			super::push(out, 3091);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3092);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 3093);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zta.
pub(crate) fn n664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 660);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3094);
			s.is_empty()
		}
		['r', 'h', 'c', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3095);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3096);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7279);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąta.
pub(crate) fn n665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 661);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęta.
pub(crate) fn n666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 662);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3098);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7280);
					match s {
						[] => true,
						['d', 'o', '\u{1}', s @ ..] => {
							super::push(out, 11039);
							s.is_empty()
						}
						['r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7281);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7282);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'r', 'e', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 13376);
									s.is_empty()
								}
								['l', 'a', 'w', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 13377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'ż', 'r', 'a', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 11041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 7283);
					match s {
						[] => true,
						['ś', '\u{1}', s @ ..] => {
							super::push(out, 11042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3099);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3100);
			match s {
				[] => true,
				['c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7284);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -towa.
pub(crate) fn n3221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3127);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 7318);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7319);
			match s {
				[] => true,
				['t', 'z', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11059);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word wojewodzicowiczowa.
pub(crate) fn n7666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7320);
	s.is_empty()
}

/// The word bednarzowa.
pub(crate) fn n7667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7321);
	s.is_empty()
}

/// Words ending -łowa.
pub(crate) fn n3223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3129);
	match s {
		[] => true,
		['a', 'r', 'i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
			super::push(out, 7322);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żowa.
pub(crate) fn n3224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3130);
	match s {
		[] => true,
		['y', 'z', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 7323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rwa.
pub(crate) fn n679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 675);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3131);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3132);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3133);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 7325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 3134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -twa.
pub(crate) fn n680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 676);
	match s {
		[] => true,
		['a', 'i', s @ ..] => {
			super::push(out, 3135);
			s.is_empty()
		}
		['i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3136);
			s.is_empty()
		}
		['o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 3137);
			s.is_empty()
		}
		['s', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 3138);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3139);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 7326);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uwa.
pub(crate) fn n681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 677);
	s.is_empty()
}

/// Words ending -ywa.
pub(crate) fn n682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 678);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3140);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3141);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3142);
			match s {
				[] => true,
				['a', 'l', s @ ..] => {
					super::push(out, 7327);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11060);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13379);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'p', 's', 'a', s @ ..] => {
							super::push(out, 11061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3143);
			match s {
				[] => true,
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7329);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zwa.
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 679);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3144);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 3145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ówa.
pub(crate) fn n684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 680);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3146);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7330);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łwa.
pub(crate) fn n685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 681);
	s.is_empty()
}

/// Words ending -żwa.
pub(crate) fn n686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 682);
	match s {
		[] => true,
		['y', 'ł', '\u{1}', s @ ..] => {
			super::push(out, 3147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zorza.
pub(crate) fn n7721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7369);
	s.is_empty()
}

/// Words ending -prza.
pub(crate) fn n3290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3187);
	match s {
		[] => true,
		['z', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 7370);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urza.
pub(crate) fn n3291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3188);
	s.is_empty()
}

/// Words ending -sza.
pub(crate) fn n702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 698);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3189);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11074);
							s.is_empty()
						}
						['m', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3190);
			match s {
				[] => true,
				['i', 'p', s @ ..] => {
					super::push(out, 7372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3191);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3192);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 7374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3193);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3194);
			match s {
				[] => true,
				['r', 'p', s @ ..] => {
					super::push(out, 7376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3195);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3196);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7378);
					match s {
						[] => true,
						['j', 'a', s @ ..] => {
							match s {
								[] => true,
								['l', 'u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 13384);
									s.is_empty()
								}
								['z', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 13385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'z', 'c', 'ę', s @ ..] => {
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 13386);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 13387);
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
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11077);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 11078);
							match s {
								[] => true,
								['o', 'ł', '\u{1}', s @ ..] => {
									super::push(out, 13388);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 3197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7379);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3198);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uza.
pub(crate) fn n703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 699);
	match s {
		[] => true,
		['a', 'p', 'o', 'r', s @ ..] => {
			super::push(out, 3199);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3200);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yza.
pub(crate) fn n704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 700);
	match s {
		[] => true,
		['r', 'o', s @ ..] => {
			super::push(out, 3201);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zza.
pub(crate) fn n705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 701);
	s.is_empty()
}

/// Words ending -ęza.
pub(crate) fn n706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 702);
	s.is_empty()
}

/// Words ending -łza.
pub(crate) fn n707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 703);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3202);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iec.
pub(crate) fn n784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 780);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3303);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7427);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7428);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3304);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7430);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 7431);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 7432);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7433);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 7434);
					s.is_empty()
				}
				['ś', s @ ..] => {
					super::push(out, 7435);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11086);
							s.is_empty()
						}
						['o', 'k', s @ ..] => {
							super::push(out, 11087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3305);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3306);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', 't', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11088);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11091);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 11092);
							match s {
								[] => true,
								['o', 'k', 's', 'a', 'ł', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13390);
									s.is_empty()
								}
								['u', 't', '\u{1}', s @ ..] => {
									super::push(out, 13391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 11093);
							s.is_empty()
						}
						['k', 'a', 'ł', '\u{1}', s @ ..] => {
							super::push(out, 11094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 't', 'a', 'r', 'b', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3307);
			match s {
				[] => true,
				['a', s @ ..] => super::t4::n7793(s, out),
				['b', s @ ..] => {
					super::push(out, 7438);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t4::n7795(s, out),
				['i', s @ ..] => {
					super::push(out, 7440);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11115);
							match s {
								[] => true,
								['r', 'k', 'u', '\u{1}', s @ ..] => {
									super::push(out, 13435);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 11116);
							match s {
								[] => true,
								['e', 's', s @ ..] => {
									super::push(out, 13436);
									match s {
										[] => true,
										['o', 'n', 'r', 'a', 'z', 'c', '\u{1}', s @ ..] => {
											super::push(out, 14708);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'd', 'a', 'p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 11117);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11118);
							s.is_empty()
						}
						['z', s @ ..] => super::t4::n11786(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t4::n7797(s, out),
				['r', s @ ..] => super::t4::n7798(s, out),
				['u', 'k', '\u{1}', s @ ..] => super::t4::n7799(s, out),
				['y', s @ ..] => super::t4::n7800(s, out),
				['ź', s @ ..] => super::t4::n7801(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t4::n3416(s, out),
		['s', s @ ..] => super::t4::n3417(s, out),
		['w', s @ ..] => super::t4::n3418(s, out),
		['z', s @ ..] => super::t4::n3419(s, out),
		_ => false,
	}
}

/// The word twardzielec.
pub(crate) fn n16408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15340);
	s.is_empty()
}

/// The word chudzielec.
pub(crate) fn n16409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15341);
	s.is_empty()
}

/// The word rudzielec.
pub(crate) fn n16410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15342);
	s.is_empty()
}

/// Words ending -olec.
pub(crate) fn n3429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3321);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7491);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 7492);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 11189);
					s.is_empty()
				}
				['ą', 'w', '\u{1}', s @ ..] => {
					super::push(out, 11190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7493);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7494);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7495);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 13568);
							s.is_empty()
						}
						['s', 'm', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 13569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7496);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 7497);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rpec.
pub(crate) fn n3433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3325);
	s.is_empty()
}

/// Words ending -rec.
pub(crate) fn n788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 784);
	match s {
		[] => true,
		['y', s @ ..] => {
			super::push(out, 3326);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zec.
pub(crate) fn n789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 785);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 3327);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7500);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13570);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13571);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11193);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11194);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 13572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7501);
					match s {
						[] => true,
						['ę', 'm', '\u{1}', s @ ..] => {
							super::push(out, 11195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'i', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11196);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 11197);
							match s {
								[] => true,
								['d', 'ź', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7502);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7503);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 11198);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13574);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 13575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'y', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7504);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7505);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 7506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3328);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 7507);
					match s {
						[] => true,
						['r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11199);
							s.is_empty()
						}
						['ł', 'g', '\u{1}', s @ ..] => {
							super::push(out, 11200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żec.
pub(crate) fn n790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 786);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3329);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 7508);
					s.is_empty()
				}
				['r', 'o', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 11201);
							match s {
								[] => true,
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 13576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 3330);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 7509);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nięcie.
pub(crate) fn n11903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11228);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14814);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13582);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14816);
					s.is_empty()
				}
				['ą', 'i', 'c', 'o', 'd', 'e', s @ ..] => {
					super::push(out, 14817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 'i', s @ ..] => {
			super::push(out, 13583);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['g', 'a', s @ ..] => {
					super::push(out, 14818);
					s.is_empty()
				}
				['w', 'z', 'o', s @ ..] => {
					super::push(out, 14819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					super::push(out, 14820);
					s.is_empty()
				}
				['i', 'n', 'z', s @ ..] => {
					super::push(out, 14821);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', s @ ..] => {
							super::push(out, 15343);
							s.is_empty()
						}
						['o', 'd', s @ ..] => {
							super::push(out, 15344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 14822);
					s.is_empty()
				}
				['t', 'o', s @ ..] => {
					super::push(out, 14823);
					s.is_empty()
				}
				['y', 'r', s @ ..] => {
					super::push(out, 14824);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 14825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 't', 'u', s @ ..] => {
			super::push(out, 13584);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14826);
					s.is_empty()
				}
				['y', 'z', 'c', 'z', 's', 'u', s @ ..] => {
					super::push(out, 14827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14828);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 'z', 'r', s @ ..] => {
							super::push(out, 15345);
							s.is_empty()
						}
						['o', 'p', s @ ..] => {
							super::push(out, 15346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', 'i', s @ ..] => {
			super::push(out, 13585);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 13586);
			s.is_empty()
		}
		['ś', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'p', s @ ..] => {
							super::push(out, 15347);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14829);
					s.is_empty()
				}
				['u', 'l', s @ ..] => {
					super::push(out, 14830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ź', 'a', s @ ..] => {
			super::push(out, 13587);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -alie.
pub(crate) fn n3555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3440);
	s.is_empty()
}

/// Words ending -mie.
pub(crate) fn n917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 911);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3441);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nie.
pub(crate) fn n918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 912);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3442);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['d', 'e', s @ ..] => {
							super::push(out, 11231);
							s.is_empty()
						}
						['o', 'd', s @ ..] => {
							super::push(out, 11232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 7577);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7578);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11233);
							match s {
								[] => true,
								['i', 'n', s @ ..] => {
									super::push(out, 13588);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 13589);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 13590);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 13591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 11234);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 13592);
									match s {
										[] => true,
										['w', 'o', 'p', s @ ..] => {
											match s {
												[] => true,
												['d', 'o', '\u{1}', s @ ..] => {
													super::push(out, 15349);
													s.is_empty()
												}
												['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 15350);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 11235);
							s.is_empty()
						}
						['ą', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 13593);
									s.is_empty()
								}
								['ż', s @ ..] => {
									super::push(out, 13594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11236);
							s.is_empty()
						}
						['e', 'ż', s @ ..] => {
							super::push(out, 11237);
							s.is_empty()
						}
						['r', 'd', s @ ..] => {
							super::push(out, 11238);
							s.is_empty()
						}
						['ę', 'g', s @ ..] => {
							super::push(out, 11239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => super::t4::n14512(s, out),
								['j', s @ ..] => super::t4::n14513(s, out),
								_ => false,
							}
						}
						['u', 'ł', 's', 'e', s @ ..] => super::t4::n11917(s, out),
						['y', s @ ..] => super::t4::n11918(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t4::n7947(s, out),
				['j', s @ ..] => super::t4::n7948(s, out),
				['k', s @ ..] => super::t4::n7949(s, out),
				['l', s @ ..] => super::t4::n7950(s, out),
				['m', s @ ..] => super::t4::n7951(s, out),
				['n', s @ ..] => super::t4::n7952(s, out),
				['p', s @ ..] => super::t4::n7953(s, out),
				['r', s @ ..] => super::t4::n7954(s, out),
				['s', s @ ..] => super::t4::n7955(s, out),
				['t', s @ ..] => super::t4::n7956(s, out),
				['w', s @ ..] => super::t4::n7957(s, out),
				['z', s @ ..] => super::t4::n7958(s, out),
				['ł', s @ ..] => super::t4::n7959(s, out),
				['ż', 'a', 'w', s @ ..] => super::t4::n7960(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t4::n3558(s, out),
		['e', s @ ..] => super::t4::n3559(s, out),
		['j', s @ ..] => super::t4::n3560(s, out),
		['o', s @ ..] => super::t4::n3561(s, out),
		['y', s @ ..] => super::t4::n3562(s, out),
		['ś', s @ ..] => super::t4::n3563(s, out),
		_ => false,
	}
}

/// Words ending -pie.
pub(crate) fn n919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 913);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3449);
			match s {
				[] => true,
				['l', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 7601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3450);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7602);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7603);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rie.
pub(crate) fn n920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 914);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3451);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sie.
pub(crate) fn n921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 915);
	match s {
		[] => true,
		['o', 'ł', 'w', s @ ..] => {
			super::push(out, 3452);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3453);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3454);
			match s {
				[] => true,
				['t', 'a', 'p', '-', 'e', 'i', 's', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7604);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wie.
pub(crate) fn n922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 916);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3455);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3456);
			match s {
				[] => true,
				['e', 'z', 'g', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3457);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7606);
					match s {
						[] => true,
						['z', 'ó', 'm', s @ ..] => {
							match s {
								[] => true,
								['o', 'd', 'o', 'w', s @ ..] => {
									super::push(out, 13671);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 13672);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7607);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7608);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 7609);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 13673);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 13674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['o', 'p', s @ ..] => {
									super::push(out, 13675);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 13676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ź', s @ ..] => {
			super::push(out, 3458);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7610);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zie.
pub(crate) fn n923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 917);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7611);
					match s {
						[] => true,
						['w', 'o', s @ ..] => {
							super::push(out, 11303);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 11304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'i', 's', 'o', 'ł', s @ ..] => {
					super::push(out, 7612);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7613);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13677);
									s.is_empty()
								}
								['d', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 7614);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iak.
pub(crate) fn n1147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1136);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7836);
					s.is_empty()
				}
				['m', 'o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 7837);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 7838);
					match s {
						[] => true,
						['z', 'r', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 11397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3790);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7839);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 11398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7840);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 11399);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11400);
							s.is_empty()
						}
						['l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11401);
							s.is_empty()
						}
						['s', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11402);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 11403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7841);
					match s {
						[] => true,
						['e', 'd', 'u', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 11404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7842);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7843);
					match s {
						[] => true,
						['a', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11406);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 11407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7844);
					match s {
						[] => true,
						['r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 11408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 7845);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 7846);
					match s {
						[] => true,
						['ó', 'ż', '\u{1}', s @ ..] => {
							super::push(out, 11409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					match s {
						[] => true,
						['e', 'b', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 11410);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11411);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 13718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11412);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 11413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3791);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7847);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7848);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 11414);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 11415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3792);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7849);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7850);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 11417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7851);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3793);
			match s {
				[] => true,
				['a', s @ ..] => super::t4::n8247(s, out),
				['b', s @ ..] => super::t4::n8248(s, out),
				['d', s @ ..] => super::t4::n8249(s, out),
				['e', s @ ..] => super::t4::n8250(s, out),
				['g', s @ ..] => super::t4::n8251(s, out),
				['i', s @ ..] => super::t4::n8252(s, out),
				['j', s @ ..] => super::t4::n8253(s, out),
				['k', s @ ..] => super::t4::n8254(s, out),
				['l', s @ ..] => super::t4::n8255(s, out),
				['m', s @ ..] => super::t4::n8256(s, out),
				['o', s @ ..] => super::t4::n8257(s, out),
				['p', s @ ..] => super::t4::n8258(s, out),
				['r', s @ ..] => super::t4::n8259(s, out),
				['t', s @ ..] => super::t4::n8260(s, out),
				['w', s @ ..] => super::t4::n8261(s, out),
				['y', s @ ..] => super::t4::n8262(s, out),
				['z', s @ ..] => super::t4::n8263(s, out),
				['ł', 'e', 'w', '\u{1}', s @ ..] => super::t4::n8264(s, out),
				['ś', s @ ..] => super::t4::n8265(s, out),
				['ź', s @ ..] => super::t4::n8266(s, out),
				['ż', s @ ..] => super::t4::n8267(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t4::n3926(s, out),
		['r', s @ ..] => super::t4::n3927(s, out),
		['s', s @ ..] => super::t4::n3928(s, out),
		['w', s @ ..] => super::t4::n3929(s, out),
		['z', s @ ..] => super::t4::n3930(s, out),
		_ => false,
	}
}

/// The word oblak.
pub(crate) fn n8294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7891);
	s.is_empty()
}

/// The word wróblak.
pub(crate) fn n8295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7892);
	s.is_empty()
}

/// Words ending -dlak.
pub(crate) fn n3940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3807);
	match s {
		[] => true,
		['y', 'b', '\u{1}', s @ ..] => {
			super::push(out, 7893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -elak.
pub(crate) fn n3941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3808);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7894);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 7895);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 11485);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -flak.
pub(crate) fn n3942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3809);
	s.is_empty()
}

/// Words ending -glak.
pub(crate) fn n3943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3810);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7896);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 7897);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hlak.
pub(crate) fn n3944(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7898);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['e', 'd', 'z', '\u{1}', s @ ..] => {
					super::push(out, 11486);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11487);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -klak.
pub(crate) fn n3945(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 7899);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7900);
			s.is_empty()
		}
		['y', 'w', 'z', '\u{1}', s @ ..] => {
			super::push(out, 7901);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olak.
pub(crate) fn n3946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3811);
	match s {
		[] => true,
		['g', 'i', 'ż', '\u{1}', s @ ..] => {
			super::push(out, 7902);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 7903);
			match s {
				[] => true,
				['a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 11488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7904);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11489);
					s.is_empty()
				}
				['o', 'n', 'j', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 11490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 7905);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11491);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11492);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word cieplak.
pub(crate) fn n3947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3812);
	s.is_empty()
}

/// Words ending -rlak.
pub(crate) fn n3948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3813);
	match s {
		[] => true,
		['a', 'm', 'u', '\u{1}', s @ ..] => {
			super::push(out, 7906);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', 'u', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11493);
					s.is_empty()
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11494);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word cebulak.
pub(crate) fn n3949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3814);
	s.is_empty()
}

/// The word niemowlak.
pub(crate) fn n3950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3815);
	s.is_empty()
}

/// Words ending -ylak.
pub(crate) fn n3951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3816);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7907);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 7908);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 7909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zlak.
pub(crate) fn n3952(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'a', 'm', 'z', '\u{1}', s @ ..] => {
			super::push(out, 7910);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11495);
					s.is_empty()
				}
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 11496);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11497);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ólak.
pub(crate) fn n3953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3817);
	s.is_empty()
}

/// Words ending -ślak.
pub(crate) fn n3954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3818);
	match s {
		[] => true,
		['a', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 7911);
			s.is_empty()
		}
		['b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 7912);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word koźlak.
pub(crate) fn n3955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3819);
	s.is_empty()
}

/// Words ending -żlak.
pub(crate) fn n3956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3820);
	s.is_empty()
}

/// Words ending -mak.
pub(crate) fn n1151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1140);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3821);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7913);
					match s {
						[] => true,
						['j', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 11498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7914);
					s.is_empty()
				}
				['k', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 7915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'l', 'ś', '\u{1}', s @ ..] => {
			super::push(out, 3822);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3823);
			match s {
				[] => true,
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 7916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3824);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7917);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7918);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7919);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 11499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 7920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3825);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 7921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 3826);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7922);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nak.
pub(crate) fn n1152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1141);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3827);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7923);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 3828);
			match s {
				[] => true,
				['o', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3829);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 7926);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3830);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3831);
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 7928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3832);
			match s {
				[] => true,
				['d', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 7929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pak.
pub(crate) fn n1153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1142);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3834);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3835);
			match s {
				[] => true,
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3836);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7931);
					match s {
						[] => true,
						['ś', '\u{1}', s @ ..] => {
							super::push(out, 11501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 7932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3837);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7933);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 7934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3838);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7935);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3839);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3840);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11502);
							s.is_empty()
						}
						['d', 'n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 11503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 3841);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11504);
							s.is_empty()
						}
						['s', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 3842);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rak.
pub(crate) fn n1154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1143);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3843);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3844);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7937);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7938);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7939);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7940);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7941);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3845);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7942);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7943);
					match s {
						[] => true,
						['ż', '\u{1}', s @ ..] => {
							super::push(out, 11507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3846);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7945);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7946);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11508);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 11509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3847);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7948);
					s.is_empty()
				}
				['z', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3848);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3849);
			match s {
				[] => true,
				['b', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 7950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 3850);
			match s {
				[] => true,
				['o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3851);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3852);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7952);
					match s {
						[] => true,
						['e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 7953);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7954);
					s.is_empty()
				}
				['s', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7955);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7956);
					match s {
						[] => true,
						['ł', 'ó', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 7957);
					match s {
						[] => true,
						['z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3853);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3854);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7958);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7959);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 7960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3855);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3856);
			match s {
				[] => true,
				['z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7963);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sak.
pub(crate) fn n1155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1144);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3857);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3858);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3859);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7964);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 11515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 7966);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3860);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3861);
			match s {
				[] => true,
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 7969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 3862);
			match s {
				[] => true,
				['i', 'm', 'o', s @ ..] => {
					super::push(out, 7970);
					match s {
						[] => true,
						['k', 'a', 'i', 'n', 'k', 'ó', 'ł', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 7971);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tak.
pub(crate) fn n1156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1145);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3863);
			match s {
				[] => true,
				['n', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3864);
			match s {
				[] => true,
				['l', 'o', 'p', 's', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'k', 's', '\u{1}', s @ ..] => {
			super::push(out, 3865);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3866);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7974);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3867);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3868);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7976);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3869);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3870);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 3871);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 3872);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 7979);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wak.
pub(crate) fn n1157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1146);
	match s {
		[] => true,
		['a', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3873);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3874);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3875);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7980);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7981);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3876);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3877);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3878);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7984);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3879);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 7986);
					s.is_empty()
				}
				['z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7987);
					s.is_empty()
				}
				['ż', '\u{1}', s @ ..] => {
					super::push(out, 7988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 7989);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13777);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 13778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7990);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zak.
pub(crate) fn n1158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1147);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3880);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3881);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7991);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7992);
					match s {
						[] => true,
						['o', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7993);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7994);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11518);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', s @ ..] => {
					super::push(out, 7995);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 11520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7996);
					match s {
						[] => true,
						['e', 'p', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11521);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 7997);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11523);
							match s {
								[] => true,
								['t', 's', 'd', s @ ..] => {
									super::push(out, 13779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 11524);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13780);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 13781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'ż', 'o', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 11525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 7998);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7999);
					match s {
						[] => true,
						['s', 'u', 'ł', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13782);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 13783);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14934);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 8000);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 11526);
							match s {
								[] => true,
								['a', 'p', s @ ..] => {
									super::push(out, 13784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11527);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 8001);
					match s {
						[] => true,
						['ó', 'i', 'c', 'ś', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3882);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3883);
			match s {
				[] => true,
				['z', 'r', '\u{1}', s @ ..] => {
					super::push(out, 8002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3884);
			match s {
				[] => true,
				['y', 'z', '\u{1}', s @ ..] => {
					super::push(out, 8003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3885);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3886);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 8004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3887);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8005);
					match s {
						[] => true,
						['n', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 11530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'd', s @ ..] => {
					super::push(out, 8006);
					s.is_empty()
				}
				['f', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8007);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8008);
					s.is_empty()
				}
				['t', 's', 'y', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3888);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 11531);
							s.is_empty()
						}
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11532);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8010);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8011);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8012);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8013);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 11535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8014);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'r', 'e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8015);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 8016);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11537);
							s.is_empty()
						}
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 11538);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 11539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3889);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3890);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 3891);
			match s {
				[] => true,
				['l', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 8017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 3892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łak.
pub(crate) fn n1159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1148);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3893);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3894);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 8018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3895);
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 8019);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8020);
					match s {
						[] => true,
						['b', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11540);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 11541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3896);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3897);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3898);
			match s {
				[] => true,
				['a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8022);
					s.is_empty()
				}
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 3899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żak.
pub(crate) fn n1160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1149);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3900);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3901);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 11542);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 11543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3902);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3903);
			match s {
				[] => true,
				['i', 'w', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 8024);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
			super::push(out, 3904);
			s.is_empty()
		}
		['y', 'z', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 3905);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ładek.
pub(crate) fn n8462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8052);
	s.is_empty()
}

/// Words ending -idek.
pub(crate) fn n4069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3928);
	match s {
		[] => true,
		['o', 'r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 8053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ldek.
pub(crate) fn n4070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3929);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8054);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8055);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word endek.
pub(crate) fn n4071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3930);
	s.is_empty()
}

/// Words ending -odek.
pub(crate) fn n4072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3931);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 8056);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 11556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 8057);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8058);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11558);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 13792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11559);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14936);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 15368);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 14937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 13793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 11560);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 13794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 11561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 8059);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14938);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 13795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 8060);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 11562);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 11563);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdek.
pub(crate) fn n4073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3932);
	match s {
		[] => true,
		['a', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 8061);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'p', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 11564);
					s.is_empty()
				}
				['t', 'h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 11565);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word esdek.
pub(crate) fn n4074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3933);
	s.is_empty()
}

/// Words ending -udek.
pub(crate) fn n4075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3934);
	match s {
		[] => true,
		['l', 'd', 'o', '\u{1}', s @ ..] => {
			super::push(out, 8062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ydek.
pub(crate) fn n4076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3935);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 8063);
			s.is_empty()
		}
		['ż', '\u{1}', s @ ..] => {
			super::push(out, 8064);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ódek.
pub(crate) fn n4077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3936);
	s.is_empty()
}

/// Words ending -ądek.
pub(crate) fn n4078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3937);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 8065);
			match s {
				[] => true,
				['a', 'ł', '\u{1}', s @ ..] => {
					super::push(out, 11566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8066);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8067);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 11567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 8068);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13796);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 8069);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11568);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eek.
pub(crate) fn n1167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1156);
	match s {
		[] => true,
		['g', '\u{1}', s @ ..] => {
			super::push(out, 3938);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iek.
pub(crate) fn n1168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1157);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3939);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 8070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8071);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 8072);
					match s {
						[] => true,
						['r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', 'o', s @ ..] => {
					super::push(out, 8073);
					match s {
						[] => true,
						['d', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3940);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8074);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3941);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3942);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8076);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 11572);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 11573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8077);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8078);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11574);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 11575);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 11576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8080);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 8081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 3943);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8082);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8083);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8084);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11577);
							match s {
								[] => true,
								['k', 'o', 't', s @ ..] => {
									super::push(out, 13798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8085);
					match s {
						[] => true,
						['ł', 'z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ć', s @ ..] => {
					super::push(out, 8086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jek.
pub(crate) fn n1169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1158);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 8087);
					s.is_empty()
				}
				['r', 'g', s @ ..] => {
					super::push(out, 8088);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3944);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 8089);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 13799);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 13800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 11580);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8090);
					match s {
						[] => true,
						['r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3945);
			match s {
				[] => true,
				['n', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3946);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 8092);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 8093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'r', 't', s @ ..] => {
			super::push(out, 3947);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 8094);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lek.
pub(crate) fn n1170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1159);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3948);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3949);
			match s {
				[] => true,
				['n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 8095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3950);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8096);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11583);
							s.is_empty()
						}
						['m', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11584);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11585);
							s.is_empty()
						}
						['ó', s @ ..] => {
							super::push(out, 11586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8097);
					match s {
						[] => true,
						['n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 11587);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11588);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 13802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11589);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 13803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8098);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11590);
							match s {
								[] => true,
								['i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 13804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8100);
					match s {
						[] => true,
						['s', 'a', s @ ..] => {
							super::push(out, 11592);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 13805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 8101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3951);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3952);
			match s {
				[] => true,
				['p', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3953);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3954);
			match s {
				[] => true,
				['d', 'a', 'i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8103);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8104);
					s.is_empty()
				}
				['n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 8105);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8106);
					s.is_empty()
				}
				['t', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 8107);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8108);
					match s {
						[] => true,
						['c', 'j', 'o', '\u{1}', s @ ..] => {
							super::push(out, 11594);
							s.is_empty()
						}
						['s', 'o', 'k', 'd', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 8109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3955);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11596);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mek.
pub(crate) fn n1171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3956);
			match s {
				[] => true,
				['d', 'i', 'w', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3957);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8111);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 11598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3958);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 8112);
					s.is_empty()
				}
				['i', 'z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11599);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 11600);
							match s {
								[] => true,
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', 'a', 'n', 'z', '\u{1}', s @ ..] => {
					super::push(out, 8113);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8114);
					s.is_empty()
				}
				['t', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8115);
					s.is_empty()
				}
				['ł', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 8116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3959);
			match s {
				[] => true,
				['e', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3960);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 8118);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nek.
pub(crate) fn n1172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1161);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3961);
			match s {
				[] => true,
				['b', 'y', s @ ..] => {
					super::push(out, 8119);
					match s {
						[] => true,
						['d', 'r', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8120);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8121);
					match s {
						[] => true,
						['c', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8122);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11604);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 13807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8123);
					match s {
						[] => true,
						['y', 'z', 'r', 't', 'u', '\u{1}', s @ ..] => {
							super::push(out, 11606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11608);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 13808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11609);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11610);
							match s {
								[] => true,
								['j', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 13809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8125);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11611);
							s.is_empty()
						}
						['s', 'y', s @ ..] => {
							super::push(out, 11612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 8126);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11613);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 11614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8127);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11615);
							match s {
								[] => true,
								['ł', '\u{1}', s @ ..] => {
									super::push(out, 13810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 8128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3962);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8130);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8131);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13811);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 13812);
									s.is_empty()
								}
								['o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 13813);
									s.is_empty()
								}
								['u', 'w', 'd', '\u{1}', s @ ..] => {
									super::push(out, 13814);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 't', 'e', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8132);
					match s {
						[] => true,
						['a', 'ł', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', 'o', '\u{1}', s @ ..] => {
					super::push(out, 8133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3963);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8134);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 11619);
							match s {
								[] => true,
								['r', 'e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 13815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8135);
					match s {
						[] => true,
						['ł', s @ ..] => {
							super::push(out, 11620);
							match s {
								[] => true,
								['ó', 'ż', '\u{1}', s @ ..] => {
									super::push(out, 13816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 8136);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8137);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11621);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 11622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8138);
					match s {
						[] => true,
						['a', 'i', s @ ..] => {
							super::push(out, 11623);
							match s {
								[] => true,
								['n', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 13817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 11624);
							s.is_empty()
						}
						['ś', 'd', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3964);
			match s {
				[] => true,
				['g', 'o', 'i', s @ ..] => {
					super::push(out, 8139);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8140);
					match s {
						[] => true,
						['s', 'o', s @ ..] => {
							super::push(out, 11626);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13818);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8141);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 8142);
					match s {
						[] => true,
						['i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'y', 'c', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8143);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8144);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11628);
							s.is_empty()
						}
						['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11629);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 11630);
							s.is_empty()
						}
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 11631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8145);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 8146);
					match s {
						[] => true,
						['z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 8147);
					match s {
						[] => true,
						['ł', 'a', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13819);
									s.is_empty()
								}
								['ł', 'ó', 'p', 's', 'w', '\u{1}', s @ ..] => {
									super::push(out, 13820);
									s.is_empty()
								}
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
			super::push(out, 3965);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3966);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11633);
							match s {
								[] => true,
								['w', 'r', 'e', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13821);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 11634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3967);
			match s {
				[] => true,
				['l', 'ś', s @ ..] => {
					super::push(out, 8148);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 11635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8149);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 11636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t4::n4110(s, out),
		_ => false,
	}
}

/// Words ending -pek.
pub(crate) fn n1173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1162);
	match s {
		[] => true,
		['a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 3969);
			s.is_empty()
		}
		['e', 'l', s @ ..] => {
			super::push(out, 3970);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 8157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'h', s @ ..] => {
			super::push(out, 3971);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3972);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 8159);
					s.is_empty()
				}
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3973);
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 8161);
					s.is_empty()
				}
				['k', 's', '\u{1}', s @ ..] => {
					super::push(out, 8162);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8163);
					s.is_empty()
				}
				['r', 't', 'z', 'o', 'r', '-', 'k', 'e', 'p', 'o', 'ł', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3974);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 8166);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 8167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11653);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 11654);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13826);
									s.is_empty()
								}
								['ł', 'ó', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13827);
									s.is_empty()
								}
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
			super::push(out, 3975);
			match s {
				[] => true,
				['n', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8168);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 8169);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 3976);
			match s {
				[] => true,
				['t', 's', 'y', s @ ..] => {
					super::push(out, 8171);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 11655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rek.
pub(crate) fn n1174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1163);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3977);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8172);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8173);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8174);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8175);
					match s {
						[] => true,
						['w', 'o', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'u', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 8176);
					s.is_empty()
				}
				['n', 'a', s @ ..] => {
					super::push(out, 8177);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 11659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'k', 'e', s @ ..] => {
					super::push(out, 8178);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 11660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8179);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11661);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 11662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3978);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'ę', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3979);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3980);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8181);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11663);
							match s {
								[] => true,
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 13828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8182);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11664);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11665);
							match s {
								[] => true,
								['f', 'o', '\u{1}', s @ ..] => {
									super::push(out, 13829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'y', s @ ..] => {
					super::push(out, 8183);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 11666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8184);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8185);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 11667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'z', s @ ..] => {
					super::push(out, 8186);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', s @ ..] => {
					super::push(out, 8187);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 11669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', s @ ..] => {
					super::push(out, 8188);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 11670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8189);
					match s {
						[] => true,
						['k', 'a', 'r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 8190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3981);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8191);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3982);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8193);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 8194);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11672);
							s.is_empty()
						}
						['ż', s @ ..] => {
							super::push(out, 11673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3983);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 8195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3984);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8197);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8198);
					match s {
						[] => true,
						['c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11676);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 13830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 11677);
							match s {
								[] => true,
								['a', 'l', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 13831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'ą', 'g', '\u{1}', s @ ..] => {
							super::push(out, 11678);
							s.is_empty()
						}
						['z', 'd', 'ę', s @ ..] => {
							super::push(out, 11679);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 13832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 's', s @ ..] => {
					super::push(out, 8199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8200);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8201);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11681);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8202);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11684);
							s.is_empty()
						}
						['e', 'f', s @ ..] => {
							super::push(out, 11685);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8203);
					match s {
						[] => true,
						['k', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 11686);
							s.is_empty()
						}
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11687);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 11688);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13834);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8204);
					match s {
						[] => true,
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 11689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11690);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13835);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 11691);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14940);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 14941);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 13836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'w', s @ ..] => {
					super::push(out, 8205);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3985);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t4::n4128(s, out),
		['ó', s @ ..] => {
			super::push(out, 3987);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8218);
					s.is_empty()
				}
				['h', s @ ..] => super::t4::n8640(s, out),
				['i', 'p', 'i', 'z', 'y', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8220);
					s.is_empty()
				}
				['k', s @ ..] => super::t4::n8642(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sek.
pub(crate) fn n1175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1164);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3988);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8222);
					match s {
						[] => true,
						['u', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 11708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'o', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8223);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 11709);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 11710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11711);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 11712);
							s.is_empty()
						}
						['o', 'r', 'ó', 'g', '\u{1}', s @ ..] => {
							super::push(out, 11713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8224);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11714);
							s.is_empty()
						}
						['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'p', 'u', 'ł', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8225);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 8226);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 8227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3989);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3990);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 8228);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8229);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3991);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8231);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8232);
					match s {
						[] => true,
						['n', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11718);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'i', 'p', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8234);
					s.is_empty()
				}
				['t', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 8235);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 8236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3992);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3993);
			match s {
				[] => true,
				['l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8238);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11720);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 11721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 8239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3994);
			match s {
				[] => true,
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 8240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 3995);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 3996);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 8241);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tek.
pub(crate) fn n1176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1165);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3997);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8242);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11722);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 11723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8243);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 11724);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 11725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8244);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8245);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 11726);
							match s {
								[] => true,
								['ś', 'ł', s @ ..] => {
									super::push(out, 13840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8246);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11727);
							match s {
								[] => true,
								['i', 'c', 'ę', 'i', 's', 'e', 'i', 'z', 'd', 'ć', s @ ..] => {
									super::push(out, 13841);
									match s {
										[] => true,
										['ś', 'e', 'z', 's', '\u{1}', s @ ..] => {
											super::push(out, 14943);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 13842);
									match s {
										[] => true,
										['w', 'ó', 'r', '\u{1}', s @ ..] => {
											super::push(out, 14944);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', 'e', s @ ..] => {
									super::push(out, 13843);
									s.is_empty()
								}
								['ł', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 13844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11728);
							match s {
								[] => true,
								['k', 'l', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 13845);
									s.is_empty()
								}
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 13846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8247);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 11729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8248);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11730);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14945);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 14946);
											s.is_empty()
										}
										['r', 'a', s @ ..] => {
											super::push(out, 14947);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 8249);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8250);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 8251);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11731);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 11732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3998);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3999);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8252);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11733);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 11734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8253);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8254);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8255);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 8256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 4000);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8257);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 11736);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 11737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4001);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4002);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8258);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8259);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11738);
							match s {
								[] => true,
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11739);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8260);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8261);
					match s {
						[] => true,
						['d', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11741);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['r', 'ó', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13848);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 13849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 11742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8262);
					s.is_empty()
				}
				['r', 'w', '\u{1}', s @ ..] => {
					super::push(out, 8263);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8264);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11743);
							s.is_empty()
						}
						['c', 'z', 's', 'e', 'l', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 11744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 8265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4003);
			match s {
				[] => true,
				['a', 'w', 'z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4004);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8267);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 8268);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 11745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8269);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11746);
							match s {
								[] => true,
								['ę', 'z', 'r', '\u{1}', s @ ..] => {
									super::push(out, 13850);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8270);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 11747);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13851);
									s.is_empty()
								}
								['y', 'w', '\u{1}', s @ ..] => {
									super::push(out, 13852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 11748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8271);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'ł', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4005);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8273);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8274);
					s.is_empty()
				}
				['k', s @ ..] => super::t4::n8699(s, out),
				['l', s @ ..] => super::t4::n8700(s, out),
				['m', s @ ..] => super::t4::n8701(s, out),
				['s', s @ ..] => super::t4::n8702(s, out),
				['z', s @ ..] => super::t4::n8703(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t4::n4148(s, out),
		['z', s @ ..] => super::t4::n4149(s, out),
		['ą', s @ ..] => super::t4::n4150(s, out),
		['ę', s @ ..] => super::t4::n4151(s, out),
		['ł', 'ó', 'ż', '\u{1}', s @ ..] => super::t4::n4152(s, out),
		_ => false,
	}
}

/// Words ending -wek.
pub(crate) fn n1177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1166);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 8292);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 8293);
					match s {
						[] => true,
						['p', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'z', 'd', 'o', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8294);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 8295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4011);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 8296);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 8297);
					s.is_empty()
				}
				['z', 'd', 'w', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'y', 's', '\u{1}', s @ ..] => {
			super::push(out, 4012);
			s.is_empty()
		}
		['o', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 4013);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 4014);
			match s {
				[] => true,
				['ł', 'g', 'ł', 'ó', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8299);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zek.
pub(crate) fn n1178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1167);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4015);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 8300);
					match s {
						[] => true,
						['a', 'n', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 8301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4016);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8302);
					s.is_empty()
				}
				['a', s @ ..] => super::t4::n8727(s, out),
				['e', s @ ..] => {
					super::push(out, 8304);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11778);
							match s {
								[] => true,
								['ą', 'ł', 'o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 13893);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => super::t4::n12505(s, out),
						['i', s @ ..] => super::t4::n12506(s, out),
						['m', s @ ..] => super::t4::n12507(s, out),
						['n', s @ ..] => super::t4::n12508(s, out),
						['r', s @ ..] => super::t4::n12509(s, out),
						['s', s @ ..] => super::t4::n12510(s, out),
						['t', s @ ..] => super::t4::n12511(s, out),
						['z', s @ ..] => super::t4::n12512(s, out),
						['ł', s @ ..] => super::t4::n12513(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t4::n8729(s, out),
				['l', s @ ..] => super::t4::n8730(s, out),
				['o', s @ ..] => super::t4::n8731(s, out),
				['r', s @ ..] => super::t4::n8732(s, out),
				['u', s @ ..] => super::t4::n8733(s, out),
				['y', s @ ..] => super::t4::n8734(s, out),
				['z', s @ ..] => super::t4::n8735(s, out),
				['ą', s @ ..] => super::t4::n8736(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n4161(s, out),
		['r', s @ ..] => super::t4::n4162(s, out),
		['s', s @ ..] => super::t4::n4163(s, out),
		['ą', s @ ..] => super::t4::n4164(s, out),
		_ => false,
	}
}

/// Words ending -łek.
pub(crate) fn n1179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1168);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4021);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8322);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8323);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11847);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 11848);
							match s {
								[] => true,
								['ś', '\u{1}', s @ ..] => {
									super::push(out, 13957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 11849);
							match s {
								[] => true,
								['d', 'e', 'i', s @ ..] => {
									super::push(out, 13958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8324);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 8325);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8326);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11850);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 13959);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 13960);
									s.is_empty()
								}
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
			super::push(out, 4022);
			match s {
				[] => true,
				['b', 'a', s @ ..] => {
					super::push(out, 8327);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11851);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 11852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'r', 'o', s @ ..] => {
					super::push(out, 8328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4023);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11853);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', 'o', '\u{1}', s @ ..] => {
									super::push(out, 13961);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 13962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8330);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8331);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 11854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4024);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8332);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8333);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8334);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8335);
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							super::push(out, 11855);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 11856);
							s.is_empty()
						}
						['s', 'e', s @ ..] => {
							super::push(out, 11857);
							match s {
								[] => true,
								['i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 13963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 11858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8336);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11859);
							s.is_empty()
						}
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 11860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', s @ ..] => {
					super::push(out, 8337);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13964);
									s.is_empty()
								}
								['o', 'l', 'p', 'y', 'd', '\u{1}', s @ ..] => {
									super::push(out, 13965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 8338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4025);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4026);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 8339);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 8340);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					super::push(out, 8341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4027);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 8342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 4028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żek.
pub(crate) fn n1180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1169);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4029);
			match s {
				[] => true,
				['i', 'n', s @ ..] => {
					super::push(out, 8343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4030);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8344);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8345);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 8346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 4031);
			match s {
				[] => true,
				['r', 'w', s @ ..] => {
					super::push(out, 8347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 4032);
			match s {
				[] => true,
				['r', 't', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8348);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -akcik.
pub(crate) fn n4192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4048);
	s.is_empty()
}

/// Words ending -ncik.
pub(crate) fn n4193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4049);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8375);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 11876);
					match s {
						[] => true,
						['e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 13973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11877);
					match s {
						[] => true,
						['y', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 13974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11878);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 13975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8376);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 11880);
					s.is_empty()
				}
				['d', 'u', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 11881);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11882);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13976);
							s.is_empty()
						}
						['i', 'l', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 13977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11883);
					match s {
						[] => true,
						['a', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 13978);
							s.is_empty()
						}
						['g', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 13979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 8377);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 8378);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 11885);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ocik.
pub(crate) fn n4194(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 8379);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 11886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8380);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8381);
			s.is_empty()
		}
		['n', 'k', s @ ..] => {
			super::push(out, 8382);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 8383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcik.
pub(crate) fn n4195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4050);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8384);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 11887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['p', 'z', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11888);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 11889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8385);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucik.
pub(crate) fn n4196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4051);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 8386);
			match s {
				[] => true,
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 11890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tycik.
pub(crate) fn n4197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4052);
	s.is_empty()
}

/// Words ending -ącik.
pub(crate) fn n4198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4053);
	s.is_empty()
}

/// The word sprzęcik.
pub(crate) fn n4199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4054);
	s.is_empty()
}

/// Words ending -ścik.
pub(crate) fn n4200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4055);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 8387);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 11891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8388);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 11892);
					s.is_empty()
				}
				['r', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eik.
pub(crate) fn n1184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1173);
	match s {
		[] => true,
		['l', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fik.
pub(crate) fn n1185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1174);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4057);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4058);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gik.
pub(crate) fn n1186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1175);
	s.is_empty()
}

/// Words ending -lik.
pub(crate) fn n1187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1176);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4060);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4061);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8389);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8390);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8391);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8392);
					match s {
						[] => true,
						['e', 's', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 8393);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11897);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 13980);
									s.is_empty()
								}
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
			super::push(out, 4062);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8394);
					s.is_empty()
				}
				['ó', 'r', 'w', '\u{1}', s @ ..] => {
					super::push(out, 8395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4063);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8396);
					match s {
						[] => true,
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4064);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8397);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8398);
					match s {
						[] => true,
						['n', 'a', 'w', 'e', '\u{1}', s @ ..] => {
							super::push(out, 11899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8399);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 't', s @ ..] => {
					super::push(out, 8400);
					match s {
						[] => true,
						['s', 'y', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8401);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 11902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8402);
					s.is_empty()
				}
				['y', 'z', 'c', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 8403);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 8404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4065);
			s.is_empty()
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8405);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4066);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4067);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8407);
					match s {
						[] => true,
						['a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8408);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8409);
					match s {
						[] => true,
						['ą', 'k', '\u{1}', s @ ..] => {
							super::push(out, 11904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8410);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 8411);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8412);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11905);
							match s {
								[] => true,
								['k', '-', s @ ..] => {
									super::push(out, 13981);
									match s {
										[] => true,
										['k', 'a', 'l', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 15004);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 8413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4068);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8414);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8415);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4069);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4070);
			match s {
				[] => true,
				['e', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 8417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4071);
			match s {
				[] => true,
				['a', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 8418);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 8419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8420);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 11908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8421);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8422);
					match s {
						[] => true,
						['y', 't', '\u{1}', s @ ..] => {
							super::push(out, 11909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', '\u{1}', s @ ..] => {
					super::push(out, 8423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4072);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8424);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8425);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 11910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 4073);
			match s {
				[] => true,
				['g', 'e', 'z', 'c', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 8426);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ś', s @ ..] => {
			super::push(out, 4074);
			s.is_empty()
		}
		['ź', s @ ..] => {
			super::push(out, 4075);
			match s {
				[] => true,
				['u', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mik.
pub(crate) fn n1188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1177);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4076);
			match s {
				[] => true,
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4077);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8430);
					match s {
						[] => true,
						['a', 'k', 'a', '\u{1}', s @ ..] => {
							super::push(out, 11911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8431);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8432);
					match s {
						[] => true,
						['b', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11912);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8433);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 11914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4078);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 4079);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4080);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8435);
					s.is_empty()
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 8437);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8438);
					s.is_empty()
				}
				['n', 'o', 'k', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8439);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8440);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 11915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11916);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 8441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4081);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8442);
					s.is_empty()
				}
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 8443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4082);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 8444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'h', 'c', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4083);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4084);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 4085);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nik.
pub(crate) fn n1189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1178);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4086);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 8445);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8446);
					s.is_empty()
				}
				['t', 'o', s @ ..] => {
					super::push(out, 8447);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 11918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4087);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8448);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 'z', 'a', 'ł', '\u{1}', s @ ..] => {
							super::push(out, 11919);
							s.is_empty()
						}
						['z', 'c', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 11920);
							s.is_empty()
						}
						['ż', 'u', 'ł', 's', '\u{1}', s @ ..] => {
							super::push(out, 11921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8449);
					match s {
						[] => true,
						['d', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13983);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 13984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 13985);
									s.is_empty()
								}
								['y', 'w', '\u{1}', s @ ..] => {
									super::push(out, 13986);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11922);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 13987);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', 'a', 'ż', '\u{1}', s @ ..] => {
							super::push(out, 11923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8450);
					s.is_empty()
				}
				['y', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11924);
							s.is_empty()
						}
						['o', 'l', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 11925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 8451);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', 'ł', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['e', 'z', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8453);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 11927);
							s.is_empty()
						}
						['n', 'ł', 'ó', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11928);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 11929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4088);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11930);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 11931);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 11932);
							s.is_empty()
						}
						['r', 'd', 'z', '\u{1}', s @ ..] => {
							super::push(out, 11933);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 11934);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11935);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 13988);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 11936);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['a', 'z', s @ ..] => {
											super::push(out, 15005);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 15382);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', '\u{1}', s @ ..] => {
											super::push(out, 15006);
											s.is_empty()
										}
										['w', '\u{1}', s @ ..] => {
											super::push(out, 15007);
											s.is_empty()
										}
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
					super::push(out, 8454);
					match s {
						[] => true,
						['i', 'w', s @ ..] => {
							match s {
								[] => true,
								['o', 'p', s @ ..] => {
									match s {
										[] => true,
										['a', 'z', '\u{1}', s @ ..] => {
											super::push(out, 15008);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 15009);
											match s {
												[] => true,
												['o', '\u{1}', s @ ..] => {
													super::push(out, 15383);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 15010);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 15011);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ź', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 13989);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'ś', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13990);
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 13991);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'r', 'p', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 11937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8455);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 11938);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 11939);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 11940);
							match s {
								[] => true,
								['z', 's', 'y', 'ż', 'u', '\u{1}', s @ ..] => {
									super::push(out, 13992);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 11941);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13993);
									s.is_empty()
								}
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 13994);
									s.is_empty()
								}
								['g', 'o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t4::n16060(s, out),
										['n', 'i', 'w', '\u{1}', s @ ..] => super::t4::n16061(s, out),
										_ => false,
									}
								}
								['y', 'z', 'r', 'p', '\u{1}', s @ ..] => super::t4::n14953(s, out),
								_ => false,
							}
						}
						['w', s @ ..] => super::t4::n12684(s, out),
						['ł', 'p', s @ ..] => super::t4::n12685(s, out),
						_ => false,
					}
				}
				['r', 'o', 'k', 'a', '\u{1}', s @ ..] => super::t4::n8890(s, out),
				['u', s @ ..] => super::t4::n8891(s, out),
				['y', 't', 's', 'w', 'z', 'e', 'b', '\u{1}', s @ ..] => super::t4::n8892(s, out),
				['ą', s @ ..] => super::t4::n8893(s, out),
				['ę', s @ ..] => super::t4::n8894(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n4238(s, out),
		['g', s @ ..] => super::t4::n4239(s, out),
		['h', s @ ..] => super::t4::n4240(s, out),
		['i', s @ ..] => super::t4::n4241(s, out),
		['j', s @ ..] => super::t4::n4242(s, out),
		['k', 'y', 'p', s @ ..] => super::t4::n4243(s, out),
		['l', s @ ..] => super::t4::n4244(s, out),
		['m', s @ ..] => super::t4::n4245(s, out),
		['n', s @ ..] => super::t4::n4246(s, out),
		['o', s @ ..] => super::t4::n4247(s, out),
		['p', s @ ..] => super::t4::n4248(s, out),
		['r', s @ ..] => super::t4::n4249(s, out),
		['t', s @ ..] => super::t4::n4250(s, out),
		['u', s @ ..] => super::t4::n4251(s, out),
		['w', s @ ..] => super::t4::n4252(s, out),
		['y', s @ ..] => super::t4::n4253(s, out),
		['z', s @ ..] => super::t4::n4254(s, out),
		['ł', s @ ..] => super::t4::n4255(s, out),
		['ś', s @ ..] => super::t4::n4256(s, out),
		['ź', s @ ..] => super::t4::n4257(s, out),
		['ż', s @ ..] => super::t4::n4258(s, out),
		_ => false,
	}
}

/// Words ending -oik.
pub(crate) fn n1190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1179);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 4110);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4111);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4112);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 4113);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4114);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12108);
							s.is_empty()
						}
						['z', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 12109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 4115);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pik.
pub(crate) fn n1191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1180);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4116);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4117);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8531);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8532);
					match s {
						[] => true,
						['ś', '\u{1}', s @ ..] => {
							super::push(out, 12110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8533);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8534);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 12111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4118);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 8535);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 12112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8536);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12113);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 12114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4119);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8537);
					s.is_empty()
				}
				['e', 't', s @ ..] => {
					super::push(out, 8538);
					match s {
						[] => true,
						['f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12115);
							s.is_empty()
						}
						['h', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4120);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4121);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 8539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ś', s @ ..] => {
			super::push(out, 4122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rik.
pub(crate) fn n1192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1181);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 4123);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sik.
pub(crate) fn n1193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1182);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 8540);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 't', s @ ..] => {
					super::push(out, 8541);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4124);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8543);
					s.is_empty()
				}
				['r', 'd', 'a', s @ ..] => {
					super::push(out, 8544);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'e', 's', s @ ..] => {
			super::push(out, 4125);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4126);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 8546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4127);
			match s {
				[] => true,
				['g', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8547);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8548);
					match s {
						[] => true,
						['a', 'b', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8549);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12119);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 8550);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8551);
					s.is_empty()
				}
				['ł', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4128);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8553);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 14145);
									s.is_empty()
								}
								['t', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 14146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8554);
					match s {
						[] => true,
						['k', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4129);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 8555);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tik.
pub(crate) fn n1194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1183);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4130);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4131);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 8556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4132);
			match s {
				[] => true,
				['a', 'l', 'p', 'o', s @ ..] => {
					super::push(out, 8557);
					s.is_empty()
				}
				['j', 'o', 'ż', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wik.
pub(crate) fn n1195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1184);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4134);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 12122);
							s.is_empty()
						}
						['u', 'ż', '\u{1}', s @ ..] => {
							super::push(out, 12123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8559);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 12124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8560);
					match s {
						[] => true,
						['c', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 12125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 8561);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 12126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4135);
			match s {
				[] => true,
				['l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8562);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 12127);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12128);
							match s {
								[] => true,
								['l', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 14147);
									s.is_empty()
								}
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
			super::push(out, 4136);
			match s {
				[] => true,
				['o', 'i', 'c', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4137);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 8564);
					match s {
						[] => true,
						['c', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 12129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 8565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ć', '\u{1}', s @ ..] => {
			super::push(out, 4138);
			s.is_empty()
		}
		['ł', 'ó', 'ż', '\u{1}', s @ ..] => {
			super::push(out, 4139);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zik.
pub(crate) fn n1196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1185);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4140);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8566);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8567);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 8568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4141);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4142);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8570);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 8571);
					match s {
						[] => true,
						['ł', 'k', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 14148);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 14149);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 14150);
									s.is_empty()
								}
								['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14151);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'l', 'ś', s @ ..] => {
					super::push(out, 8572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8573);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12133);
							s.is_empty()
						}
						['ł', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14152);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 14153);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15063);
											s.is_empty()
										}
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
					super::push(out, 8574);
					s.is_empty()
				}
				['u', 'r', s @ ..] => {
					super::push(out, 8575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 8576);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 8577);
					match s {
						[] => true,
						['b', 'a', 'ł', '\u{1}', s @ ..] => {
							super::push(out, 12136);
							s.is_empty()
						}
						['z', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 12137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ź', 'o', 'r', s @ ..] => {
					super::push(out, 8578);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 12138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4143);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4144);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 8579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8580);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tyk.
pub(crate) fn n1252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1239);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4258);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8644);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 12154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8645);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 12155);
							match s {
								[] => true,
								['e', 'n', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 8646);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8647);
					match s {
						[] => true,
						['s', 'o', 'i', 'r', 'e', 't', 'k', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4259);
			match s {
				[] => true,
				['g', 'l', s @ ..] => {
					super::push(out, 8648);
					match s {
						[] => true,
						['a', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 's', s @ ..] => {
					super::push(out, 8649);
					match s {
						[] => true,
						['o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14157);
									s.is_empty()
								}
								['o', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 14158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'g', s @ ..] => {
					super::push(out, 8650);
					match s {
						[] => true,
						['a', 'm', 'o', 'r', 'r', 'e', 'f', 'y', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8651);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8652);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 8653);
					match s {
						[] => true,
						['e', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4260);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4261);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4262);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12160);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 12161);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 12162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8654);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4263);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8656);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8657);
					match s {
						[] => true,
						['c', 'y', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8658);
					s.is_empty()
				}
				['r', 'u', s @ ..] => {
					super::push(out, 8659);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 12164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4264);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', s @ ..] => {
									super::push(out, 14159);
									s.is_empty()
								}
								['o', 'r', s @ ..] => {
									super::push(out, 14160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 12165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 8660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', s @ ..] => {
			super::push(out, 4265);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4266);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8661);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8662);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 8663);
					s.is_empty()
				}
				['y', 'r', s @ ..] => {
					super::push(out, 8664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4267);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 8666);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4268);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 8668);
					match s {
						[] => true,
						['f', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12166);
							s.is_empty()
						}
						['m', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 12167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zyk.
pub(crate) fn n1254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1241);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4269);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4270);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8670);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 12168);
							s.is_empty()
						}
						['l', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12169);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12170);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 14161);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 14162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 8671);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12171);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8672);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8673);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12173);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 14163);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8674);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 12174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 8675);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12175);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 14164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 12176);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14165);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 14166);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 14167);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15064);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 12178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8676);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14168);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 14169);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 14170);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 15065);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 15388);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', 'o', s @ ..] => {
									super::push(out, 14171);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 14172);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15066);
											s.is_empty()
										}
										['o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 15067);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'd', s @ ..] => {
							super::push(out, 12179);
							match s {
								[] => true,
								['i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 14173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12180);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 14174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'k', s @ ..] => {
							super::push(out, 12181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8677);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['j', 'y', 's', '\u{1}', s @ ..] => {
									super::push(out, 14175);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 14176);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15068);
											s.is_empty()
										}
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
					super::push(out, 8678);
					match s {
						[] => true,
						['k', 'r', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 12182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8679);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', 't', 'e', '\u{1}', s @ ..] => {
									super::push(out, 14177);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 14178);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15069);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ł', s @ ..] => super::t4::n15152(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t4::n9146(s, out),
				['u', s @ ..] => super::t4::n9147(s, out),
				['w', s @ ..] => super::t4::n9148(s, out),
				['z', s @ ..] => super::t4::n9149(s, out),
				['ę', s @ ..] => super::t4::n9150(s, out),
				['ń', s @ ..] => super::t4::n9151(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t4::n4425(s, out),
		['i', s @ ..] => super::t4::n4426(s, out),
		['r', s @ ..] => super::t4::n4427(s, out),
		['s', s @ ..] => super::t4::n4428(s, out),
		['ę', s @ ..] => super::t4::n4429(s, out),
		_ => false,
	}
}

/// The word wężyk.
pub(crate) fn n9175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8709);
	s.is_empty()
}

/// Words ending -iel.
pub(crate) fn n1296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1283);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4392);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8772);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8773);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8774);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 12220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 8775);
					s.is_empty()
				}
				['ę', 'ł', 'g', '\u{1}', s @ ..] => {
					super::push(out, 8776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4393);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8777);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 12221);
							match s {
								[] => true,
								['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'y', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 12222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8778);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12223);
							match s {
								[] => true,
								['ó', 'r', 'w', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 12224);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12225);
							s.is_empty()
						}
						['w', 'y', s @ ..] => {
							super::push(out, 12226);
							match s {
								[] => true,
								['ż', '\u{1}', s @ ..] => {
									super::push(out, 14218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 12227);
							match s {
								[] => true,
								['d', 'o', 'w', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 15089);
											match s {
												[] => true,
												['o', '\u{1}', s @ ..] => {
													super::push(out, 15393);
													s.is_empty()
												}
												_ => false,
											}
										}
										['y', s @ ..] => {
											super::push(out, 15090);
											match s {
												[] => true,
												['z', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 15394);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8779);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 8780);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 12229);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'c', 'z', s @ ..] => {
							super::push(out, 12230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 8781);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12231);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 14220);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 12232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4394);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8782);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8783);
					s.is_empty()
				}
				['j', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12233);
							s.is_empty()
						}
						['e', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8784);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 14221);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 14222);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8785);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'm', s @ ..] => {
					super::push(out, 8786);
					match s {
						[] => true,
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 12236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 8787);
					match s {
						[] => true,
						['i', 'z', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14223);
									s.is_empty()
								}
								['y', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 14224);
									s.is_empty()
								}
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
			super::push(out, 4395);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8788);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8789);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 12238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8790);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4396);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 8791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12240);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 12241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8792);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12242);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 12243);
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
				['a', s @ ..] => {
					super::push(out, 8793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4398);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 8794);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 8795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4399);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4400);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8797);
					s.is_empty()
				}
				['r', 'u', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 8798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4401);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8799);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 15091);
											s.is_empty()
										}
										['w', 't', '\u{1}', s @ ..] => {
											super::push(out, 15092);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 14225);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['m', 'ś', '\u{1}', s @ ..] => {
													super::push(out, 15395);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 15396);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ą', s @ ..] => {
							super::push(out, 12244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 8800);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rel.
pub(crate) fn n1303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1290);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4419);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 8810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4420);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8811);
					s.is_empty()
				}
				['s', 'n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8812);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sel.
pub(crate) fn n1304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1291);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4422);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 8813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4423);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8814);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tel.
pub(crate) fn n1305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1292);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4424);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 8815);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8816);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 8817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4425);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4426);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4427);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4428);
			match s {
				[] => true,
				['e', 't', 'h', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4429);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4430);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 8820);
					s.is_empty()
				}
				['y', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4431);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4432);
			s.is_empty()
		}
		['z', 's', 'a', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8822);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 4433);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 8824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uel.
pub(crate) fn n1306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1293);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 4434);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wel.
pub(crate) fn n1307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1294);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4435);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word axel.
pub(crate) fn n1308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1295);
	s.is_empty()
}

/// Words ending -zel.
pub(crate) fn n1309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1296);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4436);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 8825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4437);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8826);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 8827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 4438);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4439);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4440);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8829);
					s.is_empty()
				}
				['i', 'm', 'r', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 8830);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 8831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4441);
			match s {
				[] => true,
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8832);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żel.
pub(crate) fn n1310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1297);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 4442);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4443);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 8833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 4444);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cjonalizm.
pub(crate) fn n16466(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'n', 'r', s @ ..] => {
			super::push(out, 15466);
			s.is_empty()
		}
		['n', 'i', s @ ..] => {
			super::push(out, 15467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zjonalizm.
pub(crate) fn n16467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15397);
	s.is_empty()
}

/// Words ending -anizm.
pub(crate) fn n9400(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 12285);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 12286);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14231);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14232);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14233);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14234);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 14235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 12287);
			s.is_empty()
		}
		['k', 'y', 'r', 'e', s @ ..] => {
			super::push(out, 12288);
			s.is_empty()
		}
		['m', 'r', s @ ..] => {
			super::push(out, 12289);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 12290);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onizm.
pub(crate) fn n9403(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 12291);
			s.is_empty()
		}
		['j', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							super::push(out, 15095);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							super::push(out, 15096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'a', 'l', s @ ..] => {
			super::push(out, 12292);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 12293);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 12294);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 12295);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14238);
					s.is_empty()
				}
				['h', 'c', 'n', 'y', 's', 'a', s @ ..] => {
					super::push(out, 14239);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -atynizm.
pub(crate) fn n9404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8927);
	s.is_empty()
}

/// Words ending -oizm.
pub(crate) fn n4758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4589);
	s.is_empty()
}

/// Words ending -propizm.
pub(crate) fn n4759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4590);
	s.is_empty()
}

/// Words ending -omarksizm.
pub(crate) fn n4760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4591);
	s.is_empty()
}

/// Words ending -wizm.
pub(crate) fn n4761(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 's', s @ ..] => {
			super::push(out, 8928);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 8929);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yzm.
pub(crate) fn n1427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1411);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4592);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8930);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14240);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 14241);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 14242);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8931);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12297);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12299);
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							super::push(out, 12300);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12301);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 14243);
									s.is_empty()
								}
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
			super::push(out, 4593);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8932);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12302);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12303);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 12304);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12305);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12306);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 14244);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 14245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 't', 's', 'a', s @ ..] => {
					super::push(out, 8933);
					s.is_empty()
				}
				['i', 'p', 's', s @ ..] => {
					super::push(out, 8934);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12307);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'e', 'c', 'o', 'e', 't', s @ ..] => {
					super::push(out, 8935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'k', s @ ..] => {
			super::push(out, 4594);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4595);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14246);
									s.is_empty()
								}
								['o', 'r', s @ ..] => {
									super::push(out, 14247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'k', 'o', 't', 'u', s @ ..] => {
							super::push(out, 12309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'o', s @ ..] => {
					super::push(out, 8936);
					s.is_empty()
				}
				['k', 'e', 'l', 'a', s @ ..] => {
					super::push(out, 8937);
					s.is_empty()
				}
				['n', 'a', 'm', 'e', s @ ..] => {
					super::push(out, 8938);
					s.is_empty()
				}
				['o', 'i', s @ ..] => {
					super::push(out, 8939);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 8940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4596);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12311);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 12312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ekan.
pub(crate) fn n4805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4631);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 8973);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12321);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8974);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 12322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 8975);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 12323);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word kankan.
pub(crate) fn n9460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8977);
	s.is_empty()
}

/// Words ending -rkan.
pub(crate) fn n4809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4635);
	s.is_empty()
}

/// Words ending -ukan.
pub(crate) fn n4810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4636);
	match s {
		[] => true,
		['t', '\u{1}', s @ ..] => {
			super::push(out, 8978);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ykan.
pub(crate) fn n4811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4637);
	match s {
		[] => true,
		['r', 'e', 'm', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lan.
pub(crate) fn n1438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1422);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4638);
			match s {
				[] => true,
				['b', 'm', 'a', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 8980);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8981);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4639);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8983);
					s.is_empty()
				}
				['u', 'm', 'a', 'z', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4640);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 8985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4641);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -man.
pub(crate) fn n1439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1423);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4643);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4644);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 14252);
									s.is_empty()
								}
								['ł', '\u{1}', s @ ..] => {
									super::push(out, 14253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 12325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 4645);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4646);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4647);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4648);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8986);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 12326);
							s.is_empty()
						}
						['t', 'o', '\u{1}', s @ ..] => {
							super::push(out, 12327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4649);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					super::push(out, 8987);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8988);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12328);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 12329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4650);
			match s {
				[] => true,
				['e', 'd', s @ ..] => {
					super::push(out, 8990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4651);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8991);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 8992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4652);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 8993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4653);
			match s {
				[] => true,
				['i', 'l', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 8994);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nan.
pub(crate) fn n1440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1424);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4654);
			s.is_empty()
		}
		['o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 4655);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pan.
pub(crate) fn n1441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1425);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4656);
			s.is_empty()
		}
		['c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4657);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4658);
			match s {
				[] => true,
				['c', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8995);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8996);
					match s {
						[] => true,
						['b', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 12331);
							s.is_empty()
						}
						['n', 'ś', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 12332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4659);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 8997);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 8998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4660);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8999);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4661);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4662);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4663);
			match s {
				[] => true,
				['ż', '\u{1}', s @ ..] => {
					super::push(out, 9001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4664);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12333);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 14254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 12334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ć', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 4665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ran.
pub(crate) fn n1442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1426);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4666);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 9002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4667);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 4668);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4669);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4670);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9004);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9005);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9006);
					match s {
						[] => true,
						['r', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4671);
			match s {
				[] => true,
				['f', 'o', s @ ..] => {
					super::push(out, 9007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4672);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 9008);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -san.
pub(crate) fn n1443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1427);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 4673);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tan.
pub(crate) fn n1444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1428);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4674);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9009);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9010);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12336);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 9011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4675);
			match s {
				[] => true,
				['m', 'o', s @ ..] => {
					super::push(out, 9012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4676);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4677);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9014);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 4678);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4679);
			match s {
				[] => true,
				['o', 'k', 'a', 'i', s @ ..] => {
					super::push(out, 9016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4680);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4681);
			match s {
				[] => true,
				['o', 'g', s @ ..] => {
					super::push(out, 9017);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4682);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9018);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4683);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9020);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 9021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4684);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 4685);
			match s {
				[] => true,
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 9022);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word donżuan.
pub(crate) fn n1445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1429);
	s.is_empty()
}

/// Words ending -van.
pub(crate) fn n1446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1430);
	s.is_empty()
}

/// Words ending -wan.
pub(crate) fn n1447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1431);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4686);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4687);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 4688);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zan.
pub(crate) fn n1448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1432);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4689);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9023);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4690);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9025);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12339);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', 'o', 'd', s @ ..] => {
					super::push(out, 9026);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4691);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12342);
							s.is_empty()
						}
						['i', 'r', s @ ..] => {
							super::push(out, 12343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łan.
pub(crate) fn n1449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1433);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4693);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4694);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4695);
			match s {
				[] => true,
				['o', 'm', 'a', 'z', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9027);
					s.is_empty()
				}
				['u', 'm', 'a', 'i', 'z', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4696);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9029);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9030);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żan.
pub(crate) fn n1450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1434);
	s.is_empty()
}

/// Words ending -dżin.
pub(crate) fn n4960(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9069);
			s.is_empty()
		}
		['a', 't', '\u{1}', s @ ..] => {
			super::push(out, 9070);
			s.is_empty()
		}
		['i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urion.
pub(crate) fn n9596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9108);
	match s {
		[] => true,
		['t', 'n', 'e', 'c', '\u{1}', s @ ..] => {
			super::push(out, 12368);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word busion.
pub(crate) fn n4989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4807);
	s.is_empty()
}

/// The word kimation.
pub(crate) fn n4990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4808);
	s.is_empty()
}

/// Words ending -jon.
pub(crate) fn n1499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1483);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4809);
			match s {
				[] => true,
				['o', 't', 'o', '\u{1}', s @ ..] => {
					super::push(out, 9109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4810);
			s.is_empty()
		}
		['u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4811);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4812);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 9110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kon.
pub(crate) fn n1500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1484);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4813);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4814);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 9112);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lon.
pub(crate) fn n1501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1485);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4815);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9113);
					match s {
						[] => true,
						['m', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 12369);
							s.is_empty()
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
				['m', s @ ..] => {
					super::push(out, 9114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4817);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9115);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9116);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 12371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4818);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'b', s @ ..] => {
			super::push(out, 4819);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4820);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4821);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4822);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'n', s @ ..] => {
			super::push(out, 4823);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mon.
pub(crate) fn n1502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1486);
	match s {
		[] => true,
		['a', 'n', s @ ..] => {
			super::push(out, 4824);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4825);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4826);
			match s {
				[] => true,
				['o', 'm', s @ ..] => {
					super::push(out, 9120);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['c', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9121);
					s.is_empty()
				}
				['t', 'n', s @ ..] => {
					super::push(out, 9122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -non.
pub(crate) fn n1503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1487);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4827);
			s.is_empty()
		}
		['e', 'f', s @ ..] => {
			super::push(out, 4828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pon.
pub(crate) fn n1504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1488);
	match s {
		[] => true,
		['m', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4829);
			s.is_empty()
		}
		['z', 's', '\u{1}', s @ ..] => {
			super::push(out, 4830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ron.
pub(crate) fn n1505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1489);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4831);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 9123);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 9124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'n', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 9125);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9126);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 9127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9128);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9129);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4833);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9131);
					match s {
						[] => true,
						['s', 'o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 14263);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 14264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 9132);
					match s {
						[] => true,
						['s', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4834);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4835);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4836);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4837);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9133);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4838);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9135);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9136);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4839);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -son.
pub(crate) fn n1506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1490);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4840);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4841);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4842);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12377);
							match s {
								[] => true,
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 14266);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 12378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4843);
			match s {
				[] => true,
				['a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 9139);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ton.
pub(crate) fn n1507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1491);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4844);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4845);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9141);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 9142);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12380);
							match s {
								[] => true,
								['z', 'a', 'g', s @ ..] => {
									super::push(out, 14267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 9143);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9144);
					s.is_empty()
				}
				['k', 'o', s @ ..] => {
					super::push(out, 9145);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4846);
			match s {
				[] => true,
				['c', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4847);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9148);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4848);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9150);
					match s {
						[] => true,
						['m', 'd', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4849);
			s.is_empty()
		}
		['p', 'y', s @ ..] => {
			super::push(out, 4850);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4851);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4852);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9151);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 12382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4853);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9152);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9153);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 12383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -won.
pub(crate) fn n1508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1492);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4854);
			s.is_empty()
		}
		['a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4855);
			s.is_empty()
		}
		['i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4856);
			s.is_empty()
		}
		['z', 'd', '\u{1}', s @ ..] => {
			super::push(out, 4857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zon.
pub(crate) fn n1509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1493);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4858);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9154);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4859);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4860);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4861);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4862);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4863);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['k', 'ę', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 12384);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12385);
							s.is_empty()
						}
						['p', 'a', s @ ..] => {
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 14268);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 14269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 12386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9158);
					match s {
						[] => true,
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 12387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'b', 'y', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 9159);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łon.
pub(crate) fn n1510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1494);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 4864);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4865);
			s.is_empty()
		}
		['p', 'a', 'k', s @ ..] => {
			super::push(out, 4866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9160);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ictwo.
pub(crate) fn n9716(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14281);
					s.is_empty()
				}
				['c', 'o', 'm', 'o', 'n', s @ ..] => {
					super::push(out, 14282);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 15101);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14283);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 14284);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14285);
					match s {
						[] => true,
						['e', 'c', s @ ..] => {
							super::push(out, 15103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 15400);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 15401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', 's', 'b', s @ ..] => {
							super::push(out, 15105);
							s.is_empty()
						}
						['e', 'i', 'w', s @ ..] => {
							super::push(out, 15106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 15107);
							s.is_empty()
						}
						['y', 's', s @ ..] => {
							super::push(out, 15108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', 'y', s @ ..] => {
							super::push(out, 15109);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', 'ą', s @ ..] => {
									super::push(out, 15402);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15403);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 15404);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									super::push(out, 15405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', 'e', 'l', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 15406);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 15407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 15110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 14286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 12417);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęctwo.
pub(crate) fn n9720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9224);
	s.is_empty()
}

/// Words ending -stwo.
pub(crate) fn n5186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4997);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9225);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9226);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 9227);
			match s {
				[] => true,
				['e', 'i', 'z', 'd', 'o', 'ł', 'o', s @ ..] => {
					super::push(out, 12418);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 12419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9228);
			match s {
				[] => true,
				['e', 'i', 'c', 'y', s @ ..] => {
					super::push(out, 12420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9229);
			match s {
				[] => true,
				['i', 'z', s @ ..] => {
					super::push(out, 12421);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 12422);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12423);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12424);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12425);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14287);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', 's', 'a', 'b', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 15111);
									s.is_empty()
								}
								['s', 'e', 'f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15112);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9230);
			match s {
				[] => true,
				['u', 'k', 's', 'i', 'b', 'y', s @ ..] => {
					super::push(out, 12427);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 12428);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 12429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9231);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12430);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14289);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 15113);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14290);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'k', s @ ..] => {
									super::push(out, 15114);
									s.is_empty()
								}
								['z', 'd', 'u', s @ ..] => {
									super::push(out, 15115);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'e', 'l', s @ ..] => {
							super::push(out, 14291);
							s.is_empty()
						}
						['l', 'o', 't', s @ ..] => {
							super::push(out, 14292);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14293);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15116);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12431);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 15117);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 15118);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14294);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', 'ź', s @ ..] => {
									super::push(out, 15119);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 15120);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'o', s @ ..] => {
							super::push(out, 14295);
							s.is_empty()
						}
						['n', 't', s @ ..] => {
							super::push(out, 14296);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14297);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15121);
									s.is_empty()
								}
								['y', 'ż', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 15122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 15123);
									s.is_empty()
								}
								['s', 'i', 'n', s @ ..] => {
									super::push(out, 15124);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 14298);
							match s {
								[] => true,
								['s', 'ł', s @ ..] => {
									super::push(out, 15125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ż', s @ ..] => {
							super::push(out, 14299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12432);
					match s {
						[] => true,
						['d', 'a', 's', 'a', 'b', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 14300);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 14301);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14302);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14303);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 15408);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 15409);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 15410);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 15411);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 15126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 12433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 9232);
			match s {
				[] => true,
				['a', 'n', 'z', 'o', 'z', s @ ..] => {
					super::push(out, 12434);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', 'b', 'o', s @ ..] => {
			super::push(out, 9233);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 9234);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14304);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14305);
							match s {
								[] => true,
								['k', 'y', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 15127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ń', s @ ..] => {
			super::push(out, 9235);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12436);
					match s {
						[] => true,
						['i', 'b', s @ ..] => {
							super::push(out, 14306);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 14307);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14308);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15128);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 15129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14309);
							s.is_empty()
						}
						['z', 'c', s @ ..] => {
							super::push(out, 14310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12437);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => super::t4::n16187(s, out),
								['n', s @ ..] => super::t4::n16188(s, out),
								_ => false,
							}
						}
						['l', s @ ..] => super::t4::n15302(s, out),
						['z', 's', s @ ..] => super::t4::n15303(s, out),
						['ż', 'e', s @ ..] => super::t4::n15304(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ztwo.
pub(crate) fn n5187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4998);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12438);
					s.is_empty()
				}
				['ó', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 14314);
							s.is_empty()
						}
						['w', 'e', s @ ..] => {
							super::push(out, 14315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ywo.
pub(crate) fn n1635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1615);
	match s {
		[] => true,
		['z', 'c', 'e', s @ ..] => {
			super::push(out, 4999);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rfer.
pub(crate) fn n5361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5164);
	s.is_empty()
}

/// Words ending -sfer.
pub(crate) fn n5362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5165);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 9343);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9344);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ufer.
pub(crate) fn n5363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5166);
	match s {
		[] => true,
		['a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 9345);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 9346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yfer.
pub(crate) fn n5364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5167);
	match s {
		[] => true,
		['c', 'u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 9347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ger.
pub(crate) fn n1739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1717);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5168);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 9348);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9349);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 12471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5169);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -her.
pub(crate) fn n1740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1718);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5171);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9351);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14319);
									s.is_empty()
								}
								['k', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 14320);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 12472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9352);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 12473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9353);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5172);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 9355);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ier.
pub(crate) fn n1741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1719);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5173);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5174);
			match s {
				[] => true,
				['r', 'a', 'b', 'm', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5175);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5176);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9357);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12474);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12475);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14321);
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									super::push(out, 14322);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 14323);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12476);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 12477);
							match s {
								[] => true,
								['z', 's', '\u{1}', s @ ..] => {
									super::push(out, 14324);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 12478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9358);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 12479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9359);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9360);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 9361);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 9362);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 9363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5177);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9364);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9365);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12481);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 14325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 12482);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14326);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 14327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 12483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9366);
					match s {
						[] => true,
						['i', 'w', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 12484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9367);
					match s {
						[] => true,
						['w', 'c', '\u{1}', s @ ..] => {
							super::push(out, 12485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9368);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12486);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14328);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 14329);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 14330);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 12487);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12488);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 14331);
									s.is_empty()
								}
								['j', s @ ..] => {
									super::push(out, 14332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'm', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9369);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9370);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9371);
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							super::push(out, 12489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 't', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9372);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 9373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5178);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5179);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5180);
			match s {
				[] => true,
				['y', 'ż', s @ ..] => {
					super::push(out, 9374);
					match s {
						[] => true,
						['n', 'i', 'o', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5181);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9375);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 12491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5182);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9376);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5183);
			match s {
				[] => true,
				['k', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 9378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5184);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jer.
pub(crate) fn n1742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1720);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5185);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5186);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9379);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 9380);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9381);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 12492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 9382);
					s.is_empty()
				}
				['z', 'j', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 9383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'i', 'f', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5187);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5188);
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5189);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5190);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5191);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9385);
					s.is_empty()
				}
				['n', 'a', 'l', 'a', s @ ..] => {
					super::push(out, 9386);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 12493);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'c', 'i', 'f', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5192);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9387);
					s.is_empty()
				}
				['y', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 9388);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ker.
pub(crate) fn n1743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1721);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5193);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 9389);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9390);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5194);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12495);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5195);
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 9392);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9393);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 12497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9394);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9395);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 9396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5196);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ler.
pub(crate) fn n1744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1722);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5197);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9397);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9398);
					s.is_empty()
				}
				['w', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5198);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5199);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5200);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5201);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5202);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 9402);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 9403);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9404);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9405);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 9406);
					s.is_empty()
				}
				['o', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 9407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 5203);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 12498);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5204);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9408);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9409);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5205);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5206);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 9411);
					s.is_empty()
				}
				['r', 't', 'n', 'o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12500);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5207);
			match s {
				[] => true,
				['a', 't', 'z', 's', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 9412);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9413);
					s.is_empty()
				}
				['u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5208);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mer.
pub(crate) fn n1745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1723);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5210);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5211);
			match s {
				[] => true,
				['j', 'n', 'e', 'c', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9415);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 9416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5212);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9417);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 5213);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5214);
			match s {
				[] => true,
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5215);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 9420);
					s.is_empty()
				}
				['o', 'f', 't', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 5216);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5217);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9422);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 9423);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ner.
pub(crate) fn n1746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1724);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5218);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9424);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9425);
					s.is_empty()
				}
				['p', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 9426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9427);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 12502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', s @ ..] => {
					super::push(out, 9428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'l', s @ ..] => {
			super::push(out, 5219);
			s.is_empty()
		}
		['n', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5220);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5221);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5222);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5223);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5224);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5225);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9432);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -per.
pub(crate) fn n1747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1725);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5226);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9433);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5227);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 9435);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5228);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9437);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9438);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9439);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 12505);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 12506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', 'a', 'n', 's', '\u{1}', s @ ..] => {
			super::push(out, 5229);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5230);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					super::push(out, 9440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5231);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9442);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 9443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5232);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9444);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5233);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 9446);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ser.
pub(crate) fn n1748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1726);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5234);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5235);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 9447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5236);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 9448);
					s.is_empty()
				}
				['s', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 9449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5237);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9450);
					s.is_empty()
				}
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9451);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12508);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 12509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'a', 'd', 'r', 'o', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5238);
			s.is_empty()
		}
		['p', 'a', 'l', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5239);
			s.is_empty()
		}
		['s', 'i', 'w', 'r', 'e', 's', 's', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5240);
			s.is_empty()
		}
		['u', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5241);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5242);
			match s {
				[] => true,
				['ż', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9453);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ter.
pub(crate) fn n1749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1727);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5243);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 9454);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9455);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9456);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 14334);
									s.is_empty()
								}
								['n', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 14335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 12510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 9457);
					match s {
						[] => true,
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 12511);
							s.is_empty()
						}
						['l', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9458);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9459);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 12513);
							match s {
								[] => true,
								['z', 's', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 14336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 12514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9460);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9461);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12515);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 12516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 9462);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 12517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5245);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5246);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9463);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12518);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 14337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9464);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12519);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 5247);
			match s {
				[] => true,
				['a', 'b', 'r', 'a', 't', 's', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 9467);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 12522);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 12523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5248);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9468);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12524);
							s.is_empty()
						}
						['r', 'a', 'h', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14338);
									s.is_empty()
								}
								['c', 'r', 'a', 'w', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 14339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 'h', s @ ..] => {
			match s {
				[] => true,
				['c', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9469);
					s.is_empty()
				}
				['t', 's', 'u', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5249);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 9471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5250);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9472);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9473);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5251);
			match s {
				[] => true,
				['o', 'k', 'i', 'l', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 9475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5252);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9476);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9477);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 12525);
							s.is_empty()
						}
						['z', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 12526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9478);
					match s {
						[] => true,
						['p', 'a', s @ ..] => {
							super::push(out, 12527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5253);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9479);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9480);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12528);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 12529);
							match s {
								[] => true,
								['d', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14340);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 12530);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12531);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 14341);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15134);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 14342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 12532);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 12533);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12534);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12536);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12537);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 12538);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 12539);
							s.is_empty()
						}
						['z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9482);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9483);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12541);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12542);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => super::t4::n15334(s, out),
								['i', s @ ..] => super::t4::n15335(s, out),
								_ => false,
							}
						}
						['m', s @ ..] => super::t4::n13347(s, out),
						['n', s @ ..] => super::t4::n13348(s, out),
						_ => false,
					}
				}
				['j', s @ ..] => super::t4::n9993(s, out),
				['k', s @ ..] => super::t4::n9994(s, out),
				['o', s @ ..] => super::t4::n9995(s, out),
				['p', s @ ..] => super::t4::n9996(s, out),
				['u', s @ ..] => super::t4::n9997(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t4::n5455(s, out),
		['u', s @ ..] => super::t4::n5456(s, out),
		_ => false,
	}
}

/// Words ending -uer.
pub(crate) fn n1750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1728);
	s.is_empty()
}

/// Words ending -ver.
pub(crate) fn n1751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1729);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 5256);
			match s {
				[] => true,
				['s', 's', 'o', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9496);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wer.
pub(crate) fn n1752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1730);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5257);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 9497);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 9498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5258);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xer.
pub(crate) fn n1753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1731);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9499);
					s.is_empty()
				}
				['b', '-', 'k', 'c', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yer.
pub(crate) fn n1754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1732);
	s.is_empty()
}

/// Words ending -zer.
pub(crate) fn n1755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1733);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5260);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9501);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9502);
					s.is_empty()
				}
				['l', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 9503);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9504);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 12556);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9505);
					s.is_empty()
				}
				['z', 's', '\u{1}', s @ ..] => {
					super::push(out, 9506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9507);
					s.is_empty()
				}
				['r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 9508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5261);
			match s {
				[] => true,
				['t', 'p', 'i', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 9509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9510);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9511);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 12558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5262);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5263);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5264);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5265);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9512);
					s.is_empty()
				}
				['b', 'e', 'k', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9513);
					s.is_empty()
				}
				['p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5266);
			match s {
				[] => true,
				['t', 'a', 'm', 'o', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5267);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żer.
pub(crate) fn n1756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1734);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5268);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5269);
			match s {
				[] => true,
				['j', 'o', 'w', 'i', 'm', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5270);
			s.is_empty()
		}
		['e', 'l', 'o', 'w', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 5271);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5272);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 12559);
							s.is_empty()
						}
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5273);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', 'g', s @ ..] => {
							super::push(out, 12561);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12562);
							match s {
								[] => true,
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 14350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'a', s @ ..] => {
					super::push(out, 9517);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sor.
pub(crate) fn n1790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1768);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5323);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5324);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9552);
					match s {
						[] => true,
						['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12577);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 12578);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 9553);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9554);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12581);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 12582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9555);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 12583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5325);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9556);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5326);
			s.is_empty()
		}
		['r', 'u', 'k', 'e', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5327);
			s.is_empty()
		}
		_ => false,
	}
}

