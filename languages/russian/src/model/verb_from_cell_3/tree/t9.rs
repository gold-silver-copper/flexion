use flexion::Path;

/// Words ending -суешься.
pub(crate) fn n7834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7490);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14032);
			match s {
				[] => true,
				['л', 'г', 'о', 'с', '\u{1}', s @ ..] => {
					super::push(out, 20438);
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 20439);
					s.is_empty()
				}
				['р', 'к', '\u{1}', s @ ..] => {
					super::push(out, 20440);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26071);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', '\u{1}', s @ ..] => {
					super::push(out, 20441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14033);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20442);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30225);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'т', 'н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30227);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30228);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30229);
									s.is_empty()
								}
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
			super::push(out, 14034);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20443);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26074);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26075);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26076);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26077);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26078);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30230);
									s.is_empty()
								}
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
			super::push(out, 14035);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26079);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26081);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14036);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20444);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26083);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32488);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32489);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32490);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32491);
											s.is_empty()
										}
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
					super::push(out, 20445);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20446);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26084);
							match s {
								[] => true,
								['и', 'р', 'х', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32492);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33299);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 32493);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33300);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'и', 'г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20447);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14037);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14038);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20450);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20451);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 26086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20452);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14039);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20453);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26089);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26090);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26091);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 14040);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30231);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30232);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30233);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -туешься.
pub(crate) fn n7835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7491);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14041);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20454);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14042);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20456);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 20457);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26094);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32494);
											s.is_empty()
										}
										['о', s @ ..] => {
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
				['п', s @ ..] => {
					super::push(out, 20458);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14043);
			match s {
				[] => true,
				['д', 'е', 'р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26096);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26097);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 26098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 20459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14044);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20460);
					match s {
						[] => true,
						['р', 'т', 'н', 'о', 'к', 'а', s @ ..] => {
							super::push(out, 26099);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20461);
					match s {
						[] => true,
						['л', 'п', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30236);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30237);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 30238);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26100);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14045);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 14046);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20463);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30239);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30240);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20464);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26102);
							s.is_empty()
						}
						['т', 'а', 'п', 'а', s @ ..] => {
							super::push(out, 26103);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30241);
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
						['б', s @ ..] => {
							super::push(out, 26104);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30242);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 26105);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 26106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20465);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20466);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30243);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30244);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 30245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30246);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30248);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30249);
									s.is_empty()
								}
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
			super::push(out, 14047);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14048);
			match s {
				[] => true,
				['а', 'в', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26107);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26108);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20467);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'п', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26111);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26113);
							s.is_empty()
						}
						['п', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 26114);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14049);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20468);
					match s {
						[] => true,
						['л', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30250);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30251);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20469);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26116);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 30253);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32496);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t11::n27685(s, out),
						['т', s @ ..] => super::t11::n27686(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t11::n21659(s, out),
				['ы', s @ ..] => super::t11::n21660(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t11::n14856(s, out),
		['ф', s @ ..] => super::t11::n14857(s, out),
		['х', s @ ..] => super::t11::n14858(s, out),
		['ю', s @ ..] => super::t11::n14859(s, out),
		['я', s @ ..] => super::t11::n14860(s, out),
		_ => false,
	}
}

/// Words ending -фуешься.
pub(crate) fn n7836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7492);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14055);
			match s {
				[] => true,
				['р', 'т', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26123);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14056);
			match s {
				[] => true,
				['л', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26125);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26126);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26127);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30261);
									s.is_empty()
								}
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
			super::push(out, 14057);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хуешься.
pub(crate) fn n7837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7493);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14058);
			match s {
				[] => true,
				['р', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26128);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26129);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 26130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'р', 'т', 'ш', '\u{1}', s @ ..] => {
			super::push(out, 14059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цуешься.
pub(crate) fn n7838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7494);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14060);
			match s {
				[] => true,
				['у', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26131);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26132);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14061);
			match s {
				[] => true,
				['л', 'е', s @ ..] => {
					super::push(out, 20475);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 14062);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 14063);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20477);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26135);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 30262);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30263);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30264);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30265);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30266);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30267);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30268);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32498);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33301);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20478);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 20479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14064);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14065);
			match s {
				[] => true,
				['а', 'в', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26136);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26138);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26139);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14066);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 14067);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['в', 'з', s @ ..] => {
									super::push(out, 30269);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 30270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30271);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чуешься.
pub(crate) fn n7839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7495);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14068);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20480);
					s.is_empty()
				}
				['р', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26141);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 26142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14069);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20481);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20482);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14070);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14071);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14072);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20484);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20485);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 26144);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30273);
									s.is_empty()
								}
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
			super::push(out, 14073);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20486);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30274);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30275);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30276);
									s.is_empty()
								}
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
			super::push(out, 14074);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 14075);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шуешься.
pub(crate) fn n7840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7496);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14076);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14077);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20488);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26145);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хочешься.
pub(crate) fn n14898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14092);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20512);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пчешься.
pub(crate) fn n7845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7501);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14093);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26168);
							s.is_empty()
						}
						['а', 'з', s @ ..] => {
							super::push(out, 26169);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14094);
			match s {
				[] => true,
				['т', '\u{1}', s @ ..] => {
					super::push(out, 20514);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычешься.
pub(crate) fn n7846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7502);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14095);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14096);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14097);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20517);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячешься.
pub(crate) fn n7847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7503);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14098);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26172);
							s.is_empty()
						}
						['а', 'з', s @ ..] => {
							super::push(out, 26173);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шьешься.
pub(crate) fn n7865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7521);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14143);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14144);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 14145);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14146);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20578);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 20579);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20580);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26202);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26203);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20581);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26204);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20582);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20583);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20584);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -убавляешься.
pub(crate) fn n32153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30297);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 32502);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33302);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сплавляешься.
pub(crate) fn n32154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30298);
	s.is_empty()
}

/// Words ending -равляешься.
pub(crate) fn n27811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26234);
	s.is_empty()
}

/// Words ending -ыставляешься.
pub(crate) fn n27812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26235);
	match s {
		[] => true,
		['в', 'о', s @ ..] => {
			super::push(out, 30299);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32503);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивляешься.
pub(crate) fn n21831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20624);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 26236);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32504);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32505);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33304);
									s.is_empty()
								}
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
			super::push(out, 26237);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 26238);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32506);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32507);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 26239);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овляешься.
pub(crate) fn n21832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20625);
	s.is_empty()
}

/// Words ending -твляешься.
pub(crate) fn n21833(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26240);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 26241);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -являешься.
pub(crate) fn n21834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20626);
	s.is_empty()
}

/// Words ending -гляешься.
pub(crate) fn n14974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14166);
	s.is_empty()
}

/// Words ending -еляешься.
pub(crate) fn n14975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14167);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 20627);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 20628);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 20629);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 20630);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30300);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30301);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 30302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 20631);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26242);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иляешься.
pub(crate) fn n14976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14168);
	s.is_empty()
}

/// Words ending -мляешься.
pub(crate) fn n14977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14169);
	s.is_empty()
}

/// Words ending -голяешься.
pub(crate) fn n14978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14170);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 20632);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 26243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пляешься.
pub(crate) fn n14979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14171);
	s.is_empty()
}

/// Words ending -сляешься.
pub(crate) fn n14980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14172);
	s.is_empty()
}

/// Words ending -тляешься.
pub(crate) fn n14981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14173);
	s.is_empty()
}

/// Words ending -уляешься.
pub(crate) fn n14982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14174);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 20633);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 26244);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 20634);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 20635);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 20636);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фляешься.
pub(crate) fn n14983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14175);
	s.is_empty()
}

/// Words ending -хляешься.
pub(crate) fn n14984(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 20637);
			match s {
				[] => true,
				['в', 'а', s @ ..] => {
					super::push(out, 26247);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 30305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 20638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шляешься.
pub(crate) fn n14985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14176);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20639);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26248);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 30306);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 26249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20640);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26250);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 26251);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26252);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 20641);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26253);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыляешься.
pub(crate) fn n14986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14177);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 20642);
			match s {
				[] => true,
				['х', 'у', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30308);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32510);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 30309);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32511);
									s.is_empty()
								}
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
			super::push(out, 20643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -няешься.
pub(crate) fn n7873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7529);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14178);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20644);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26254);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14179);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26255);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26256);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26257);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26258);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26259);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30311);
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
							super::push(out, 26260);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26261);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30313);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30314);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30315);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32512);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14180);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14181);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20646);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20647);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30316);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 30317);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 30318);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 26262);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 26263);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26264);
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30320);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32513);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 30321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 26265);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26266);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 26267);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26268);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20648);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14182);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					super::push(out, 20650);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 30324);
									s.is_empty()
								}
								['у', 'о', s @ ..] => {
									super::push(out, 30325);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32514);
											s.is_empty()
										}
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
					super::push(out, 20651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14183);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 14184);
			match s {
				[] => true,
				['ч', 'о', s @ ..] => {
					super::push(out, 20652);
					match s {
						[] => true,
						['с', 'а', s @ ..] => {
							super::push(out, 26269);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30326);
									s.is_empty()
								}
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
			super::push(out, 14185);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 14186);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14187);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20653);
					s.is_empty()
				}
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 26270);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 26271);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26273);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26274);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32515);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 32516);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 30328);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32517);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30329);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30330);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32518);
											s.is_empty()
										}
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
					super::push(out, 20654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14188);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 14189);
			match s {
				[] => true,
				['е', 'т', s @ ..] => {
					super::push(out, 20655);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26275);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30331);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30332);
									match s {
										[] => true,
										['п', s @ ..] => super::t11::n34476(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t11::n21868(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t11::n14999(s, out),
		['ч', s @ ..] => super::t11::n15000(s, out),
		_ => false,
	}
}

/// Words ending -ояешься.
pub(crate) fn n7874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7530);
	s.is_empty()
}

/// Words ending -ряешься.
pub(crate) fn n7875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7531);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14192);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 14193);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 14194);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14195);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20657);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26276);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26278);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26279);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									super::push(out, 30333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26280);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30334);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20659);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26282);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26283);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26284);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26285);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30337);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30338);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 32520);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26286);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 26287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14196);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20661);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20662);
					s.is_empty()
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26289);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 26290);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30340);
									s.is_empty()
								}
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
			super::push(out, 14197);
			match s {
				[] => true,
				['в', 'т', 'о', s @ ..] => {
					super::push(out, 20663);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14198);
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20664);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 20665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26292);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30341);
									s.is_empty()
								}
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
			super::push(out, 14199);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 14200);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26293);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 30342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26294);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32521);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32522);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32523);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 32524);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33307);
													s.is_empty()
												}
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
							super::push(out, 26295);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30343);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					super::push(out, 20666);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сяешься.
pub(crate) fn n7876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7532);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14201);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уяешься.
pub(crate) fn n7877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7533);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14202);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word муравишься.
pub(crate) fn n32238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30378);
	s.is_empty()
}

/// Words ending -амуравишься.
pub(crate) fn n32239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30379);
	s.is_empty()
}

/// Words ending -савишься.
pub(crate) fn n15051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14240);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 20751);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тавишься.
pub(crate) fn n15052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14241);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26380);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26381);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 26382);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26383);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30380);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26384);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30381);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32531);
									s.is_empty()
								}
								['о', s @ ..] => {
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

/// Words ending -чавишься.
pub(crate) fn n15053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14242);
	match s {
		[] => true,
		['р', 'у', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26385);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26386);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шавишься.
pub(crate) fn n15054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14243);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 20752);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -евишься.
pub(crate) fn n7891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7547);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 14244);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26387);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 14245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -звишься.
pub(crate) fn n7892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7548);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14246);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26389);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26390);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26391);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 26392);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30382);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30383);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30384);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32533);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14247);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20753);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивишься.
pub(crate) fn n7893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7549);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14248);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14249);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20755);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26393);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 26394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20756);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20757);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14250);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20759);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14251);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20761);
					match s {
						[] => true,
						['о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30385);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14252);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20763);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26395);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26396);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26397);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26398);
							match s {
								[] => true,
								['в', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 14253);
			match s {
				[] => true,
				['е', 'п', 'с', 'а', s @ ..] => {
					super::push(out, 20764);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'у', 'к', 'а', s @ ..] => {
					super::push(out, 20765);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14254);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20766);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30388);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30389);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32534);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 30390);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20767);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20768);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26401);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 30391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лвишься.
pub(crate) fn n7894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7550);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14255);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26402);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26403);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 26404);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 26405);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26406);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30393);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32535);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 30394);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32536);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26407);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26408);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26409);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овишься.
pub(crate) fn n7895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7551);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 14256);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20769);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20770);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14257);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20771);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20772);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 20773);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20774);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20775);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20776);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26415);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26416);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20777);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26417);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26418);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30397);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32537);
											s.is_empty()
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 30398);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32538);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20778);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20779);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20780);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14258);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20781);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26422);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32539);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32540);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32541);
											s.is_empty()
										}
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
					super::push(out, 20782);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20783);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20784);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 20785);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14259);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20787);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26425);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26426);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20788);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							super::push(out, 26427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20789);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14260);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20790);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							super::push(out, 26429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20791);
					match s {
						[] => true,
						['а', 'в', 'ш', '\u{1}', s @ ..] => {
							super::push(out, 26430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 14261);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20792);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 26431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рвишься.
pub(crate) fn n7896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7552);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 14262);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20793);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твишься.
pub(crate) fn n7897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7553);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14263);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26432);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14264);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 14265);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20794);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 26434);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30400);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 32542);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 26435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20795);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20796);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26437);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явишься.
pub(crate) fn n7898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7554);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14266);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14267);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14268);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 20798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14269);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14270);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20800);
					match s {
						[] => true,
						['у', 'р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30402);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30403);
									s.is_empty()
								}
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
			super::push(out, 14271);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 20801);
					match s {
						[] => true,
						['л', 'с', '\u{1}', s @ ..] => {
							super::push(out, 26438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14272);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14273);
			match s {
				[] => true,
				['д', 'у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26439);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26440);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'ч', 'у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26442);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26444);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26445);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 14274);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20802);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20803);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20804);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20805);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14275);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20806);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ередишься.
pub(crate) fn n22062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20835);
	match s {
		[] => true,
		['б', 'з', s @ ..] => {
			super::push(out, 26466);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 26467);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 26468);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 26469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чредишься.
pub(crate) fn n22067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20840);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26470);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 26471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -седишься.
pub(crate) fn n15103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14292);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 20841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цедишься.
pub(crate) fn n15104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14293);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20842);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 20843);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 20844);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 20845);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26472);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здишься.
pub(crate) fn n7903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7559);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14294);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20846);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14295);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14296);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20847);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 20848);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26474);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30412);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30413);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32543);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 30414);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32544);
											s.is_empty()
										}
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
					super::push(out, 20849);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20850);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26475);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20851);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20852);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 20853);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20854);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14297);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26477);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26478);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26479);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30416);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 30417);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 30418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26480);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30419);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30420);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30421);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32545);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 26481);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30422);
									s.is_empty()
								}
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
			super::push(out, 14298);
			match s {
				[] => true,
				['в', 'г', '\u{1}', s @ ..] => {
					super::push(out, 20855);
					s.is_empty()
				}
				['м', 'о', 'р', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26482);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26483);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'о', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26485);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идишься.
pub(crate) fn n7904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7560);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14299);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14300);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20857);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20858);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26487);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 26488);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30423);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32546);
											s.is_empty()
										}
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
					super::push(out, 20859);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26489);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 30424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20860);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20861);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20862);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20863);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14301);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20865);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20866);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20868);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20869);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20870);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йдишься.
pub(crate) fn n7905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7561);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14303);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20871);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндишься.
pub(crate) fn n7906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7562);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14304);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20872);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одишься.
pub(crate) fn n7907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7563);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14305);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20873);
					match s {
						[] => true,
						['в', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30425);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30426);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30427);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32547);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14306);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20874);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 20875);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20876);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20877);
					match s {
						[] => true,
						['р', 'е', 'п', 'а', s @ ..] => {
							super::push(out, 26491);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20878);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26492);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30429);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32548);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 26493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20879);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26494);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 26495);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 30430);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32549);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 26496);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26497);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30431);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 32550);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33308);
													match s {
														[] => true,
														['з', s @ ..] => super::t11::n35543(s, out),
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
									super::push(out, 30432);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32551);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33309);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 32552);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 33310);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 32553);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33311);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20880);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 26498);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30433);
									s.is_empty()
								}
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
			super::push(out, 14307);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20881);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20882);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20883);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20884);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14308);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20886);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26501);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30434);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							super::push(out, 26502);
							s.is_empty()
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30435);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30436);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30437);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32554);
											s.is_empty()
										}
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
					super::push(out, 20887);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26503);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26504);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26505);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30438);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30439);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32555);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 26506);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t11::n15120(s, out),
		['х', s @ ..] => {
			super::push(out, 14310);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20897);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 20898);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20899);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26523);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t11::n22130(s, out),
				['с', s @ ..] => super::t11::n22131(s, out),
				['у', s @ ..] => {
					super::push(out, 20902);
					s.is_empty()
				}
				['ы', s @ ..] => super::t11::n22133(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдишься.
pub(crate) fn n7908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7564);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14311);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20904);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26529);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30458);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 30459);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 30460);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30461);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32567);
											s.is_empty()
										}
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
					super::push(out, 20905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26530);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26532);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26533);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26534);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 30462);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 32568);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 30463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26535);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 26536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14312);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26537);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26538);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26539);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удишься.
pub(crate) fn n7909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7565);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14313);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14314);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14315);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20909);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20910);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26541);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20911);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14316);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14317);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20913);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20914);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20915);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14318);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20916);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30465);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30466);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30467);
									s.is_empty()
								}
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
			super::push(out, 14319);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20917);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 20918);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20919);
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
		['н', s @ ..] => {
			super::push(out, 14320);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20920);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20921);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20922);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20923);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14321);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14322);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20924);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26550);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26551);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20925);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 26552);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30469);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20926);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 26553);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20927);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20928);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26556);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26557);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20930);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 20931);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20932);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20933);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20934);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14324);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20935);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26560);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26561);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26562);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26563);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30472);
									s.is_empty()
								}
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
			super::push(out, 14325);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20936);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20937);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 14326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20938);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20939);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20940);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20941);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14327);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдишься.
pub(crate) fn n7910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7566);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 14328);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26568);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26569);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26570);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 26571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдишься.
pub(crate) fn n7911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7567);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14329);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядишься.
pub(crate) fn n7912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7568);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14330);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26572);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14331);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20945);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20946);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30473);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30474);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32569);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30475);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32570);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30476);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30477);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32571);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 20947);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20948);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20949);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20950);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26577);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32572);
											s.is_empty()
										}
										['о', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 20951);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20952);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20953);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20954);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20955);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -резжишься.
pub(crate) fn n22222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20991);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30487);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 30488);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изжишься.
pub(crate) fn n15164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14353);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 20992);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -озжишься.
pub(crate) fn n15165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14354);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26597);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26598);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 26599);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзжишься.
pub(crate) fn n15166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14355);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 20993);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ижишься.
pub(crate) fn n7920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7576);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14356);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 14357);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 14358);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20994);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лжишься.
pub(crate) fn n7921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7577);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14359);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26600);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26601);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нжишься.
pub(crate) fn n7922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7578);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ожишься.
pub(crate) fn n7923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7579);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14361);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20995);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20996);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20997);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14362);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20999);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30491);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30492);
									s.is_empty()
								}
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
			super::push(out, 14363);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21000);
					match s {
						[] => true,
						['к', 'с', s @ ..] => {
							super::push(out, 26602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14364);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21001);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21002);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21003);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							super::push(out, 26603);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21004);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21005);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 26604);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 30494);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 32575);
											s.is_empty()
										}
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
					super::push(out, 21006);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21007);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14365);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14366);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21009);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30495);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 21010);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26606);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 26607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14367);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21011);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26608);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30497);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30498);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32576);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32577);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 26609);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30499);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 26610);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32578);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32579);
											s.is_empty()
										}
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
					super::push(out, 21012);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14368);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21013);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 21015);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржишься.
pub(crate) fn n7924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7580);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14369);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26615);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26616);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26617);
							s.is_empty()
						}
						['о', 'с', s @ ..] => {
							super::push(out, 26618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 26619);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30501);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужишься.
pub(crate) fn n7925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7581);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14370);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14371);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21017);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26620);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26621);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30502);
									s.is_empty()
								}
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
			super::push(out, 14372);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21018);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26623);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26624);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26625);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30503);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26626);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21020);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26628);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26629);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30504);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 30505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 26630);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26631);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30507);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 30508);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 30509);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 30510);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26632);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 26633);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21021);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26634);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30512);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30513);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32580);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26636);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26637);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 26638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21023);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14373);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14374);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21025);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21026);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21027);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21028);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыжишься.
pub(crate) fn n7926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7582);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 14375);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21029);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21030);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 21031);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14376);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21032);
					s.is_empty()
				}
				['у', 'м', '\u{1}', s @ ..] => {
					super::push(out, 21033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14377);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21034);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -южишься.
pub(crate) fn n7927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7583);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 14378);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26642);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26643);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30514);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 30515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 26644);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26645);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 26646);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26647);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30516);
									s.is_empty()
								}
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
			super::push(out, 14379);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26648);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжишься.
pub(crate) fn n7928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7584);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14380);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14381);
			match s {
				[] => true,
				['в', 'е', 'ч', 'о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 21036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -безобразишься.
pub(crate) fn n35309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33314);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33541);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -знообразишься.
pub(crate) fn n35310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33315);
	s.is_empty()
}

/// Words ending -озишься.
pub(crate) fn n7935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7591);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14397);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21057);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30523);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 26666);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30524);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30525);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 30526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 21058);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21059);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21060);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21061);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21062);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26668);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 26669);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21063);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 21064);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30527);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30528);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21065);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26672);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 26673);
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 26674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21066);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21067);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21068);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21069);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30530);
									s.is_empty()
								}
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
			super::push(out, 14398);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26676);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14399);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14400);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21072);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							super::push(out, 26678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21073);
					match s {
						[] => true,
						['о', 'т', '\u{1}', s @ ..] => {
							super::push(out, 26679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14401);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14402);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21075);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26680);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26681);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26682);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30531);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21076);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30532);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30533);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32584);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узишься.
pub(crate) fn n7937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7593);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14405);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14406);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21079);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 21080);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26683);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26684);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14407);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21081);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21082);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14408);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21083);
					match s {
						[] => true,
						['у', 'к', '\u{1}', s @ ..] => {
							super::push(out, 26687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14409);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14410);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21085);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26688);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26689);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26690);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30534);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 30535);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 30536);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 26691);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21086);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26692);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30538);
									s.is_empty()
								}
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
			super::push(out, 14411);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 14412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21087);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 21088);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 21089);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21090);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14413);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 21092);
					match s {
						[] => true,
						['о', 'к', '\u{1}', s @ ..] => {
							super::push(out, 26694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 14414);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 21093);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -азбахвалишься.
pub(crate) fn n35311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33316);
	s.is_empty()
}

/// Words ending -выхвалишься.
pub(crate) fn n32413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30550);
	s.is_empty()
}

/// Words ending -ывалишься.
pub(crate) fn n22345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21107);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 26701);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30551);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -галишься.
pub(crate) fn n15230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14419);
	s.is_empty()
}

/// Words ending -далишься.
pub(crate) fn n15231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14420);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 21108);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26702);
					match s {
						[] => true,
						['к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32590);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32591);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32593);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32594);
									s.is_empty()
								}
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
			super::push(out, 21109);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21110);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жалишься.
pub(crate) fn n15232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14421);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21111);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 21112);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21113);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21114);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 21115);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иалишься.
pub(crate) fn n15233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14422);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 21116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -калишься.
pub(crate) fn n15234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14423);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21117);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21118);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 21119);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21120);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21121);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21122);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21123);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21124);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26708);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26709);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 21125);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 21126);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26712);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26713);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 30553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -малишься.
pub(crate) fn n15235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14424);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 21127);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 21128);
			match s {
				[] => true,
				['а', 'р', 'к', '\u{1}', s @ ..] => {
					super::push(out, 26714);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -палишься.
pub(crate) fn n15236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14425);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21129);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21130);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 21131);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 21132);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21133);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26715);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 26716);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21134);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 21135);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26718);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ралишься.
pub(crate) fn n15237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14426);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 21136);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 26719);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -салишься.
pub(crate) fn n15238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14427);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21137);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21138);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 21139);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 21140);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26721);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чалишься.
pub(crate) fn n15239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14428);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21141);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21142);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30555);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 30556);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32595);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33317);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 30557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21143);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30558);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 30559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21144);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 21145);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26722);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шалишься.
pub(crate) fn n15240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14429);
	s.is_empty()
}

/// Words ending -блишься.
pub(crate) fn n7941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7597);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14430);
			match s {
				[] => true,
				['к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26723);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26724);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26725);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -глишься.
pub(crate) fn n7942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7598);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 14431);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14432);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21147);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21148);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 21149);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21150);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21151);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26728);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30561);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30562);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30563);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30564);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30565);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32596);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14433);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21152);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14434);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21153);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -длишься.
pub(crate) fn n7943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7599);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14435);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14436);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 21154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26730);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 26731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14437);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21155);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21156);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елишься.
pub(crate) fn n7944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7600);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14438);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21157);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21158);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21159);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21160);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21161);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21162);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26735);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 26736);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 26737);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21163);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21164);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21165);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14439);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21166);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30566);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30567);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32597);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30568);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32598);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30569);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32599);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14440);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21167);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21168);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26740);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 26741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21169);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30570);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32600);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 30571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21170);
					match s {
						[] => true,
						['а', 'р', 'д', s @ ..] => {
							super::push(out, 26742);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21171);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21172);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21173);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21174);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14441);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 21175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14442);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21176);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21177);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21178);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21179);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21180);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21181);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 21182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26749);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21183);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14443);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21184);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 26752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14444);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21185);
					match s {
						[] => true,
						['с', 'ы', s @ ..] => {
							super::push(out, 26753);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30573);
									s.is_empty()
								}
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
			super::push(out, 14445);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21186);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21187);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21188);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21189);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21190);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21191);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30574);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 30575);
									s.is_empty()
								}
								['о', 'с', s @ ..] => {
									super::push(out, 30576);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32601);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21192);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26756);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 30577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26757);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21193);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21194);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26758);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21195);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21196);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t11::n15257(s, out),
		['ц', s @ ..] => super::t11::n15258(s, out),
		['ш', s @ ..] => super::t11::n15259(s, out),
		['щ', s @ ..] => super::t11::n15260(s, out),
		_ => false,
	}
}

/// Words ending -злишься.
pub(crate) fn n7945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7601);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14450);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14451);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -илишься.
pub(crate) fn n7946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7602);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14452);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14453);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21208);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21209);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21210);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14454);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14455);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21213);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21214);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26771);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 26772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 21215);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21216);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21217);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21218);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26774);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21219);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21220);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 21221);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21222);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26778);
							match s {
								[] => true,
								['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									super::push(out, 30582);
									s.is_empty()
								}
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
			super::push(out, 14456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21223);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21224);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14457);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 14458);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21227);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клишься.
pub(crate) fn n7947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7603);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14459);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21229);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 26780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14460);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 21230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -млишься.
pub(crate) fn n7948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7604);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14462);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14463);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 21232);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олишься.
pub(crate) fn n7949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7605);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14464);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21233);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26781);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26782);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14465);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21235);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							super::push(out, 26783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21236);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26784);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26785);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21237);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14466);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21239);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21240);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14467);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14468);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21243);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21244);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21245);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21246);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21247);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26788);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21248);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26789);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14469);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 21249);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 26790);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30586);
									s.is_empty()
								}
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
			super::push(out, 14470);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21250);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21251);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21252);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26791);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26792);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26793);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21253);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26795);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26796);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26797);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26798);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30588);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32604);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 26799);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30589);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21255);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21256);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21257);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14471);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14472);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21259);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21260);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21261);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21262);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21263);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26805);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 26806);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21264);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26808);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 26809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21265);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26810);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26811);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30592);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21266);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14473);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21267);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14474);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21269);
					match s {
						[] => true,
						['н', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30593);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30594);
									s.is_empty()
								}
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
			super::push(out, 14475);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 21270);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плишься.
pub(crate) fn n7950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7606);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14476);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26814);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26815);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26816);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26817);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14477);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 21271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14478);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 14479);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлишься.
pub(crate) fn n7951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7607);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14480);
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26820);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26821);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26822);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30597);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 30598);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26823);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26824);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слишься.
pub(crate) fn n7952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7608);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14481);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26825);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26826);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30600);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 30601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 26827);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 26828);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30603);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26829);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26830);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30605);
									s.is_empty()
								}
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
			super::push(out, 14482);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14483);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21274);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26831);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26832);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 21275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26833);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26834);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26835);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26836);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26837);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26838);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30608);
									s.is_empty()
								}
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
			super::push(out, 14484);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 21276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26839);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26840);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26841);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21277);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26842);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14485);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26844);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26845);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26846);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26847);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26848);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26849);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30610);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32605);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33320);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30611);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32606);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26850);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30612);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 26851);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлишься.
pub(crate) fn n7953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7609);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14486);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14487);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26852);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26853);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 21279);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14488);
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26855);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26856);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26857);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26858);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30615);
									s.is_empty()
								}
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
			super::push(out, 14489);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 14490);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улишься.
pub(crate) fn n7954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14491);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14492);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 14493);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14494);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21283);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21285);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21286);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21287);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14496);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21288);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21289);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14497);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21290);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21291);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26863);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 30616);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21292);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26864);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30617);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30618);
									s.is_empty()
								}
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
			super::push(out, 14498);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21293);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21294);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 14499);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21295);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлишься.
pub(crate) fn n7955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7611);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14500);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14501);
			match s {
				[] => true,
				['ч', '\u{1}', s @ ..] => {
					super::push(out, 21296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14502);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 21297);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26866);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14503);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26868);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шлишься.
pub(crate) fn n7956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7612);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14504);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылишься.
pub(crate) fn n7957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7613);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 14505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21299);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21300);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21301);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21302);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21303);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14506);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21304);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21305);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21306);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21307);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26871);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26872);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30619);
									s.is_empty()
								}
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
			super::push(out, 14507);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26873);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30620);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32607);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30621);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элишься.
pub(crate) fn n7958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7614);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 14508);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юлишься.
pub(crate) fn n7959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7615);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14509);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 14510);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21310);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ялишься.
pub(crate) fn n7960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7616);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14511);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21311);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21312);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21313);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21314);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21315);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21316);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26875);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21317);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21318);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21319);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21320);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21321);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21322);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21323);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вломишься.
pub(crate) fn n22593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21345);
	s.is_empty()
}

/// Words ending -дломишься.
pub(crate) fn n22594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21346);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26895);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еломишься.
pub(crate) fn n22595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21347);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 26897);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 26898);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зломишься.
pub(crate) fn n22596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21348);
	s.is_empty()
}

/// Words ending -оломишься.
pub(crate) fn n22597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21349);
	s.is_empty()
}

/// Words ending -сломишься.
pub(crate) fn n22598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21350);
	s.is_empty()
}

/// Words ending -тломишься.
pub(crate) fn n22599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21351);
	s.is_empty()
}

/// Words ending -ыломишься.
pub(crate) fn n22600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21352);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 26899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -номишься.
pub(crate) fn n15341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14528);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21353);
			match s {
				[] => true,
				['к', 'э', '\u{1}', s @ ..] => {
					super::push(out, 26900);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ромишься.
pub(crate) fn n15342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14529);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 21354);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21355);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26901);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32610);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26902);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -томишься.
pub(crate) fn n15343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14530);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21356);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 21357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рмишься.
pub(crate) fn n7968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7624);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14531);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21358);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26903);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 26904);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26905);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26906);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26907);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30630);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 21359);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тмишься.
pub(crate) fn n7969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7625);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14532);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14533);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -умишься.
pub(crate) fn n7970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7626);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 14534);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21360);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21361);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30631);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32612);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 30632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21362);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14535);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14536);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26910);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26911);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14537);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 14538);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 14539);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ымишься.
pub(crate) fn n7971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7627);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14540);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21366);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21367);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьмишься.
pub(crate) fn n7972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7628);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14541);
			match s {
				[] => true,
				['у', 'с', '\u{1}', s @ ..] => {
					super::push(out, 21369);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юмишься.
pub(crate) fn n7973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7629);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14542);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21370);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21371);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ямишься.
pub(crate) fn n7974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7630);
	match s {
		[] => true,
		['р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21372);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21373);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26915);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26916);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26917);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30634);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21375);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -манишься.
pub(crate) fn n15364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14550);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21389);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21390);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21391);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21392);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 26927);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30645);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 30646);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 30647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26928);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21393);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21394);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26930);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 26931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21395);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 26932);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 30649);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32619);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32620);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 30650);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32622);
									s.is_empty()
								}
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
			super::push(out, 21396);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 21397);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26933);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 21398);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26934);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранишься.
pub(crate) fn n15365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14551);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21399);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21400);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26935);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30652);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30653);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32623);
									s.is_empty()
								}
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
			super::push(out, 21401);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26936);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26937);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26938);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 26939);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21402);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26940);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 21403);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21404);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21405);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21406);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26945);
					match s {
						[] => true,
						['о', 'н', s @ ..] => {
							super::push(out, 30655);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 32624);
									s.is_empty()
								}
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
			super::push(out, 21407);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26946);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -санишься.
pub(crate) fn n15366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14552);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21408);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -танишься.
pub(crate) fn n15367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14553);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21409);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фанишься.
pub(crate) fn n15368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14554);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21410);
			match s {
				[] => true,
				['р', 'о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 26949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щанишься.
pub(crate) fn n15369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14555);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бнишься.
pub(crate) fn n7976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7632);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 14556);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26950);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -внишься.
pub(crate) fn n7977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7633);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14557);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21412);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14558);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26953);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21415);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 26955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гнишься.
pub(crate) fn n7978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7634);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 14559);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21416);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21417);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 21418);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -днишься.
pub(crate) fn n7979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7635);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14560);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14561);
			match s {
				[] => true,
				['б', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 21420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14562);
			match s {
				[] => true,
				['а', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 21421);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14563);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21423);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 21424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26956);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26957);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26958);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26959);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14564);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енишься.
pub(crate) fn n7980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7636);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14565);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21425);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26961);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 26962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21426);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30656);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30657);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30658);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32625);
											s.is_empty()
										}
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
					super::push(out, 21427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14566);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21428);
					s.is_empty()
				}
				['о', 'р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26963);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'е', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26965);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14567);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21429);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30659);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 30660);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21430);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14568);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21431);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14569);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14570);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21434);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21435);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21436);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21437);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26969);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26970);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30662);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30663);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32626);
											s.is_empty()
										}
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
					super::push(out, 21438);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26971);
							s.is_empty()
						}
						['и', 'о', s @ ..] => {
							super::push(out, 26972);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30664);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21439);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21440);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21441);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26974);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26975);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30665);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21442);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 21443);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26976);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26977);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26978);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 26979);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26980);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30666);
									s.is_empty()
								}
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
			super::push(out, 14571);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21444);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26981);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 26982);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 21445);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21446);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21447);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21448);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26983);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30668);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32627);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 30669);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32628);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 30670);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32629);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 30671);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26984);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30672);
									s.is_empty()
								}
								['б', s @ ..] => super::t11::n32541(s, out),
								['о', s @ ..] => super::t11::n32542(s, out),
								['с', s @ ..] => super::t11::n32543(s, out),
								['ы', s @ ..] => super::t11::n32544(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t11::n22702(s, out),
				['и', s @ ..] => super::t11::n22703(s, out),
				['с', s @ ..] => super::t11::n22704(s, out),
				['т', s @ ..] => super::t11::n22705(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t11::n15386(s, out),
		['р', s @ ..] => super::t11::n15387(s, out),
		['с', s @ ..] => super::t11::n15388(s, out),
		['т', s @ ..] => super::t11::n15389(s, out),
		['ц', s @ ..] => super::t11::n15390(s, out),
		['ч', s @ ..] => super::t11::n15391(s, out),
		['щ', s @ ..] => super::t11::n15392(s, out),
		_ => false,
	}
}

/// Words ending -жнишься.
pub(crate) fn n7981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7637);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14579);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14580);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21468);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 21469);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26998);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32637);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 32638);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знишься.
pub(crate) fn n7982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7638);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14581);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21470);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26999);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 21471);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27001);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32639);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32640);
											s.is_empty()
										}
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
					super::push(out, 21472);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27002);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 27003);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30685);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30686);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 30687);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30688);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32641);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14582);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21473);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27004);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30689);
									s.is_empty()
								}
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
			super::push(out, 14583);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21474);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30690);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30691);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30692);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 30693);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30694);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32642);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инишься.
pub(crate) fn n7983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7639);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14584);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21475);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21476);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21477);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21478);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27005);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30695);
									match s {
										[] => true,
										['п', 'у', s @ ..] => {
											super::push(out, 32643);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 27006);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27007);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30696);
									s.is_empty()
								}
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
			super::push(out, 14585);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21479);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 27008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14586);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27009);
							s.is_empty()
						}
						['о', 'с', 'с', s @ ..] => {
							super::push(out, 27010);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30697);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32644);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 27011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14587);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21481);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30698);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30699);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30700);
									s.is_empty()
								}
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
			super::push(out, 14588);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21482);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14589);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21483);
					match s {
						[] => true,
						['ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30701);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30702);
									s.is_empty()
								}
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
			super::push(out, 14590);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21484);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 21485);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21486);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21487);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27016);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 27017);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27018);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30704);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27019);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27020);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30706);
									s.is_empty()
								}
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
			super::push(out, 14591);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21488);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21489);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14592);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21490);
					match s {
						[] => true,
						['щ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30707);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21491);
					match s {
						[] => true,
						['а', 'р', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30709);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30710);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21492);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14593);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 14594);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21495);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21496);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21497);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 21498);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21499);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21500);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21501);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21502);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27025);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32645);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32646);
											s.is_empty()
										}
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
					super::push(out, 21503);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 14595);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21504);
					match s {
						[] => true,
						['о', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30711);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 30712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лнишься.
pub(crate) fn n7984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7640);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14596);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21505);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 21506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27027);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27028);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27029);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27030);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27031);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мнишься.
pub(crate) fn n7985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7641);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14597);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14598);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21507);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14599);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14600);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21508);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21509);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27034);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27035);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27036);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27037);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 27038);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21510);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14601);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21511);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онишься.
pub(crate) fn n7986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7642);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14602);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27040);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27041);
							match s {
								[] => true,
								['р', 'т', s @ ..] => {
									super::push(out, 30715);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 32647);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27042);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27043);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30716);
									s.is_empty()
								}
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
			super::push(out, 14603);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21513);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21514);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21515);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21516);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14604);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21517);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14605);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14606);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21520);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14607);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14608);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27046);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27047);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 27048);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27049);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27050);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30720);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27051);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 27052);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30721);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 27053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21523);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27054);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30722);
									match s {
										[] => true,
										['д', 'о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33321);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 33322);
													s.is_empty()
												}
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
							super::push(out, 27055);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30723);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27056);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21524);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27057);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27058);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 27059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14609);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21525);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27060);
							match s {
								[] => true,
								['е', 'ц', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32648);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32649);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32650);
											s.is_empty()
										}
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
					super::push(out, 21526);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21527);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30727);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 30728);
									s.is_empty()
								}
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
			super::push(out, 14610);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21528);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30729);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30730);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 30731);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32651);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30732);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t11::n15425(s, out),
		['с', s @ ..] => {
			super::push(out, 14612);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21536);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30746);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30747);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32656);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t11::n15427(s, out),
		['ф', s @ ..] => {
			super::push(out, 14614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пнишься.
pub(crate) fn n7987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7643);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14615);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27074);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14616);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнишься.
pub(crate) fn n7988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7644);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14617);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21539);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', '\u{1}', s @ ..] => {
					super::push(out, 21540);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21541);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 21542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27077);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27078);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27079);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27080);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снишься.
pub(crate) fn n7989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7645);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14618);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14619);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27081);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27082);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27083);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27084);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30749);
									s.is_empty()
								}
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
			super::push(out, 14620);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21543);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21544);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27086);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30750);
									s.is_empty()
								}
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
			super::push(out, 14621);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 21545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14622);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21546);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21547);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21548);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21549);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 21550);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27088);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 27089);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30751);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32657);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 30752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21551);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнишься.
pub(crate) fn n7990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7646);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14623);
			match s {
				[] => true,
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27091);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 27092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14624);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21552);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унишься.
pub(crate) fn n7991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7647);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14625);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21553);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14626);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21554);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30753);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30754);
									s.is_empty()
								}
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
			super::push(out, 14627);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14628);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21556);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чнишься.
pub(crate) fn n7992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7648);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14629);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шнишься.
pub(crate) fn n7993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7649);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14631);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 21558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14632);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ынишься.
pub(crate) fn n7994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7650);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14633);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21559);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30755);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30756);
									s.is_empty()
								}
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
			super::push(out, 14634);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21560);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнишься.
pub(crate) fn n7995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7651);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14635);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21561);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14636);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21562);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14637);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21563);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30757);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30758);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 30759);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32658);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 30760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янишься.
pub(crate) fn n7996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7652);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14638);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14639);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21565);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21566);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14640);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 14641);
			match s {
				[] => true,
				['у', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27097);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27098);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 27099);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30761);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32659);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27100);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14642);
			match s {
				[] => true,
				['г', 'а', 'б', 'о', s @ ..] => {
					super::push(out, 21567);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14643);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 14644);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 14645);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27102);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21572);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стоишься.
pub(crate) fn n15485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14671);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21597);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21598);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21599);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21600);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27118);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 30766);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32663);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21601);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21602);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21603);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 21604);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спишься.
pub(crate) fn n8013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7669);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14690);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14691);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 14692);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21633);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упишься.
pub(crate) fn n8014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7670);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 14693);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21634);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21635);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21636);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27146);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30773);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 30774);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27147);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27148);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21637);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21638);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21639);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14694);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21640);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21641);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21642);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 21643);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21644);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21645);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21646);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21647);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21648);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27151);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30776);
									s.is_empty()
								}
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
		['п', s @ ..] => {
			super::push(out, 14695);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21649);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14696);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21650);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21651);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14697);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21652);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21653);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21654);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21655);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япишься.
pub(crate) fn n8016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7672);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14699);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 21656);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одаришься.
pub(crate) fn n22937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21676);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 27172);
			match s {
				[] => true,
				['а', 'л', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32666);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32667);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 27173);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жаришься.
pub(crate) fn n15517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14703);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21679);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21680);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21681);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 21682);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -заришься.
pub(crate) fn n15518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14704);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21683);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21684);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 21685);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27176);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21686);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иаришься.
pub(crate) fn n15519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14705);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 21687);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27177);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27178);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -маришься.
pub(crate) fn n15520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14706);
	s.is_empty()
}

/// Words ending -паришься.
pub(crate) fn n15521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14707);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21688);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21689);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21690);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 21691);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21692);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21693);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27179);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 27180);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 27181);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27182);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21694);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 21695);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 21696);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27184);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27185);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27186);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 21697);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27187);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таришься.
pub(crate) fn n15522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14708);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21698);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21699);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21700);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27188);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27189);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21701);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27190);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 21702);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30786);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 30787);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уаришься.
pub(crate) fn n15523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14709);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 21703);
			match s {
				[] => true,
				['а', 'с', 'к', 'и', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30789);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 30790);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -харишься.
pub(crate) fn n15524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14710);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21704);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30791);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 30792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21705);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27192);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21706);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27193);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30794);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32669);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 30795);
							match s {
								[] => true,
								['а', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -царишься.
pub(crate) fn n15525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14711);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21707);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шаришься.
pub(crate) fn n15526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14712);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21708);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21709);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бришься.
pub(crate) fn n8018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7674);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14713);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21710);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 27194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30796);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 21711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27195);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27196);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 21712);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27198);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27199);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30797);
									s.is_empty()
								}
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
			super::push(out, 14714);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27200);
							s.is_empty()
						}
						['е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30798);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30799);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30800);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30801);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32671);
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
								['\u{1}', s @ ..] => {
									super::push(out, 30802);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30803);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30804);
									s.is_empty()
								}
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
			super::push(out, 14715);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14716);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27201);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27202);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27203);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вришься.
pub(crate) fn n8019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7675);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14717);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21716);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гришься.
pub(crate) fn n8020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7676);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14718);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21717);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21718);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27204);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27205);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 27206);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14719);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21721);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27208);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 27209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дришься.
pub(crate) fn n8021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7677);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14720);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14721);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21723);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21724);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 21725);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27212);
							match s {
								[] => true,
								['а', s @ ..] => {
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
		['з', s @ ..] => {
			super::push(out, 14722);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21726);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14723);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21728);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 27213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21729);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14724);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27215);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14725);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21731);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 21732);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21733);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27218);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27219);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27220);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27221);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30807);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32672);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еришься.
pub(crate) fn n8022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7678);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14726);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21734);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 27222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14727);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21735);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21736);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27225);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27226);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27227);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27228);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30808);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21739);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21740);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21741);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27229);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32673);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32674);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 32675);
											s.is_empty()
										}
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
					super::push(out, 21742);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21743);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14729);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 21744);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14730);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21745);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30809);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 30810);
									s.is_empty()
								}
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
			super::push(out, 14731);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 14732);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21746);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21747);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21748);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27232);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 30811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21749);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21750);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21751);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27233);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21752);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 21753);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27234);
							match s {
								[] => true,
								['о', s @ ..] => {
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
		['н', s @ ..] => {
			super::push(out, 14733);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21754);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 27235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21755);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21756);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21757);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21758);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21759);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14735);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 14736);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21761);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30814);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30815);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 30816);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 30817);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30818);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30819);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32676);
											s.is_empty()
										}
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
					super::push(out, 21762);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30820);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30821);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30822);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 27238);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27239);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 30823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 21763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 14737);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21764);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21765);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21766);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 27241);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30824);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 21767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27242);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 14738);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => super::t11::n15553(s, out),
		['ь', s @ ..] => super::t11::n15554(s, out),
		_ => false,
	}
}

/// Words ending -зришься.
pub(crate) fn n8023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7679);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14741);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 14742);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14743);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21772);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14744);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иришься.
pub(crate) fn n8024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7680);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14745);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 21773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14746);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21774);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 21775);
					match s {
						[] => true,
						['а', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 27246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21776);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14747);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21777);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21778);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27248);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30826);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21779);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21780);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27249);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 27250);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 14748);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21782);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21783);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21784);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кришься.
pub(crate) fn n8025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7681);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14749);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 14750);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21786);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оришься.
pub(crate) fn n8026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7682);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14751);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21787);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14752);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21789);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27254);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27255);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21790);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27256);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30829);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30830);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30831);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30832);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32677);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27257);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30833);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32678);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32679);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32680);
											s.is_empty()
										}
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
					super::push(out, 21791);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27258);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30834);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27259);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27260);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27261);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27262);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30835);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27263);
							match s {
								[] => true,
								['в', 'и', 'ж', '\u{1}', s @ ..] => {
									super::push(out, 30836);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 30837);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32681);
											s.is_empty()
										}
										['о', 'б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33323);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33324);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30838);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27264);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27265);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30839);
									s.is_empty()
								}
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
			super::push(out, 14753);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14754);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21794);
					match s {
						[] => true,
						['з', 'з', s @ ..] => {
							super::push(out, 27266);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14755);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21796);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21797);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27268);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30841);
									s.is_empty()
								}
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
			super::push(out, 14756);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14757);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21799);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21800);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21801);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21802);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 21803);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14758);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21804);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21805);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14759);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 14760);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21806);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27273);
							match s {
								[] => true,
								['с', 'а', s @ ..] => {
									super::push(out, 30842);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32682);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33325);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 30843);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27274);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27275);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30844);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 30845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21808);
					match s {
						[] => true,
						['к', s @ ..] => super::t11::n28915(s, out),
						_ => false,
					}
				}
				['ш', s @ ..] => super::t11::n23070(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t11::n15575(s, out),
		['т', s @ ..] => super::t11::n15576(s, out),
		['ф', s @ ..] => super::t11::n15577(s, out),
		['х', s @ ..] => super::t11::n15578(s, out),
		['ш', s @ ..] => super::t11::n15579(s, out),
		_ => false,
	}
}

/// Words ending -тришься.
pub(crate) fn n8027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7683);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14766);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21826);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27294);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27295);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30851);
									s.is_empty()
								}
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
			super::push(out, 14767);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 21827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14768);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21828);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30852);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30853);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32683);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14769);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 21829);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30854);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30855);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 30856);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32684);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30857);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30858);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32685);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14770);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21830);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27296);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30859);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30860);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 30861);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30862);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32686);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27297);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27298);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 27299);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27300);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21832);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30865);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 30866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уришься.
pub(crate) fn n8028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7684);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14771);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21833);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21834);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21835);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21836);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14772);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21837);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14773);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 14774);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21838);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21839);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14775);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14776);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21842);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21843);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27305);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 27306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 21844);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 21845);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21846);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21847);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21848);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21849);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27311);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21850);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27313);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27314);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21851);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 21852);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27316);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27317);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30867);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32687);
											s.is_empty()
										}
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
					super::push(out, 21853);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14777);
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					super::push(out, 21854);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 21855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27319);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27321);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27322);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27323);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30868);
									s.is_empty()
								}
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
			super::push(out, 14778);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21856);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14779);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21858);
					match s {
						[] => true,
						['ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30869);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30870);
									s.is_empty()
								}
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
			super::push(out, 14780);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21859);
					match s {
						[] => true,
						['к', 'у', 'т', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30871);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30872);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30873);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32688);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 32689);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 30874);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32690);
											s.is_empty()
										}
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
					super::push(out, 21860);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 21861);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27325);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 30875);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 21862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 14781);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 14782);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 14783);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21864);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21865);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21866);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фришься.
pub(crate) fn n8029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7685);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14784);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27326);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хришься.
pub(crate) fn n8030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7686);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14785);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 21868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14786);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27328);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27329);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27330);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30877);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 30878);
									s.is_empty()
								}
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
			super::push(out, 14787);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 21869);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27331);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щришься.
pub(crate) fn n8031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7687);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыришься.
pub(crate) fn n8032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7688);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14789);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21870);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14790);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21871);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21872);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30880);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30881);
									s.is_empty()
								}
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
			super::push(out, 14791);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21873);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21874);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30882);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14792);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21876);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14793);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30884);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30885);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30886);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 27334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юришься.
pub(crate) fn n8033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7689);
	s.is_empty()
}

/// Words ending -яришься.
pub(crate) fn n8034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7690);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14794);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14795);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14796);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 14797);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 21879);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 14798);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

