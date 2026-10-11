use flexion::Path;

/// Words ending -tt.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 432);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3106);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11056);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 22257);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 31688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 11057);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22258);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 22259);
							s.is_empty()
						}
						['l', 'e', 'b', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 22260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11058);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 22261);
							match s {
								[] => true,
								['b', 'u', 'a', 't', s @ ..] => {
									super::push(out, 31689);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 31690);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 38026);
											match s {
												[] => true,
												['a', 'v', 'e', 'l', 'u', 'o', 'b', '\u{1}', s @ ..] => {
													super::push(out, 41817);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 31691);
									s.is_empty()
								}
								['i', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 31692);
									s.is_empty()
								}
								['l', 'e', s @ ..] => {
									super::push(out, 31693);
									match s {
										[] => true,
										['b', 'e', 'i', 'w', 'z', '\u{1}', s @ ..] => {
											super::push(out, 38027);
											s.is_empty()
										}
										['d', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 38028);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 38029);
											s.is_empty()
										}
										['p', 'p', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 38030);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 41818);
													s.is_empty()
												}
												['i', 't', '\u{1}', s @ ..] => {
													super::push(out, 41819);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 31694);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['n', 'a', s @ ..] => {
													super::push(out, 41820);
													match s {
														[] => true,
														['n', s @ ..] => super::t11::n46642(s, out),
														['t', 'a', 'l', 'p', '\u{1}', s @ ..] => super::t11::n46643(s, out),
														_ => false,
													}
												}
												['v', s @ ..] => {
													super::push(out, 41821);
													match s {
														[] => true,
														['a', 'u', 'g', '\u{1}', s @ ..] => super::t11::n46644(s, out),
														['o', s @ ..] => super::t11::n46645(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['e', 'b', 'r', 'o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 38031);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 38032);
											s.is_empty()
										}
										['v', 'l', 'o', 'v', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 38033);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['k', 'c', 'e', 'r', 'd', '\u{1}', s @ ..] => {
											super::push(out, 38034);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 38035);
											match s {
												[] => true,
												['t', 'p', 'y', 'l', 'a', 'k', 'u', 'e', '\u{1}', s @ ..] => {
													super::push(out, 41822);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'p', s @ ..] => {
									super::push(out, 31695);
									s.is_empty()
								}
								['y', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 31696);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 31697);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 38036);
											match s {
												[] => true,
												['e', 'r', 'r', 'u', 'k', 'n', 'o', 'k', '\u{1}', s @ ..] => {
													super::push(out, 41823);
													s.is_empty()
												}
												['i', 'm', 'r', 'e', 'f', 'f', 'e', 'f', 'p', '\u{1}', s @ ..] => {
													super::push(out, 41824);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 38037);
											match s {
												[] => true,
												['e', 'h', '\u{1}', s @ ..] => {
													super::push(out, 41825);
													s.is_empty()
												}
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
							super::push(out, 22262);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11059);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 22263);
							s.is_empty()
						}
						['n', 'e', 'r', s @ ..] => {
							super::push(out, 22264);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 22265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11060);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 22266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t11::n11191(s, out),
				['s', s @ ..] => super::t11::n11192(s, out),
				['t', s @ ..] => super::t11::n11193(s, out),
				['w', s @ ..] => super::t11::n11194(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t11::n3112(s, out),
		['i', s @ ..] => super::t11::n3113(s, out),
		['o', s @ ..] => super::t11::n3114(s, out),
		['u', s @ ..] => super::t11::n3115(s, out),
		['ö', s @ ..] => super::t11::n3116(s, out),
		['ü', s @ ..] => super::t11::n3117(s, out),
		_ => false,
	}
}

/// Words ending -ut.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 433);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3113);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 11105);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 22358);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 31769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 22359);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 31770);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 31771);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38105);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 31772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 22360);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 22361);
							match s {
								[] => true,
								['e', 'ß', s @ ..] => {
									super::push(out, 31773);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 38106);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 22362);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 31774);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 31775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 22363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11106);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 22364);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 31776);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 41867);
													s.is_empty()
												}
												['i', '\u{1}', s @ ..] => {
													super::push(out, 41868);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'g', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 22365);
							s.is_empty()
						}
						['k', 'c', 'a', s @ ..] => {
							super::push(out, 22366);
							match s {
								[] => true,
								['n', 'k', '\u{1}', s @ ..] => {
									super::push(out, 31777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 22367);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 22368);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 22369);
							match s {
								[] => true,
								['e', 't', 't', 'i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 31778);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 31779);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 38107);
											s.is_empty()
										}
										['u', 'n', 'k', '\u{1}', s @ ..] => {
											super::push(out, 38108);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 't', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 31780);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 22370);
							match s {
								[] => true,
								['i', 'e', s @ ..] => {
									super::push(out, 31781);
									match s {
										[] => true,
										['l', 'g', '\u{1}', s @ ..] => {
											super::push(out, 38109);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 31782);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 38110);
											s.is_empty()
										}
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
					super::push(out, 11107);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11108);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 22371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11109);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 22372);
							s.is_empty()
						}
						['k', s @ ..] => super::t11::n23046(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3114);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11110);
					match s {
						[] => true,
						['r', 't', 't', s @ ..] => {
							super::push(out, 22374);
							match s {
								[] => true,
								['a', 'n', s @ ..] => super::t11::n33346(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t11::n11241(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3115);
			s.is_empty()
		}
		['e', s @ ..] => super::t11::n3121(s, out),
		['f', s @ ..] => super::t11::n3122(s, out),
		['g', s @ ..] => super::t11::n3123(s, out),
		['h', s @ ..] => super::t11::n3124(s, out),
		['k', s @ ..] => super::t11::n3125(s, out),
		['l', s @ ..] => super::t12::n3126(s, out),
		['m', s @ ..] => super::t12::n3127(s, out),
		['n', s @ ..] => super::t12::n3128(s, out),
		['o', s @ ..] => super::t12::n3129(s, out),
		['p', s @ ..] => super::t12::n3130(s, out),
		['r', s @ ..] => super::t12::n3131(s, out),
		['t', s @ ..] => super::t12::n3132(s, out),
		['w', s @ ..] => super::t12::n3133(s, out),
		['ä', s @ ..] => super::t12::n3134(s, out),
		_ => false,
	}
}

/// Words ending -xt.
pub(crate) fn n434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 434);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3130);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3131);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 11181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11182);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 22457);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 31824);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 22458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yt.
pub(crate) fn n435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 435);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3132);
			match s {
				[] => true,
				['o', 'n', 'a', 'o', s @ ..] => {
					super::push(out, 11183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3133);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11184);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 22459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11185);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 31825);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 31826);
									match s {
										[] => true,
										['t', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 38135);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'x', '\u{1}', s @ ..] => {
											super::push(out, 38136);
											s.is_empty()
										}
										['o', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 38137);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'e', 'm', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 31827);
									s.is_empty()
								}
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
			super::push(out, 3134);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11186);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 11187);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11188);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 11189);
					match s {
						[] => true,
						['h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 22460);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 22461);
							s.is_empty()
						}
						['r', 't', 'k', 'e', 'l', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31828);
									s.is_empty()
								}
								['t', 's', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 31829);
									s.is_empty()
								}
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
			super::push(out, 3135);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 3136);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 22462);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 31830);
									match s {
										[] => true,
										['n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 38138);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'd', s @ ..] => {
							super::push(out, 22463);
							match s {
								[] => true,
								['n', 'o', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 31831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zt.
pub(crate) fn n436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 436);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 3137);
			match s {
				[] => true,
				['a', 'o', s @ ..] => {
					super::push(out, 11191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3138);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11192);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 22464);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 22465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 11193);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ät.
pub(crate) fn n437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3139);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 11194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3140);
			match s {
				[] => true,
				['r', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3141);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3142);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11196);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 11197);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11198);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 22466);
							match s {
								[] => true,
								['d', 'n', 'a', s @ ..] => {
									super::push(out, 31832);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', 'l', 'y', s @ ..] => {
											super::push(out, 38139);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 38140);
											match s {
												[] => true,
												['i', 'e', 'z', '\u{1}', s @ ..] => {
													super::push(out, 41887);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'e', 'l', 'e', s @ ..] => {
											super::push(out, 38141);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 31833);
									match s {
										[] => true,
										['n', 'u', 'f', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 41888);
													s.is_empty()
												}
												['d', s @ ..] => {
													match s {
														[] => true,
														['n', s @ ..] => super::t12::n46655(s, out),
														['r', 'o', 'b', '\u{1}', s @ ..] => super::t12::n46656(s, out),
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
									super::push(out, 31834);
									match s {
										[] => true,
										['e', 'f', 'a', 't', '\u{1}', s @ ..] => {
											super::push(out, 38142);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 31835);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 41889);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 41890);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'b', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 38143);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['f', 'u', 'l', 'b', 'a', 'n', 'e', 'h', 'c', 'ü', 'k', '\u{1}', s @ ..] => {
											super::push(out, 38144);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 38145);
											s.is_empty()
										}
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
					super::push(out, 11199);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 22467);
							match s {
								[] => true,
								['o', 'p', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', s @ ..] => {
											super::push(out, 38146);
											match s {
												[] => true,
												['t', 'a', 'i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 41891);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 's', s @ ..] => {
											super::push(out, 38147);
											match s {
												[] => true,
												['b', 'l', 'e', 's', '\u{1}', s @ ..] => {
													super::push(out, 41892);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3143);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 11200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t12::n3149(s, out),
		_ => false,
	}
}

/// Words ending -öt.
pub(crate) fn n438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 3145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üt.
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3146);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3147);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3148);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3149);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 11207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -au.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3150);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3151);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11208);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 11209);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11210);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 22474);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 22475);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 22476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 11211);
					match s {
						[] => true,
						['a', s @ ..] => super::t12::n23159(s, out),
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 22477);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 22478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t12::n11343(s, out),
				['e', s @ ..] => super::t12::n11344(s, out),
				['f', s @ ..] => super::t12::n11345(s, out),
				['g', s @ ..] => super::t12::n11346(s, out),
				['h', s @ ..] => super::t12::n11347(s, out),
				['k', s @ ..] => {
					super::push(out, 11217);
					match s {
						[] => true,
						['c', 'ü', s @ ..] => {
							super::push(out, 22492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t12::n11349(s, out),
				['m', s @ ..] => super::t12::n11350(s, out),
				['n', s @ ..] => super::t12::n11351(s, out),
				['r', s @ ..] => super::t12::n11352(s, out),
				['s', s @ ..] => super::t12::n11353(s, out),
				['t', s @ ..] => super::t12::n11354(s, out),
				['u', s @ ..] => {
					super::push(out, 11224);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 11225);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 11226);
					s.is_empty()
				}
				['z', s @ ..] => super::t12::n11358(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t12::n3157(s, out),
		['e', s @ ..] => super::t12::n3158(s, out),
		['f', s @ ..] => super::t12::n3159(s, out),
		['g', s @ ..] => super::t12::n3160(s, out),
		['h', s @ ..] => super::t12::n3161(s, out),
		['i', s @ ..] => super::t12::n3162(s, out),
		['j', s @ ..] => super::t12::n3163(s, out),
		['l', s @ ..] => super::t12::n3164(s, out),
		['m', s @ ..] => super::t12::n3165(s, out),
		['n', s @ ..] => super::t12::n3166(s, out),
		['p', s @ ..] => super::t12::n3167(s, out),
		['r', s @ ..] => super::t12::n3168(s, out),
		['s', s @ ..] => super::t12::n3169(s, out),
		['t', s @ ..] => super::t12::n3170(s, out),
		['u', s @ ..] => super::t12::n3171(s, out),
		['v', s @ ..] => super::t12::n3172(s, out),
		['w', s @ ..] => super::t12::n3173(s, out),
		_ => false,
	}
}

/// The word mannstreu.
pub(crate) fn n44681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41964);
	s.is_empty()
}

/// Words ending -hu.
pub(crate) fn n448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3203);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 11301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3204);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3205);
			match s {
				[] => true,
				['u', 'h', 't', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3206);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3207);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3208);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oju.
pub(crate) fn n3214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3209);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 11303);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ku.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3211);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 11305);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 11306);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3212);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11308);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 11309);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3213);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3214);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3215);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11311);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 22550);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 22551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 11312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3216);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 11313);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lu.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3217);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3218);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3219);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 11314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 11315);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mu.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3220);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3221);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3222);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3223);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3224);
			s.is_empty()
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3225);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nu.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3226);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3227);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 11316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3228);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3229);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3230);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3231);
			match s {
				[] => true,
				['a', 's', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 11319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ou.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3232);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 3233);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3234);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3235);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3236);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3237);
			match s {
				[] => true,
				['n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pu.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	s.is_empty()
}

/// Words ending -ru.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 456);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3239);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 11321);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3240);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3241);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3243);
			match s {
				[] => true,
				['g', 'n', s @ ..] => {
					super::push(out, 11326);
					match s {
						[] => true,
						['ä', 'k', '\u{1}', s @ ..] => {
							super::push(out, 22554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -su.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3244);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3245);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3246);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3247);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3248);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3249);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11328);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 22555);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 22556);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 22557);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 22558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 11329);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11330);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tu.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3250);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3251);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3252);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11331);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11332);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3253);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3254);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 11334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11335);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vu.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 459);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 3255);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wu.
pub(crate) fn n460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 460);
	s.is_empty()
}

/// Words ending -yu.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 461);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 3256);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3257);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3258);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zu.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 462);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3259);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3260);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äu.
pub(crate) fn n463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 463);
	match s {
		[] => true,
		['b', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 3262);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3263);
			match s {
				[] => true,
				['b', 'e', s @ ..] => {
					super::push(out, 11337);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -çu.
pub(crate) fn n464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 464);
	s.is_empty()
}

/// Words ending -ativ.
pub(crate) fn n11476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11345);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 22564);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 22565);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31954);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 31955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 22566);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 31956);
					match s {
						[] => true,
						['l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 38266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 22567);
			match s {
				[] => true,
				['i', 'd', 'ä', s @ ..] => {
					super::push(out, 31957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 22568);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 31958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 22569);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 22570);
			match s {
				[] => true,
				['i', 'm', s @ ..] => {
					match s {
						[] => true,
						['o', 'n', 'e', s @ ..] => {
							super::push(out, 38267);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43771);
											s.is_empty()
										}
										['e', 'r', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 43772);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 38268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 31959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 22571);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 22572);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 31960);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 38269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 31961);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 38270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 31962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 22573);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 38271);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 38272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 22574);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 38273);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 38274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 31963);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 31964);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 31965);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 31966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 22575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ektiv.
pub(crate) fn n23278(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['j', s @ ..] => {
			super::push(out, 31971);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 38275);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 38276);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43773);
									s.is_empty()
								}
								['l', 'a', 'n', s @ ..] => {
									super::push(out, 43774);
									match s {
										[] => true,
										['i', 'm', 'o', 'n', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 44634);
											s.is_empty()
										}
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
					super::push(out, 38277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'l', s @ ..] => {
			super::push(out, 31972);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 31973);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 38278);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntiv.
pub(crate) fn n11479(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22581);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 22582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utiv.
pub(crate) fn n11481(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 22583);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 22584);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lv.
pub(crate) fn n469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 469);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nv.
pub(crate) fn n470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 470);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 3276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rv.
pub(crate) fn n471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 471);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3277);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['h', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 22585);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 22586);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 22587);
							match s {
								[] => true,
								['e', 'n', 'n', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 31974);
									s.is_empty()
								}
								['h', 'c', 'u', 'r', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 31975);
									s.is_empty()
								}
								['k', 'c', 'a', 'm', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 31976);
									s.is_empty()
								}
								['t', 'h', 'c', 'i', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 31977);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 31978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sv.
pub(crate) fn n472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 472);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 3278);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tv.
pub(crate) fn n473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3279);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 3280);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 11349);
					match s {
						[] => true,
						['r', 'a', 'm', 's', '\u{1}', s @ ..] => {
							super::push(out, 22588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uv.
pub(crate) fn n474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 3281);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3282);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wv.
pub(crate) fn n475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 475);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3283);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üv.
pub(crate) fn n476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 3284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11350);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word pneumothorax.
pub(crate) fn n44684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41965);
	s.is_empty()
}

/// Words ending -dex.
pub(crate) fn n3325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3320);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 11385);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 38284);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 38285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 31982);
							match s {
								[] => true,
								['c', 'n', 'a', 'm', 'r', 'o', 'f', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 38286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 41966);
											s.is_empty()
										}
										['t', 'k', 'a', '\u{1}', s @ ..] => {
											super::push(out, 41967);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 38287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 31983);
							match s {
								[] => true,
								['g', 'n', 'u', 'h', 'c', 'e', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 38288);
									s.is_empty()
								}
								['i', 'e', 'r', 'p', 'r', 'e', 'h', 'c', 'u', 'a', 'r', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 38289);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 38290);
									s.is_empty()
								}
								['r', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 38291);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 38292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 31984);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 38293);
									s.is_empty()
								}
								['i', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 38294);
									s.is_empty()
								}
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
			super::push(out, 11386);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 22603);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 22604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31985);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 31986);
							match s {
								[] => true,
								['s', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 38295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'n', 'e', 't', 'l', 'a', 'h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 31987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 22605);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lex.
pub(crate) fn n3328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3323);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11389);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 22607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 11390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22608);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 22609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11391);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11392);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 22611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11393);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 22612);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 22613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 11394);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 22614);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 31988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 22615);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 31989);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 38296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 31990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 22616);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 31991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sex.
pub(crate) fn n3333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3328);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11399);
			s.is_empty()
		}
		['a', 'l', 'l', 'i', 'n', 'a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 11400);
			s.is_empty()
		}
		['i', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 11401);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 31992);
							s.is_empty()
						}
						['r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 31993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 22617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11402);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'm', 'ü', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 31994);
							s.is_empty()
						}
						['p', 'p', 'u', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 31995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 11403);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11404);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 31996);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 31997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tex.
pub(crate) fn n3334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3329);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11405);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 11406);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 22618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11407);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 11408);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11409);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22619);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 22620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11410);
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 22621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11411);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 22622);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ix.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 492);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3330);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11412);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3331);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11414);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 11415);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 22623);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							match s {
								[] => true,
								['b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 31999);
									s.is_empty()
								}
								['n', 'h', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 32000);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 32001);
									match s {
										[] => true,
										['g', 'n', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', 'e', 'n', 'i', 'e', 'l', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 41968);
													s.is_empty()
												}
												['t', 'i', 'e', 'l', 'b', 'a', '\u{1}', s @ ..] => {
													super::push(out, 41969);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'o', 'i', 'x', 'e', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 38297);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'i', 't', 'u', 'n', 'i', 'm', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 32002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 11416);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 22624);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 32003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11417);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 22625);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 32004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', 'r', 'p', 's', s @ ..] => {
					super::push(out, 11418);
					match s {
						[] => true,
						['n', 'o', 'i', 'x', 'e', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 22626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3332);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3333);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 11420);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3334);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 11422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 11423);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 32005);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38298);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 32006);
									s.is_empty()
								}
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
			super::push(out, 3335);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 11424);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 32007);
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 38299);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 38300);
											match s {
												[] => true,
												['e', 'i', 'r', 'd', 'n', 'o', 'h', 'c', 'o', 't', 'i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 41970);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 38301);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 't', 's', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 22628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nx.
pub(crate) fn n493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 493);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3336);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 11425);
					match s {
						[] => true,
						['a', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 22629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3337);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 11427);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3338);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3339);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 11429);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11430);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ox.
pub(crate) fn n494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 494);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3340);
			match s {
				[] => true,
				['-', 'k', s @ ..] => {
					super::push(out, 11431);
					match s {
						[] => true,
						['c', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 22630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'c', s @ ..] => {
					super::push(out, 11432);
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 11433);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 11434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3341);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3342);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3343);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 11437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3344);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 11438);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 22631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3345);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3346);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 3347);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11440);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 11441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ux.
pub(crate) fn n495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 495);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3348);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3349);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3350);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 3351);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 3352);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3353);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11444);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 11445);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 22632);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 22633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 11446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3354);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3355);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11447);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yx.
pub(crate) fn n496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 496);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 3357);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 11449);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22634);
							s.is_empty()
						}
						['d', 'r', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 22635);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 22636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3358);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ey.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 502);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3383);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3384);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3385);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 11472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3386);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11473);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 32012);
									s.is_empty()
								}
								['j', s @ ..] => {
									super::push(out, 32013);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11474);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 22653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 11475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3387);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11476);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11477);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 22654);
							s.is_empty()
						}
						['t', 's', 'n', 'ü', 'k', '\u{1}', s @ ..] => {
							super::push(out, 22655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 11478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 11479);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 22657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11480);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 22658);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 32014);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 32015);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 22659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11481);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 22660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3388);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 11482);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11483);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3389);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 11485);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3390);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 11487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3391);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 11488);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 3392);
			match s {
				[] => true,
				['r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 11490);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lly.
pub(crate) fn n3404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3399);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11494);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32016);
							s.is_empty()
						}
						['d', 'n', 'e', 's', 'e', 'r', 'h', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 32017);
							s.is_empty()
						}
						['s', 'r', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 32018);
							s.is_empty()
						}
						['t', 'k', 'r', 'a', 'm', 'n', 'e', 'r', 'ä', 'b', '\u{1}', s @ ..] => {
							super::push(out, 32019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11495);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11496);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 11497);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 22662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oly.
pub(crate) fn n3405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3400);
	match s {
		[] => true,
		['p', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 11498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -my.
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 507);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 3401);
			s.is_empty()
		}
		['r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ny.
pub(crate) fn n509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 508);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3403);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3404);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11499);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11500);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 22663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3405);
			match s {
				[] => true,
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 11501);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oy.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 509);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3406);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3407);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11502);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 11503);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -py.
pub(crate) fn n511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 510);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3408);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11504);
					s.is_empty()
				}
				['o', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 11505);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11506);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ry.
pub(crate) fn n512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 511);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3409);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11507);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11508);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3410);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3411);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3412);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11510);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3413);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 11512);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11513);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22664);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 22665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3414);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11515);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 22666);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 32020);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 32021);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 22667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 11516);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 22668);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32022);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 32023);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 22669);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32024);
									s.is_empty()
								}
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
			super::push(out, 3415);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11517);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3416);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 11519);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sy.
pub(crate) fn n513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 512);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3417);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11520);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11521);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11522);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 22670);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 22671);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 22672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3418);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11523);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11524);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ty.
pub(crate) fn n514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 513);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3419);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 11525);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3420);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 22673);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 22674);
							match s {
								[] => true,
								['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 32025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11527);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					super::push(out, 11528);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3421);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11530);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 22675);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 22676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3422);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11531);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3423);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3424);
			match s {
				[] => true,
				['a', 'p', 'r', 'e', 'p', s @ ..] => {
					super::push(out, 11533);
					match s {
						[] => true,
						['p', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 22677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3425);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3426);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 11535);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xy.
pub(crate) fn n515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 514);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3427);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11536);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 11537);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11538);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ilz.
pub(crate) fn n3459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3453);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 11562);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 22689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11563);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 11564);
			match s {
				[] => true,
				['ß', s @ ..] => {
					super::push(out, 22691);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 32032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olz.
pub(crate) fn n3460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3454);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 11565);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 22692);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 22693);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 32033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 22694);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 22695);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 32034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'd', s @ ..] => {
					super::push(out, 22696);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 32035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 22697);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 32036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11566);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 22698);
					match s {
						[] => true,
						['e', s @ ..] => {
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

/// Words ending -älz.
pub(crate) fn n3462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3456);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 11568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ölz.
pub(crate) fn n3463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3457);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 11569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nz.
pub(crate) fn n524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 523);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3458);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11570);
					match s {
						[] => true,
						['n', 'u', s @ ..] => {
							super::push(out, 22699);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 32038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 11571);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 22700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 11572);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 22701);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 22702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 11573);
					match s {
						[] => true,
						['c', 's', 'n', 'e', 'm', 'm', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 22703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 11574);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 32039);
									s.is_empty()
								}
								['l', 'a', 'd', s @ ..] => {
									super::push(out, 32040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 11575);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 11576);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 22704);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 32041);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 38306);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 22705);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 32042);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 32043);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 38307);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 22706);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 22707);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 32044);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 32045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11577);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 22708);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 22709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11578);
					match s {
						[] => true,
						['i', 'f', 'h', s @ ..] => {
							super::push(out, 22710);
							s.is_empty()
						}
						['o', 's', 'e', 'r', 'o', s @ ..] => {
							super::push(out, 22711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22712);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 22713);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 22714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11580);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 22715);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 32046);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 32047);
									match s {
										[] => true,
										['o', 't', 'l', s @ ..] => {
											super::push(out, 38308);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 32048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 22716);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 32049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 22717);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 22718);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 32050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 22719);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 22720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 11581);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 22721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t12::n11721(s, out),
				['u', s @ ..] => {
					super::push(out, 11583);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 11584);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 11585);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 22737);
							match s {
								[] => true,
								['c', 's', 's', s @ ..] => {
									super::push(out, 32065);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 38317);
											match s {
												[] => true,
												['n', 'o', 'h', '\u{1}', s @ ..] => {
													super::push(out, 41975);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11586);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 22738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t12::n3465(s, out),
		['i', s @ ..] => {
			super::push(out, 3460);
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 11599);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11600);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 22765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11601);
					match s {
						[] => true,
						['p', 's', s @ ..] => super::t12::n23486(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t12::n11742(s, out),
				_ => false,
			}
		}
		['u', 'l', s @ ..] => super::t12::n3467(s, out),
		['ü', s @ ..] => super::t12::n3468(s, out),
		_ => false,
	}
}

/// Words ending -rz.
pub(crate) fn n525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 524);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3463);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 11605);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 22768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 11606);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 22769);
							match s {
								[] => true,
								['h', 'c', 'u', s @ ..] => {
									super::push(out, 32085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 11607);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 22770);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38337);
											s.is_empty()
										}
										['u', 'a', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 38338);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11608);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 11609);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11610);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 22771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 11611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22772);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 22773);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 32086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 22774);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 22775);
							match s {
								[] => true,
								['l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 32087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 22776);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'u', 'k', 'b', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 38339);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 38340);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 41983);
													s.is_empty()
												}
												['e', 'i', 'r', '\u{1}', s @ ..] => {
													super::push(out, 41984);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 38341);
											s.is_empty()
										}
										['w', 'ö', 'l', '\u{1}', s @ ..] => {
											super::push(out, 38342);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 22777);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 32088);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 22778);
							match s {
								[] => true,
								['e', 'd', 'u', s @ ..] => {
									super::push(out, 32089);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 22779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 11612);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 22780);
							match s {
								[] => true,
								['l', 'b', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 38343);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 38344);
											s.is_empty()
										}
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
					super::push(out, 11613);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 22781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11614);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11615);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 22782);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 38345);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 41985);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 38346);
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 41986);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 38347);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 38348);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 41987);
													s.is_empty()
												}
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
							super::push(out, 22783);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 22784);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 32090);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11616);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 22785);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 22786);
							match s {
								[] => true,
								['s', 'i', 'e', 't', s @ ..] => {
									super::push(out, 32091);
									match s {
										[] => true,
										['e', 'n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 38349);
											s.is_empty()
										}
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
					super::push(out, 11617);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 22787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22788);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 22789);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 3465);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3466);
			s.is_empty()
		}
		['u', s @ ..] => super::t12::n3473(s, out),
		['ä', s @ ..] => {
			super::push(out, 3468);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11623);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 22798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 3469);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 11624);
					match s {
						[] => true,
						['e', 'g', 'l', s @ ..] => {
							super::push(out, 22799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sz.
pub(crate) fn n526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 525);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tz.
pub(crate) fn n527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 526);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3471);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11625);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 22800);
							s.is_empty()
						}
						['m', 'm', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 22801);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 22802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 11626);
					match s {
						[] => true,
						['o', 'k', 'c', 'a', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 22803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 11627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22804);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 22805);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'f', s @ ..] => {
											super::push(out, 38358);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 38359);
											s.is_empty()
										}
										['t', 'r', 'o', 'w', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 41990);
													s.is_empty()
												}
												['b', s @ ..] => {
													super::push(out, 41991);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 41992);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 41993);
													match s {
														[] => true,
														['e', 't', s @ ..] => super::t12::n46715(s, out),
														_ => false,
													}
												}
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
							super::push(out, 22806);
							match s {
								[] => true,
								['e', 'r', 'ä', 'b', '\u{1}', s @ ..] => {
									super::push(out, 32102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 11628);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11629);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 22808);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 22809);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 32103);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 38360);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 32104);
									match s {
										[] => true,
										['u', 'l', 'f', '-', s @ ..] => {
											super::push(out, 38361);
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 41994);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 32105);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22810);
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 22811);
							s.is_empty()
						}
						['n', 'e', 's', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 22812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11631);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 22813);
							match s {
								[] => true,
								['r', 'h', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 32106);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 22814);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 32107);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11632);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 22815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t12::n11773(s, out),
				['w', s @ ..] => {
					super::push(out, 11634);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 22830);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 32120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t12::n3478(s, out),
		['i', s @ ..] => super::t12::n3479(s, out),
		['l', s @ ..] => {
			super::push(out, 3474);
			match s {
				[] => true,
				['a', 'f', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11651);
					s.is_empty()
				}
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 11652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3475);
			match s {
				[] => true,
				['a', 'r', 'k', 'n', 'e', 's', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 11653);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3476);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 11655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 11656);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 11657);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11658);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 22854);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 22855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11659);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 22856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t12::n3483(s, out),
		['u', s @ ..] => super::t12::n3484(s, out),
		['ä', s @ ..] => super::t12::n3485(s, out),
		['ö', s @ ..] => super::t12::n3486(s, out),
		['ü', s @ ..] => super::t12::n3487(s, out),
		_ => false,
	}
}

/// Words ending -uz.
pub(crate) fn n528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 527);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3482);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 11677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3483);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3484);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3485);
			match s {
				[] => true,
				['r', 'k', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 22869);
							match s {
								[] => true,
								['g', 'e', s @ ..] => {
									super::push(out, 32155);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 32156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 22870);
							s.is_empty()
						}
						['n', 'e', 't', s @ ..] => {
							super::push(out, 22871);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 32157);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 22872);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 22873);
							match s {
								[] => true,
								['a', 'l', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 3486);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 11680);
					match s {
						[] => true,
						['k', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 22874);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 22875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zz.
pub(crate) fn n529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 528);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äz.
pub(crate) fn n530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 529);
	s.is_empty()
}

/// Words ending -öz.
pub(crate) fn n531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 530);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3488);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eß.
pub(crate) fn n534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 533);
	match s {
		[] => true,
		['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3499);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3500);
			match s {
				[] => true,
				['r', 'a', 'w', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 11707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3501);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3502);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 11709);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11710);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11711);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 22893);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 32165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'n', s @ ..] => {
							super::push(out, 22894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 11712);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3503);
			s.is_empty()
		}
		['s', 'h', 'c', 'u', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 3504);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3505);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11713);
					match s {
						[] => true,
						['i', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 22896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11714);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iß.
pub(crate) fn n535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3506);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3507);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3508);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 11716);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 11717);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 22897);
							match s {
								[] => true,
								['s', 's', 'g', 'n', 'u', 'r', 'ö', 'w', 'h', 'c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 32166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 22898);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 22899);
							match s {
								[] => true,
								['l', 'u', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 32167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11718);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 22900);
							s.is_empty()
						}
						['h', 'c', 's', 'r', 'e', 'v', 'n', s @ ..] => {
							super::push(out, 22901);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 22902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11719);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 22903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11720);
					match s {
						[] => true,
						['p', 'u', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 22904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11721);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 11722);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22905);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 22906);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 32168);
									match s {
										[] => true,
										['s', 'r', s @ ..] => {
											super::push(out, 38410);
											match s {
												[] => true,
												['e', 'n', s @ ..] => {
													super::push(out, 42023);
													s.is_empty()
												}
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
							super::push(out, 22907);
							match s {
								[] => true,
								['e', 'l', s @ ..] => {
									super::push(out, 32169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 22908);
							match s {
								[] => true,
								['n', 'i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 32170);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 22909);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32171);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 32172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 22910);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 22911);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 22912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3509);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22913);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 22914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3510);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11723);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11724);
					s.is_empty()
				}
				['o', 'r', 'p', 'm', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 11725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3511);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 22915);
							s.is_empty()
						}
						['m', 'a', 'd', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 22916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', s @ ..] => {
					super::push(out, 11726);
					match s {
						[] => true,
						['n', 'e', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32173);
									s.is_empty()
								}
								['r', 'e', '\u{1}', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 3512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oß.
pub(crate) fn n536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3513);
			match s {
				[] => true,
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 11727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3514);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22917);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 22918);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38411);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 38412);
											match s {
												[] => true,
												['c', 'a', 'd', '\u{1}', s @ ..] => {
													super::push(out, 42024);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 'e', 'l', 'b', '\u{1}', s @ ..] => {
											super::push(out, 38413);
											s.is_empty()
										}
										['l', 'e', 't', 'n', 'a', 'm', 'l', s @ ..] => {
											match s {
												[] => true,
												['i', 'e', 't', '\u{1}', s @ ..] => {
													super::push(out, 42025);
													s.is_empty()
												}
												['l', 'o', 'v', '\u{1}', s @ ..] => {
													super::push(out, 42026);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'n', 'o', 'i', 't', 'a', 'm', 'r', 'o', 'f', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 38414);
											s.is_empty()
										}
										['z', 't', 'i', 'p', 's', 'l', 'h', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 38415);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3515);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 11728);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 11729);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 22919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 11730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3516);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 11731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3517);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 11732);
					match s {
						[] => true,
						['e', 'n', 'r', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 22920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 11733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3518);
			match s {
				[] => true,
				['s', 'n', 'e', 'm', s @ ..] => {
					super::push(out, 11734);
					match s {
						[] => true,
						['m', 'a', 's', 'u', 'z', '-', s @ ..] => {
							super::push(out, 22921);
							match s {
								[] => true,
								['e', 'h', 'a', 'n', 'i', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 32175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uß.
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 536);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3519);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 11735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 11736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11738);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', 'g', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 32176);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 32177);
									s.is_empty()
								}
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
			super::push(out, 3520);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11739);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 22925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11740);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11741);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11742);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 22926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 11743);
					match s {
						[] => true,
						['i', 'e', 'g', s @ ..] => {
							super::push(out, 22927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3521);
			match s {
				[] => true,
				['r', 'e', 't', 'u', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 11744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3522);
			match s {
				[] => true,
				['c', 's', 's', 'u', 'a', 's', s @ ..] => {
					super::push(out, 11745);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 22928);
							match s {
								[] => true,
								['r', 'i', 'z', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 32178);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11746);
					s.is_empty()
				}
				['d', 'n', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 11747);
					s.is_empty()
				}
				['e', 'd', 'r', 'e', 'f', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11748);
					s.is_empty()
				}
				['o', 'k', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 11749);
					s.is_empty()
				}
				['s', 'd', 'e', 'i', 'h', 'c', 's', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 11750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3523);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 11751);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 22929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 11752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3524);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3525);
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 11754);
					s.is_empty()
				}
				['t', 'd', 'n', 'a', 'k', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 11755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11756);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 11757);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 22930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 11758);
					match s {
						[] => true,
						['n', 'e', 'n', 'h', s @ ..] => {
							super::push(out, 22931);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 22932);
							match s {
								[] => true,
								['e', 'r', 'h', 'ü', s @ ..] => {
									super::push(out, 32179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11759);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 32180);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 32181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 11760);
					match s {
						[] => true,
						['l', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 22933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üß.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3530);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11762);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11763);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -evia.
pub(crate) fn n3755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3745);
	s.is_empty()
}

/// Words ending -ivia.
pub(crate) fn n3756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3746);
	s.is_empty()
}

/// Words ending -zia.
pub(crate) fn n676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 675);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 3747);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3748);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rka.
pub(crate) fn n703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 702);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3806);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3807);
			match s {
				[] => true,
				['h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3808);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11944);
					s.is_empty()
				}
				['s', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11945);
					s.is_empty()
				}
				['z', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -atka.
pub(crate) fn n3821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3811);
	s.is_empty()
}

/// Words ending -uka.
pub(crate) fn n706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 705);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 3812);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3813);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3814);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wka.
pub(crate) fn n707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 706);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3815);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nella.
pub(crate) fn n12122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11972);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22994);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 22995);
			match s {
				[] => true,
				['r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 32188);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tella.
pub(crate) fn n12124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11974);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 22996);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 32189);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 32190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 22997);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 22998);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -illa.
pub(crate) fn n3852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3842);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11976);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 11977);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 11978);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 11979);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 22999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11980);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23000);
					match s {
						[] => true,
						['m', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 32191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 23001);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 23002);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 32192);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 32193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11981);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 23003);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 23004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 11982);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olla.
pub(crate) fn n3853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3843);
	s.is_empty()
}

/// Words ending -ulla.
pub(crate) fn n3854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3844);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 11983);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11984);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ylla.
pub(crate) fn n3855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3845);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 11985);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ola.
pub(crate) fn n714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 713);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3846);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3847);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 11988);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11989);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11991);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3848);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3849);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3850);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 3851);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3852);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3853);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11995);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 23006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3854);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3855);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11996);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 3857);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 3858);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pla.
pub(crate) fn n715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 714);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 3859);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rla.
pub(crate) fn n716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 715);
	s.is_empty()
}

/// Words ending -sla.
pub(crate) fn n717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 716);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3860);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3861);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ula.
pub(crate) fn n718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 717);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3862);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3863);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3864);
			match s {
				[] => true,
				['i', 'v', 'a', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3865);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3867);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23007);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 23008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3868);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3869);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3870);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 3871);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3872);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 12001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ála.
pub(crate) fn n719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 718);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 3873);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ima.
pub(crate) fn n725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 724);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3897);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 12033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3898);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3899);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 12034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 12035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23015);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 23016);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 23017);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 23018);
							match s {
								[] => true,
								['e', 'k', s @ ..] => {
									super::push(out, 32202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 23019);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 23020);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 32203);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 32204);
									s.is_empty()
								}
								['t', 'f', s @ ..] => {
									super::push(out, 32205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 23021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12036);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 23022);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 32206);
									s.is_empty()
								}
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
			super::push(out, 3900);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3901);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 12037);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3902);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12039);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12040);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3903);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12042);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32207);
									s.is_empty()
								}
								['n', 'e', 'a', 'p', 'e', 't', 'n', 'a', 'e', s @ ..] => {
									super::push(out, 32208);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 38419);
											s.is_empty()
										}
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
					super::push(out, 12043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oma.
pub(crate) fn n728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 727);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3909);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3910);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 12047);
					s.is_empty()
				}
				['y', 'z', '\u{1}', s @ ..] => {
					super::push(out, 12048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3911);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 12049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3912);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 12050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3913);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12051);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12052);
					match s {
						[] => true,
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 23025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3914);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12054);
					s.is_empty()
				}
				['o', 't', 'y', 'z', '\u{1}', s @ ..] => {
					super::push(out, 12055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3915);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 12056);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rma.
pub(crate) fn n729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 728);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3916);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12057);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 23028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 12058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3917);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12059);
					match s {
						[] => true,
						['o', 'r', 'e', 'x', '\u{1}', s @ ..] => {
							super::push(out, 23029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 12060);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 23030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3918);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3919);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 12061);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sma.
pub(crate) fn n730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 729);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['i', 'h', s @ ..] => {
					super::push(out, 12062);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 23031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12063);
					match s {
						[] => true,
						['p', 'o', 'y', s @ ..] => {
							super::push(out, 23032);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 32210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'r', 'h', s @ ..] => {
			super::push(out, 3920);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12064);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uma.
pub(crate) fn n732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 731);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3922);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12065);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32211);
									s.is_empty()
								}
								['n', 'r', 'i', 'h', 'l', 'e', 'd', 'ä', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 32212);
									s.is_empty()
								}
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
			super::push(out, 3923);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12066);
					match s {
						[] => true,
						['u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 23033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3924);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3925);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3926);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12068);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3927);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 12070);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 23034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3928);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3929);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yma.
pub(crate) fn n733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 732);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 3930);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3931);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äma.
pub(crate) fn n734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 733);
	s.is_empty()
}

/// Words ending -wina.
pub(crate) fn n3981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3968);
	s.is_empty()
}

/// Words ending -zina.
pub(crate) fn n3982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3969);
	s.is_empty()
}

/// Words ending -mna.
pub(crate) fn n741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 740);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 3970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nna.
pub(crate) fn n742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 741);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3971);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12108);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12109);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 23045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 12110);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12111);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12112);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 23046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 12113);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3972);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23048);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 23049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3973);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3974);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23050);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 23051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3975);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ona.
pub(crate) fn n743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 742);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 3976);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3977);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3978);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12115);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12116);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rna.
pub(crate) fn n744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 743);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 3979);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3980);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sna.
pub(crate) fn n745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 744);
	s.is_empty()
}

/// Words ending -tna.
pub(crate) fn n746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 745);
	match s {
		[] => true,
		['i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 3981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -una.
pub(crate) fn n747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 746);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3982);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12118);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							super::push(out, 23052);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 23053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12119);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3983);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3984);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 12121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3985);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wna.
pub(crate) fn n748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 747);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3986);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3987);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äna.
pub(crate) fn n749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 748);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3988);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -thra.
pub(crate) fn n4054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4041);
	match s {
		[] => true,
		['e', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23063);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 23064);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ira.
pub(crate) fn n773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 772);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4042);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4043);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12182);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 23065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kra.
pub(crate) fn n774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 773);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4045);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4046);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ora.
pub(crate) fn n778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 777);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4048);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4049);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4050);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 12185);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 32217);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 32218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4051);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12187);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4052);
			match s {
				[] => true,
				['f', 's', s @ ..] => {
					super::push(out, 12189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4053);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4054);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12190);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4055);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4056);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4057);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pra.
pub(crate) fn n779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 778);
	s.is_empty()
}

/// Words ending -rra.
pub(crate) fn n780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 779);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4058);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12192);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 12193);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4059);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4060);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12195);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 12196);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tra.
pub(crate) fn n781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 780);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4061);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4062);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4063);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 12197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4064);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 12198);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4065);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12199);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4066);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12201);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12202);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 23067);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 23068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4067);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 4068);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ura.
pub(crate) fn n782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 781);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4069);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4070);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 12203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4071);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 12204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4072);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4073);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 4074);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4075);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4076);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4077);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 12206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4078);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12207);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23069);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 23070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12208);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wra.
pub(crate) fn n783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 782);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yra.
pub(crate) fn n784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 783);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 4080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ära.
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 784);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 4081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sta.
pub(crate) fn n806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 805);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4156);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12279);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12280);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 23086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 12281);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 23087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4157);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12282);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 23088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 12283);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 12284);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 12285);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 23089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4158);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12286);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 23090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tta.
pub(crate) fn n807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 806);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4159);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12288);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12289);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 12290);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 12291);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4160);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12293);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12294);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4161);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4162);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12297);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23091);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 23092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 12298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uta.
pub(crate) fn n808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 807);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 4163);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4164);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4165);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12299);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hyta.
pub(crate) fn n4180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4166);
	s.is_empty()
}

/// Words ending -zta.
pub(crate) fn n811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 810);
	s.is_empty()
}

/// Words ending -äta.
pub(crate) fn n812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 811);
	s.is_empty()
}

/// Words ending -litbetrieb.
pub(crate) fn n46721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43783);
	s.is_empty()
}

/// Words ending -herwerb.
pub(crate) fn n33866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32245);
	s.is_empty()
}

/// Words ending -serwerb.
pub(crate) fn n33867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32246);
	match s {
		[] => true,
		['g', 'n', 'u', 'r', 'h', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 38438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -terwerb.
pub(crate) fn n33868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32247);
	match s {
		[] => true,
		['o', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 38439);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 38440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orb.
pub(crate) fn n908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 907);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 4292);
			match s {
				[] => true,
				['n', 'e', 'p', s @ ..] => {
					super::push(out, 12367);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 23135);
							s.is_empty()
						}
						['y', 't', '\u{1}', s @ ..] => {
							super::push(out, 23136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12368);
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							super::push(out, 23137);
							s.is_empty()
						}
						['s', 'b', s @ ..] => {
							super::push(out, 23138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urb.
pub(crate) fn n909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 908);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ärb.
pub(crate) fn n910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 909);
	s.is_empty()
}

/// The word aushub.
pub(crate) fn n12551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12385);
	s.is_empty()
}

/// Words ending -lub.
pub(crate) fn n918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 917);
	match s {
		[] => true,
		['c', 't', s @ ..] => {
			super::push(out, 4312);
			match s {
				[] => true,
				['h', 'g', s @ ..] => {
					super::push(out, 12386);
					match s {
						[] => true,
						['i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 23152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pub.
pub(crate) fn n919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 918);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 4313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rub.
pub(crate) fn n920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 919);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4314);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sub.
pub(crate) fn n921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 920);
	s.is_empty()
}

/// The word höhenunterschied.
pub(crate) fn n44764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42043);
	s.is_empty()
}

/// Words ending -massenunterschied.
pub(crate) fn n44765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42044);
	s.is_empty()
}

/// Words ending -lid.
pub(crate) fn n997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 995);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4404);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4405);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4406);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4407);
			match s {
				[] => true,
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 12455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4408);
			match s {
				[] => true,
				['e', 'g', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 12456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4409);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4410);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4411);
			match s {
				[] => true,
				['e', 't', 'n', 'u', 'n', 'e', 'g', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 12458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4412);
			match s {
				[] => true,
				['t', 'e', 'c', 'a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 23200);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 23201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mid.
pub(crate) fn n998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 996);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4413);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12459);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12460);
					s.is_empty()
				}
				['l', 'y', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 23202);
							match s {
								[] => true,
								['t', 'e', 'i', 'd', '-', s @ ..] => {
									super::push(out, 32289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 23203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12461);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12462);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 23204);
							match s {
								[] => true,
								['f', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 32290);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 23205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'e', 'z', s @ ..] => {
					super::push(out, 12463);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 12464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4414);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4415);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 23206);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 32291);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 32292);
									s.is_empty()
								}
								['l', 'y', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 38459);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 38460);
											match s {
												[] => true,
												['e', 'c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 42045);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 32293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 23207);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 32294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nid.
pub(crate) fn n999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 997);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4416);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12465);
					match s {
						[] => true,
						['o', 'h', 'r', 'n', s @ ..] => {
							super::push(out, 23208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23209);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 23210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4417);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12466);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12467);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12468);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							match s {
								[] => true,
								['i', 'd', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 38461);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 38462);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'y', s @ ..] => {
									super::push(out, 32295);
									s.is_empty()
								}
								['m', 'u', 'i', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 38463);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 38464);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4418);
			match s {
				[] => true,
				['m', 'i', 't', 'n', 'a', 'm', s @ ..] => {
					super::push(out, 12469);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oid.
pub(crate) fn n1000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 998);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4419);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4420);
			match s {
				[] => true,
				['r', 'e', 't', 'k', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 12470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4421);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4422);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12472);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 12473);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 23211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12474);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12475);
					match s {
						[] => true,
						['t', 'p', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 23212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12476);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23213);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 23214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12477);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 12478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4423);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12479);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12480);
					match s {
						[] => true,
						['t', 'o', s @ ..] => {
							super::push(out, 23215);
							match s {
								[] => true,
								['r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 32296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12481);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 12482);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 23216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4424);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12483);
					match s {
						[] => true,
						['l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 23217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 12484);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 23218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 12485);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12486);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32297);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 32298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12487);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 23219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4425);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4426);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12489);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 4427);
			match s {
				[] => true,
				['i', 'f', 'f', 'a', s @ ..] => {
					super::push(out, 12491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'e', 'p', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 4428);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -apid.
pub(crate) fn n4449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4429);
	match s {
		[] => true,
		['r', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 12492);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ipid.
pub(crate) fn n4450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4430);
	match s {
		[] => true,
		['l', 'o', 'c', s @ ..] => {
			super::push(out, 12493);
			match s {
				[] => true,
				['y', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 23220);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rid.
pub(crate) fn n1002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1000);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4431);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4432);
			match s {
				[] => true,
				['y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 12495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4433);
			match s {
				[] => true,
				['y', 'h', s @ ..] => {
					match s {
						[] => true,
						['l', 'l', s @ ..] => {
							super::push(out, 23221);
							s.is_empty()
						}
						['m', 'u', 'i', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									super::push(out, 32299);
									s.is_empty()
								}
								['n', 'a', s @ ..] => {
									super::push(out, 32300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'e', s @ ..] => {
							super::push(out, 23222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4434);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12496);
					s.is_empty()
				}
				['t', 'u', 'e', 'd', 'm', s @ ..] => {
					super::push(out, 12497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4435);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12498);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 23223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12499);
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 32301);
									match s {
										[] => true,
										['r', 't', 'e', 't', 'm', s @ ..] => {
											super::push(out, 38465);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 32302);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 32303);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 38466);
											match s {
												[] => true,
												['u', 'ä', 's', 'g', 'i', 's', 's', 'e', '\u{1}', s @ ..] => {
													super::push(out, 42046);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 32304);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 32305);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 32306);
									match s {
										[] => true,
										['u', 'i', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 42047);
													match s {
														[] => true,
														['o', 'c', s @ ..] => super::t12::n46722(s, out),
														_ => false,
													}
												}
												['z', 'l', 'a', 'k', '\u{1}', s @ ..] => {
													super::push(out, 42048);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 32307);
									match s {
										[] => true,
										['e', 's', s @ ..] => {
											super::push(out, 38467);
											match s {
												[] => true,
												['i', 'e', '\u{1}', s @ ..] => {
													super::push(out, 42049);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', 'b', 'l', 'i', 's', 'k', s @ ..] => {
									super::push(out, 32308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12500);
					match s {
						[] => true,
						['l', 'f', s @ ..] => {
							match s {
								[] => true,
								['a', 'x', 'e', 'h', 'n', s @ ..] => {
									super::push(out, 32309);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 32310);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 32311);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 38468);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 38469);
											match s {
												[] => true,
												['i', 'n', s @ ..] => {
													super::push(out, 42050);
													match s {
														[] => true,
														['i', 'm', 'u', 'l', 'a', '\u{1}', s @ ..] => super::t12::n46723(s, out),
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
									super::push(out, 32312);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 38470);
											match s {
												[] => true,
												['m', s @ ..] => {
													super::push(out, 42051);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 32313);
									s.is_empty()
								}
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
			super::push(out, 4436);
			match s {
				[] => true,
				['i', 'n', 'm', s @ ..] => {
					super::push(out, 12501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4437);
			match s {
				[] => true,
				['l', 'l', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 12502);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sid.
pub(crate) fn n1003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1001);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 4438);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4439);
			match s {
				[] => true,
				['i', 'v', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 12503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4440);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4441);
			match s {
				[] => true,
				['a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 12504);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tid.
pub(crate) fn n1004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1002);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4442);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4443);
			match s {
				[] => true,
				['r', 't', 'm', s @ ..] => {
					super::push(out, 12505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4444);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4445);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23224);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 23225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4446);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12506);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 12507);
					match s {
						[] => true,
						['r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 23226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uid.
pub(crate) fn n1005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1003);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 4447);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 4448);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12508);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12509);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vid.
pub(crate) fn n1006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1004);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 4449);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wid.
pub(crate) fn n1007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1005);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4450);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xid.
pub(crate) fn n1008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1006);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 12510);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 12511);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 23228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'f', s @ ..] => {
					super::push(out, 12512);
					match s {
						[] => true,
						['o', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['k', 'c', 'i', 't', 's', 'i', s @ ..] => {
									super::push(out, 32314);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 32315);
									match s {
										[] => true,
										['e', 's', 's', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 38471);
											s.is_empty()
										}
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
					super::push(out, 12513);
					match s {
						[] => true,
						['c', 's', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 23229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12514);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['f', 'f', 'o', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 38472);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 38473);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 32316);
									s.is_empty()
								}
								['m', 'u', 'i', 'z', s @ ..] => {
									super::push(out, 32317);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 23230);
							s.is_empty()
						}
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32318);
									s.is_empty()
								}
								['n', 'e', 's', s @ ..] => {
									super::push(out, 32319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 12515);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23231);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 23232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12516);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23233);
							s.is_empty()
						}
						['l', 'a', 't', 'e', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32320);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 32321);
									s.is_empty()
								}
								['t', 'h', 'c', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 32322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'y', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 23234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12517);
					match s {
						[] => true,
						['u', 'i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 32323);
									match s {
										[] => true,
										['n', 'a', s @ ..] => {
											super::push(out, 38474);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 32324);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 32325);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38475);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 38476);
											match s {
												[] => true,
												['g', 'o', 'r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
													super::push(out, 42052);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 't', 't', s @ ..] => {
									super::push(out, 32326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12518);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23235);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'y', s @ ..] => {
									super::push(out, 32327);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 32328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 23236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 12519);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12520);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23237);
							match s {
								[] => true,
								['y', 'h', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 38477);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 38478);
											match s {
												[] => true,
												['u', 'i', 'r', s @ ..] => {
													super::push(out, 42053);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 38479);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 38480);
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
								['b', s @ ..] => {
									super::push(out, 32329);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 32330);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 32331);
									s.is_empty()
								}
								['p', s @ ..] => super::t12::n33963(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 23238);
							s.is_empty()
						}
						['t', s @ ..] => super::t12::n24006(s, out),
						_ => false,
					}
				}
				['t', 'n', s @ ..] => super::t12::n12700(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zid.
pub(crate) fn n1009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1007);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4451);
			match s {
				[] => true,
				['m', 'u', 'i', 'd', s @ ..] => {
					super::push(out, 12522);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4452);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12524);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32334);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 32335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 12525);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23241);
							s.is_empty()
						}
						['n', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 23242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12526);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23243);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 32336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 23244);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 32337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 23245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'm', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 12527);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12528);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23246);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 23247);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 23248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12529);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 23249);
							match s {
								[] => true,
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 32338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12530);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 23250);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'o', '\u{1}', s @ ..] => {
					super::push(out, 12531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4453);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12532);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 12533);
					match s {
						[] => true,
						['ö', '\u{1}', s @ ..] => {
							super::push(out, 23251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4454);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stenfeld.
pub(crate) fn n40794(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 42055);
			s.is_empty()
		}
		['r', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 42056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utenfeld.
pub(crate) fn n40795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38486);
	s.is_empty()
}

/// The word weizenfeld.
pub(crate) fn n33979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32347);
	s.is_empty()
}

/// Words ending -rnfeld.
pub(crate) fn n24028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23259);
	s.is_empty()
}

/// Words ending -rfeld.
pub(crate) fn n12724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12544);
	match s {
		[] => true,
		['e', 'f', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 23260);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 23261);
			match s {
				[] => true,
				['o', 'r', 'r', 'e', 'k', 'c', 'u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 32348);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sfeld.
pub(crate) fn n12725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12545);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 23262);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 23263);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 32349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 23264);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 23265);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 32350);
					match s {
						[] => true,
						['g', 'n', 'i', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 38487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 32351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 23266);
			match s {
				[] => true,
				['i', 'e', 'k', s @ ..] => {
					super::push(out, 32352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tfeld.
pub(crate) fn n12726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12546);
	match s {
		[] => true,
		['a', 'n', s @ ..] => {
			super::push(out, 23267);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 23268);
			match s {
				[] => true,
				['n', 'g', 'a', 'm', 'd', s @ ..] => {
					super::push(out, 32353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 'a', s @ ..] => {
			super::push(out, 23269);
			match s {
				[] => true,
				['l', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 32354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 23270);
			match s {
				[] => true,
				['u', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 32355);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yfeld.
pub(crate) fn n12727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12547);
	match s {
		[] => true,
		['b', 'g', 'u', 'r', '\u{1}', s @ ..] => {
			super::push(out, 23271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -geld.
pub(crate) fn n4478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4457);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 12548);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 32356);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 38488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', s @ ..] => {
							super::push(out, 32357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 12549);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 23272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 12550);
			match s {
				[] => true,
				['c', 'u', s @ ..] => {
					super::push(out, 23273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 12551);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 12552);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 23275);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 32358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 23276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 12553);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 23277);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 23278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 12554);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 32359);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 38489);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 38490);
									match s {
										[] => true,
										['e', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 42057);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 32360);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 32361);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 32362);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 38491);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 38492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 23279);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32363);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 38493);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 32364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 23280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 12555);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'i', 'k', 't', s @ ..] => {
							super::push(out, 32365);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 32366);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 38494);
									match s {
										[] => true,
										['l', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 42058);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 38495);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'p', s @ ..] => {
							super::push(out, 32367);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 32368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 12556);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 23281);
					match s {
						[] => true,
						['u', 'a', 'l', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 32369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 23282);
					match s {
						[] => true,
						['n', 'u', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 38496);
									match s {
										[] => true,
										['d', 'r', 'o', '\u{1}', s @ ..] => {
											super::push(out, 42059);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 38497);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 23283);
					match s {
						[] => true,
						['h', 'c', 'a', 'n', 'h', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 32370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 12557);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 23284);
					s.is_empty()
				}
				['s', 'e', 'r', s @ ..] => {
					super::push(out, 23285);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 23286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 12558);
			s.is_empty()
		}
		['ß', s @ ..] => {
			super::push(out, 12559);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 23287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -held.
pub(crate) fn n4479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4458);
	s.is_empty()
}

/// Words ending -ield.
pub(crate) fn n4480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4459);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 12560);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -veld.
pub(crate) fn n4481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4460);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12561);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 12562);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 23288);
					match s {
						[] => true,
						['s', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 32371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 12563);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ild.
pub(crate) fn n1012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1010);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4461);
			match s {
				[] => true,
				['d', 'n', 'a', 't', 's', 'r', s @ ..] => {
					super::push(out, 12564);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12565);
					match s {
						[] => true,
						['d', 'm', 'r', 'ä', 'b', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 23289);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 23290);
							s.is_empty()
						}
						['r', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 23291);
							s.is_empty()
						}
						['s', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 23292);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 23293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'e', 's', s @ ..] => {
					super::push(out, 12566);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12567);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 23294);
							match s {
								[] => true,
								['s', 'ü', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 32372);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 23295);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 23296);
							match s {
								[] => true,
								['b', 'm', 'y', 's', '\u{1}', s @ ..] => {
									super::push(out, 32373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12568);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 23297);
							match s {
								[] => true,
								['h', 'o', s @ ..] => {
									super::push(out, 32374);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 23298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12569);
					match s {
						[] => true,
						['e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 23299);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 23300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12570);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12571);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							super::push(out, 23301);
							match s {
								[] => true,
								['h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 32375);
									s.is_empty()
								}
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
			super::push(out, 4462);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4463);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23302);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 23303);
							match s {
								[] => true,
								['n', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 32376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 23304);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32377);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 32378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 23305);
							match s {
								[] => true,
								['f', 'o', 't', 's', 't', 's', 'n', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 32379);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 32380);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 23306);
							match s {
								[] => true,
								['r', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 32381);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 23307);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 38498);
											match s {
												[] => true,
												['p', s @ ..] => {
													super::push(out, 42060);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 38499);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 32382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 23308);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 23309);
							match s {
								[] => true,
								['i', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38500);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 38501);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'n', s @ ..] => {
									super::push(out, 32383);
									match s {
										[] => true,
										['a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 38502);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 23310);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 32384);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 23311);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 32385);
									s.is_empty()
								}
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
			super::push(out, 4464);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 12573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4465);
			match s {
				[] => true,
				['n', 'e', 'l', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 12574);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12575);
					s.is_empty()
				}
				['s', 's', 'o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 12576);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12577);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 12578);
					match s {
						[] => true,
						['r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 23312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', 'o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 12579);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -old.
pub(crate) fn n1013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1011);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4466);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4467);
			match s {
				[] => true,
				['h', 'o', s @ ..] => {
					super::push(out, 12580);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23313);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 32386);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38503);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 23314);
							s.is_empty()
						}
						['o', 't', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 23315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4468);
			match s {
				[] => true,
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 12582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4469);
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 12583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rld.
pub(crate) fn n1014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1012);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 4471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uld.
pub(crate) fn n1015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1013);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4472);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4473);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 23316);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32387);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 32388);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 32389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32390);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 32391);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 32392);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 32393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 23317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4474);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 12584);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -and.
pub(crate) fn n1018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1016);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4477);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12588);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 12589);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12590);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 12591);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 12592);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12593);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 23321);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 23322);
							match s {
								[] => true,
								['n', 'u', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 32395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 12594);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12595);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 23323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 12596);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 32396);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 32397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12597);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 12598);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23324);
							match s {
								[] => true,
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 32398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 23325);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 32399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 23326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32400);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 32401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 23327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12599);
					match s {
						[] => true,
						['r', 'a', 'k', s @ ..] => {
							super::push(out, 23328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12600);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23329);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 32402);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 32403);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 32404);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 32405);
									match s {
										[] => true,
										['l', 'i', 'm', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 38504);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 32406);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38505);
											s.is_empty()
										}
										['l', 'o', 'w', '\u{1}', s @ ..] => {
											super::push(out, 38506);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 32407);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 32408);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 32409);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 32410);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 38507);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 38508);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 23330);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32411);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38509);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 23331);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 23332);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 23333);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12601);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 23334);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 23335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t12::n12787(s, out),
				['r', s @ ..] => super::t12::n12788(s, out),
				['s', s @ ..] => super::t12::n12789(s, out),
				['t', s @ ..] => super::t12::n12790(s, out),
				['y', s @ ..] => super::t12::n12791(s, out),
				['z', s @ ..] => super::t12::n12792(s, out),
				['ß', s @ ..] => super::t12::n12793(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t12::n4499(s, out),
		['g', s @ ..] => super::t12::n4500(s, out),
		['h', s @ ..] => super::t12::n4501(s, out),
		['i', s @ ..] => super::t12::n4502(s, out),
		['k', s @ ..] => super::t12::n4503(s, out),
		['l', s @ ..] => super::t12::n4504(s, out),
		['m', s @ ..] => super::t12::n4505(s, out),
		['r', s @ ..] => super::t12::n4506(s, out),
		['s', s @ ..] => super::t12::n4507(s, out),
		['t', s @ ..] => super::t12::n4508(s, out),
		['w', s @ ..] => super::t12::n4509(s, out),
		_ => false,
	}
}

/// Words ending -kind.
pub(crate) fn n4527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4506);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 12692);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 23453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 12693);
			match s {
				[] => true,
				['e', 'm', 'u', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 23454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 12694);
			match s {
				[] => true,
				['e', 't', 's', s @ ..] => {
					super::push(out, 23455);
					match s {
						[] => true,
						['i', 'w', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 32506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 12695);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 23456);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 23457);
					match s {
						[] => true,
						['o', 'i', 'n', s @ ..] => {
							super::push(out, 32507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 23458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 12696);
			match s {
				[] => true,
				['s', 'i', s @ ..] => {
					super::push(out, 23459);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lspind.
pub(crate) fn n24247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23461);
	s.is_empty()
}

/// Words ending -rind.
pub(crate) fn n4530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4509);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 12698);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 23462);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 23463);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 23464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 12699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wind.
pub(crate) fn n4531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4510);
	match s {
		[] => true,
		['e', 'e', s @ ..] => {
			super::push(out, 12700);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32508);
							s.is_empty()
						}
						['t', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 32509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 12701);
			match s {
				[] => true,
				['e', 'g', 'e', 'g', s @ ..] => {
					super::push(out, 23465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'b', s @ ..] => {
			super::push(out, 12702);
			match s {
				[] => true,
				['m', 'e', s @ ..] => {
					match s {
						[] => true,
						['v', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 32510);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 32511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 12703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ond.
pub(crate) fn n1021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1019);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4511);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 12704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4512);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4513);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12705);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12706);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12707);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12708);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 32512);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 32513);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 32514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12709);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12710);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12711);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4514);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 12712);
					match s {
						[] => true,
						['l', 'i', 'k', 'r', 'e', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 23467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rnd.
pub(crate) fn n1022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1020);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4515);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -und.
pub(crate) fn n1023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1021);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 4516);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 12713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4517);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12714);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 23468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 12715);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12716);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 23469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 12717);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23470);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 23471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12718);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12719);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23472);
							match s {
								[] => true,
								['s', 's', 'ü', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 32515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12720);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12721);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23473);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 32516);
									match s {
										[] => true,
										['r', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 38588);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 23474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12722);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 32517);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 32518);
									match s {
										[] => true,
										['m', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 38589);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 32519);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 38590);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 23475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12723);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 12724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4518);
			match s {
				[] => true,
				['r', 'f', 'n', 'e', 't', s @ ..] => {
					super::push(out, 12725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4519);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12726);
					match s {
						[] => true,
						['b', 'e', s @ ..] => {
							super::push(out, 23477);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 23478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12727);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4520);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12729);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 23479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12730);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23480);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 32520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 23481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 12731);
					s.is_empty()
				}
				['n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 12732);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12733);
					match s {
						[] => true,
						['e', 'd', 'e', s @ ..] => {
							super::push(out, 23482);
							match s {
								[] => true,
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 32521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', 'p', 's', 'n', s @ ..] => {
							super::push(out, 23483);
							match s {
								[] => true,
								['e', 'b', 'm', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 32522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'u', 'k', s @ ..] => {
					super::push(out, 12734);
					s.is_empty()
				}
				['t', 's', 'n', s @ ..] => {
					super::push(out, 12735);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 12736);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 23484);
							s.is_empty()
						}
						['t', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 23485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', 'i', 'e', 'h', s @ ..] => {
					super::push(out, 12737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4521);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4522);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4523);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12740);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12741);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23486);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 23487);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38591);
											s.is_empty()
										}
										['n', 'e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 38592);
											s.is_empty()
										}
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
					super::push(out, 12742);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 23488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4524);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12744);
					match s {
						[] => true,
						['y', 'e', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 23489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12745);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 23490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4525);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 12747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12748);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 12749);
					match s {
						[] => true,
						['l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 23491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 12750);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23492);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							super::push(out, 23493);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 23494);
							match s {
								[] => true,
								['h', 'c', 'u', 'l', 'f', 'h', s @ ..] => {
									super::push(out, 32523);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12751);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12752);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 23495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4527);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4528);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12754);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -änd.
pub(crate) fn n1024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1022);
	s.is_empty()
}

/// Words ending -nmord.
pub(crate) fn n12978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12791);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['s', 'r', s @ ..] => {
					super::push(out, 32527);
					s.is_empty()
				}
				['t', 'r', s @ ..] => {
					super::push(out, 32528);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 32529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 23514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -smord.
pub(crate) fn n12979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12792);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 23515);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 32530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 23516);
			match s {
				[] => true,
				['n', 'u', 'h', 'e', s @ ..] => {
					super::push(out, 32531);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urd.
pub(crate) fn n1044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1042);
	s.is_empty()
}

/// Words ending -ärd.
pub(crate) fn n1045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1043);
	s.is_empty()
}

/// Words ending -iance.
pub(crate) fn n13055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12866);
	match s {
		[] => true,
		['l', 'p', s @ ..] => {
			super::push(out, 23566);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lance.
pub(crate) fn n13056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12867);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', '-', s @ ..] => {
					super::push(out, 32571);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 32572);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mance.
pub(crate) fn n13057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12868);
	s.is_empty()
}

/// Words ending -nance.
pub(crate) fn n13058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12869);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 23567);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rance.
pub(crate) fn n13059(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 23568);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 23569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sance.
pub(crate) fn n13060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12870);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 23570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tance.
pub(crate) fn n13061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12871);
	s.is_empty()
}

/// Words ending -uance.
pub(crate) fn n13062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12872);
	s.is_empty()
}

/// Words ending -ence.
pub(crate) fn n4664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4640);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 12873);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23571);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -once.
pub(crate) fn n4665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4641);
	s.is_empty()
}

/// Words ending -unce.
pub(crate) fn n4666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4642);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 12874);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 23572);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rce.
pub(crate) fn n1082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1080);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 4643);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uce.
pub(crate) fn n1083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1081);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4645);
			match s {
				[] => true,
				['s', 't', s @ ..] => {
					super::push(out, 12876);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4647);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12878);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sfriede.
pub(crate) fn n34230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32578);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 38610);
			match s {
				[] => true,
				['t', 't', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 42109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 38611);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jede.
pub(crate) fn n4690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4666);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 12903);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rede.
pub(crate) fn n4691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4667);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12904);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 23591);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wede.
pub(crate) fn n4692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4668);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 12905);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ide.
pub(crate) fn n1088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1086);
	match s {
		[] => true,
		['a', 'd', s @ ..] => {
			super::push(out, 4669);
			match s {
				[] => true,
				['e', 'b', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 12906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4670);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12907);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23592);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 23593);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 23594);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 32580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12908);
					match s {
						[] => true,
						['k', 'a', s @ ..] => {
							super::push(out, 23595);
							match s {
								[] => true,
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 32581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12909);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 23596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12910);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 23597);
							match s {
								[] => true,
								['r', 'e', 'b', s @ ..] => {
									super::push(out, 32582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 23598);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 32583);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38612);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 32584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12911);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 23599);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 32585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 12912);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23600);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32586);
									s.is_empty()
								}
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
			super::push(out, 4671);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 12913);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 12914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4672);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4673);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12915);
					match s {
						[] => true,
						['r', 't', 's', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 23601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 12916);
					s.is_empty()
				}
				['e', 'h', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 12917);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 23602);
							s.is_empty()
						}
						['y', 'z', '\u{1}', s @ ..] => {
							super::push(out, 23603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12918);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 23604);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 23605);
							match s {
								[] => true,
								['n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 32587);
									s.is_empty()
								}
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
			super::push(out, 4674);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12919);
					match s {
						[] => true,
						['r', 'y', 'p', s @ ..] => {
							match s {
								[] => true,
								['-', 's', s @ ..] => {
									super::push(out, 32588);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 38613);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'p', s @ ..] => {
									super::push(out, 32589);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 38614);
											s.is_empty()
										}
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
					super::push(out, 12920);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 23606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4675);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12921);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 12922);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12923);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 23607);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 32590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4676);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12925);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12926);
					match s {
						[] => true,
						['a', 'r', 't', 's', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 23608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'm', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 12927);
					s.is_empty()
				}
				['p', 'o', 'r', 'h', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 12928);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12929);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23609);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 23610);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 23611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4677);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12930);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4678);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12932);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12933);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12934);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 23612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12935);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12936);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12937);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 23613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4679);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4680);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4681);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 12939);
					s.is_empty()
				}
				['q', 'e', '\u{1}', s @ ..] => {
					super::push(out, 12940);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 4682);
			match s {
				[] => true,
				['a', 'n', 'z', 'a', 'h', 'g', '\u{1}', s @ ..] => {
					super::push(out, 12942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4683);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 4684);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 12944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4685);
			match s {
				[] => true,
				['e', 'y', '\u{1}', s @ ..] => {
					super::push(out, 12945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 4686);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 12946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lde.
pub(crate) fn n1089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1087);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4687);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 12947);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 12948);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 23614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4688);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4689);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12949);
					s.is_empty()
				}
				['f', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 12950);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12951);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12952);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4690);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 12955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4691);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12956);
					match s {
						[] => true,
						['l', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 23616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 4692);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12959);
					match s {
						[] => true,
						['e', 'g', 's', s @ ..] => {
							super::push(out, 23618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 4693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mde.
pub(crate) fn n1090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1088);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4694);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nde.
pub(crate) fn n1091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1089);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4695);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12960);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 23619);
							match s {
								[] => true,
								['e', 'i', s @ ..] => {
									super::push(out, 32591);
									match s {
										[] => true,
										['l', 'i', 'm', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 38615);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 23620);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 32592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 23621);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 32593);
									match s {
										[] => true,
										['f', 'a', 'h', 'c', 's', 'd', 'n', 'u', 'e', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 38616);
											s.is_empty()
										}
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
					super::push(out, 12961);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12962);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 23622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12963);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 23623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 12964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4696);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12965);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 12966);
					match s {
						[] => true,
						['e', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 23624);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 23625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 12967);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 23626);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 23627);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 32594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 23628);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 23629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12968);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 12969);
					match s {
						[] => true,
						['a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 23630);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 23631);
							match s {
								[] => true,
								['a', 'h', 'c', 's', 't', s @ ..] => {
									super::push(out, 32595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 23632);
							s.is_empty()
						}
						['r', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 23633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 12970);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23634);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32596);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 32597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 23635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 12971);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 32598);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 38617);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 38618);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38619);
											match s {
												[] => true,
												['l', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 42110);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 38620);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 23636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23637);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 23638);
							match s {
								[] => true,
								['e', 'd', 's', 'r', 'e', 'd', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 32599);
									s.is_empty()
								}
								['i', 'r', 't', 'r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 32600);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12972);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 23639);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 32601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 23640);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 23641);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 32602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12973);
					match s {
						[] => true,
						['l', 'i', s @ ..] => {
							super::push(out, 23642);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 23643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12974);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23644);
							s.is_empty()
						}
						['r', s @ ..] => super::t12::n24445(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t12::n13167(s, out),
				['r', s @ ..] => super::t12::n13168(s, out),
				['s', s @ ..] => super::t12::n13169(s, out),
				['t', s @ ..] => super::t12::n13170(s, out),
				['u', s @ ..] => {
					super::push(out, 12979);
					s.is_empty()
				}
				['w', s @ ..] => super::t12::n13172(s, out),
				['z', s @ ..] => super::t12::n13173(s, out),
				['ß', s @ ..] => super::t12::n13174(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t12::n4721(s, out),
		['o', s @ ..] => super::t12::n4722(s, out),
		['r', s @ ..] => super::t12::n4723(s, out),
		['u', s @ ..] => super::t12::n4724(s, out),
		['ä', s @ ..] => super::t12::n4725(s, out),
		['ü', s @ ..] => super::t12::n4726(s, out),
		_ => false,
	}
}

/// Words ending -ode.
pub(crate) fn n1092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1090);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4703);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 13001);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23700);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 23701);
							match s {
								[] => true,
								['i', 'c', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 32645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4704);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4705);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13004);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13005);
					match s {
						[] => true,
						['e', 'm', s @ ..] => {
							match s {
								[] => true,
								['-', 'l', s @ ..] => {
									super::push(out, 32646);
									s.is_empty()
								}
								['d', 'i', s @ ..] => {
									super::push(out, 32647);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 32648);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 32649);
									s.is_empty()
								}
								['s', 'g', 'n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38638);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 38639);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4706);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4707);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 13006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4708);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4709);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4710);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4711);
			match s {
				[] => true,
				['i', 'h', s @ ..] => {
					super::push(out, 13007);
					match s {
						[] => true,
						['p', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 23702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4712);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4713);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4714);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 23703);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 23704);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
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
		['w', s @ ..] => {
			super::push(out, 4715);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rde.
pub(crate) fn n1093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1091);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4716);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13009);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							super::push(out, 23705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 13010);
					s.is_empty()
				}
				['h', 'p', s @ ..] => {
					super::push(out, 13011);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 23706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13012);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 13013);
					s.is_empty()
				}
				['n', 'u', s @ ..] => {
					super::push(out, 13014);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13015);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23707);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 23708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 13016);
					match s {
						[] => true,
						['o', 'v', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 23709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4717);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13017);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 13018);
					match s {
						[] => true,
						['g', 'u', s @ ..] => {
							super::push(out, 23710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13019);
					s.is_empty()
				}
				['r', 'u', s @ ..] => {
					super::push(out, 13020);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 13021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4718);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4719);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 13022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 4720);
			match s {
				[] => true,
				['h', 'e', 'b', '-', 'k', s @ ..] => {
					super::push(out, 13023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 4721);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13024);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 13025);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 23711);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32651);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 32652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 't', 'k', 'o', 'd', 'n', s @ ..] => {
							super::push(out, 23712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ude.
pub(crate) fn n1094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1092);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4722);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4723);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4724);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 13026);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'd', s @ ..] => {
									super::push(out, 32653);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 32654);
									s.is_empty()
								}
								['r', 'e', 'k', s @ ..] => {
									super::push(out, 32655);
									s.is_empty()
								}
								['s', 'n', s @ ..] => {
									super::push(out, 32656);
									match s {
										[] => true,
										['e', 'b', s @ ..] => {
											super::push(out, 38640);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4725);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4726);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4727);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4729);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4730);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13030);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 23713);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 23714);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 23715);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 32657);
									s.is_empty()
								}
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
			super::push(out, 4731);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13031);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13032);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yde.
pub(crate) fn n1095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1093);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 4732);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äde.
pub(crate) fn n1096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1094);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 4733);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öde.
pub(crate) fn n1097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1095);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 4734);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4735);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4736);
			s.is_empty()
		}
		['r', 'a', 'h', 't', 'i', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4737);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üde.
pub(crate) fn n1098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1096);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 4738);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4739);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pee.
pub(crate) fn n1109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1107);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4776);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4777);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ree.
pub(crate) fn n1110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4779);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 4780);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4781);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4782);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4783);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4784);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13066);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4785);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13068);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 4786);
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 13070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 4787);
			match s {
				[] => true,
				['p', 's', s @ ..] => {
					super::push(out, 13071);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -see.
pub(crate) fn n1111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1109);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4788);
			match s {
				[] => true,
				['ü', s @ ..] => {
					super::push(out, 13072);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 23729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4789);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 4790);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4791);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 13074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4792);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4793);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4794);
			match s {
				[] => true,
				['p', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 13075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4795);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 23730);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 23731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4796);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13076);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 23732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 13077);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 13078);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 13079);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13080);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13081);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 13082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4797);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4798);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13083);
					s.is_empty()
				}
				['r', 'u', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 13084);
					s.is_empty()
				}
				['u', 'e', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 13085);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tee.
pub(crate) fn n1112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1110);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 4799);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4800);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13086);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4801);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4802);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13088);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13089);
					match s {
						[] => true,
						['o', 'k', 's', 'n', s @ ..] => {
							super::push(out, 23733);
							match s {
								[] => true,
								['o', 'i', 't', 'a', 'r', s @ ..] => {
									super::push(out, 32661);
									s.is_empty()
								}
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
			super::push(out, 4803);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4804);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23734);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 23735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4805);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4806);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4807);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4808);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 4809);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zee.
pub(crate) fn n1114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1112);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4811);
			match s {
				[] => true,
				['d', 'o', 'e', s @ ..] => {
					super::push(out, 13092);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word abbiegehilfe.
pub(crate) fn n40956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38642);
	s.is_empty()
}

/// Words ending -nehilfe.
pub(crate) fn n24555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23750);
	s.is_empty()
}

/// The word dürrehilfe.
pub(crate) fn n24556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23751);
	s.is_empty()
}

/// The word lesehilfe.
pub(crate) fn n24557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23752);
	s.is_empty()
}

/// Words ending -ehhilfe.
pub(crate) fn n13322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13124);
	s.is_empty()
}

/// Words ending -ialhilfe.
pub(crate) fn n13323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13125);
	s.is_empty()
}

/// Words ending -nhilfe.
pub(crate) fn n13324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13126);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'o', s @ ..] => {
					super::push(out, 32666);
					s.is_empty()
				}
				['s', 'o', 'l', 'h', s @ ..] => {
					super::push(out, 32667);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 32668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rhilfe.
pub(crate) fn n13325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13127);
	s.is_empty()
}

/// Words ending -shilfe.
pub(crate) fn n13326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13128);
	match s {
		[] => true,
		['t', 'r', s @ ..] => {
			super::push(out, 23753);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -thilfe.
pub(crate) fn n13327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13129);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 23754);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ülfe.
pub(crate) fn n4850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4823);
	match s {
		[] => true,
		['h', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 13130);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pfe.
pub(crate) fn n1120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1118);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4824);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 13131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4825);
			s.is_empty()
		}
		['m', 'i', s @ ..] => {
			super::push(out, 4826);
			match s {
				[] => true,
				['h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 13132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4827);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 13133);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13134);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rfe.
pub(crate) fn n1121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1119);
	match s {
		[] => true,
		['ä', 'h', 'c', 's', 'd', s @ ..] => {
			super::push(out, 4828);
			match s {
				[] => true,
				['l', 'i', 'w', s @ ..] => {
					super::push(out, 13135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ufe.
pub(crate) fn n1122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1120);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4829);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13136);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13137);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23757);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 32669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13138);
					match s {
						[] => true,
						['t', 'l', s @ ..] => {
							super::push(out, 23758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13139);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 23759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4830);
			match s {
				[] => true,
				['t', 'b', s @ ..] => {
					super::push(out, 13140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4831);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rage.
pub(crate) fn n4868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4841);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13166);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13167);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 23773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 13168);
			match s {
				[] => true,
				['-', 'a', s @ ..] => {
					super::push(out, 23774);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 23775);
					s.is_empty()
				}
				['d', 'n', 'a', s @ ..] => {
					super::push(out, 23776);
					match s {
						[] => true,
						['l', 'h', s @ ..] => {
							super::push(out, 32673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'd', s @ ..] => {
					super::push(out, 23777);
					s.is_empty()
				}
				['r', 'i', s @ ..] => {
					super::push(out, 23778);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['g', 'n', 'i', s @ ..] => {
							super::push(out, 32674);
							match s {
								[] => true,
								['l', 't', s @ ..] => {
									super::push(out, 38644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', s @ ..] => {
							super::push(out, 32675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13169);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13170);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 23779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 13171);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13172);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 23780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 23781);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 23782);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 32677);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 32678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ssage.
pub(crate) fn n13372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13174);
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			super::push(out, 23784);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 32679);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 32680);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 32681);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 38645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 23785);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 32682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 23786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tage.
pub(crate) fn n4870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4843);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13175);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13176);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 13177);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23787);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 32683);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 32684);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 38646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 23788);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 32685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 23789);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 38647);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 38648);
									s.is_empty()
								}
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
			super::push(out, 13178);
			match s {
				[] => true,
				['b', 'a', 'k', s @ ..] => {
					super::push(out, 23790);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 23791);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 23792);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 32686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13179);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13180);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23793);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 23794);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 23795);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 23796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 13181);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23797);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 23798);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dge.
pub(crate) fn n1124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1122);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4846);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23799);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 23800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4847);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 23801);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 23802);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 23803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ege.
pub(crate) fn n1125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1123);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 4849);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4850);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4851);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13185);
					match s {
						[] => true,
						['f', 'b', s @ ..] => {
							super::push(out, 23804);
							match s {
								[] => true,
								['e', 'w', 'h', 'c', 's', 'n', s @ ..] => {
									super::push(out, 32687);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13186);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 23805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4852);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13187);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 23806);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 23807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 13188);
					match s {
						[] => true,
						['p', 'r', s @ ..] => {
							super::push(out, 23808);
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									super::push(out, 32688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13189);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 23809);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 32689);
									s.is_empty()
								}
								['k', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 4853);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4854);
			match s {
				[] => true,
				['o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 13190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4855);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13191);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gge.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1124);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4856);
			s.is_empty()
		}
		['e', 'w', s @ ..] => {
			super::push(out, 4857);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4858);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 13193);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 23810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4859);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4860);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13194);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hge.
pub(crate) fn n1127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1125);
	s.is_empty()
}

/// Words ending -ige.
pub(crate) fn n1128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1126);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4861);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4862);
			match s {
				[] => true,
				['n', 'ä', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['b', 'l', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 23811);
							s.is_empty()
						}
						['t', 's', 'b', 'l', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 23812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4863);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13195);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 13196);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 13197);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 23813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 13198);
					match s {
						[] => true,
						['n', 'a', 'd', s @ ..] => {
							super::push(out, 23814);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 32691);
									s.is_empty()
								}
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
			super::push(out, 4864);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4865);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4866);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 13199);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 23815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4867);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13200);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13201);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 23816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 13202);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4868);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13203);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13204);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 38649);
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 42115);
													match s {
														[] => true,
														['p', 'r', 'a', 'k', '\u{1}', s @ ..] => super::t12::n46749(s, out),
														_ => false,
													}
												}
												['p', s @ ..] => {
													super::push(out, 42116);
													match s {
														[] => true,
														['s', 'e', 'w', 'f', 'p', 'u', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => super::t12::n46750(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 38650);
											match s {
												[] => true,
												['a', 'f', 'l', s @ ..] => {
													super::push(out, 42117);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 32692);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['l', s @ ..] => {
													match s {
														[] => true,
														['g', s @ ..] => super::t12::n46751(s, out),
														['t', s @ ..] => super::t12::n46752(s, out),
														_ => false,
													}
												}
												['w', s @ ..] => {
													super::push(out, 42118);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ß', s @ ..] => {
									super::push(out, 32693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13205);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23819);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 32694);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'l', 'u', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 23820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lge.
pub(crate) fn n1129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1127);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4869);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4870);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13206);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4871);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23822);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32695);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 23823);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 23824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4872);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nge.
pub(crate) fn n1130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1128);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4873);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13208);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13209);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13210);
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							super::push(out, 23825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13211);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13212);
					match s {
						[] => true,
						['ß', 'e', s @ ..] => {
							super::push(out, 23826);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 32696);
									s.is_empty()
								}
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
			super::push(out, 4874);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13213);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 23827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13214);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23828);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13215);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23829);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 23830);
							match s {
								[] => true,
								['s', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 32698);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 23831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4875);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13217);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23833);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 23834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13218);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 23835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13219);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4876);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13221);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 13222);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 23837);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 32699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 23838);
							match s {
								[] => true,
								['l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 32700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 23839);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['s', 's', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 38651);
											s.is_empty()
										}
										['ß', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 38652);
											s.is_empty()
										}
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
					super::push(out, 13223);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 23840);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 32701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13224);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 23841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13225);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 23842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 13226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 4877);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13227);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13228);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 23844);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 32702);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 23845);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 32703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13229);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23846);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 23847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13230);
					match s {
						[] => true,
						['s', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 23848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oge.
pub(crate) fn n1131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1129);
	match s {
		[] => true,
		['g', 'a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13231);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13232);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 23849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4878);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13233);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 13235);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13236);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 23852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13237);
					match s {
						[] => true,
						['h', 't', s @ ..] => {
							super::push(out, 23853);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32704);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 38653);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 23854);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 32705);
									match s {
										[] => true,
										['y', 'h', 'p', 's', s @ ..] => {
											super::push(out, 38654);
											match s {
												[] => true,
												['e', 'n', 'n', 'i', 's', '\u{1}', s @ ..] => {
													super::push(out, 42119);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'i', s @ ..] => {
							super::push(out, 23855);
							match s {
								[] => true,
								['r', 'k', 'o', 'd', 'n', 'e', 'o', s @ ..] => {
									super::push(out, 32706);
									match s {
										[] => true,
										['r', 'u', 'e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 38655);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', s @ ..] => {
							super::push(out, 23856);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 32707);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13238);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 4879);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4880);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4881);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13240);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rge.
pub(crate) fn n1132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1130);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4882);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4883);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13241);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 13242);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13243);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 23858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13244);
					match s {
						[] => true,
						['c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 23859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13245);
					match s {
						[] => true,
						['h', 'c', 's', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 23860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4884);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13248);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 32708);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38656);
											s.is_empty()
										}
										['e', 'k', s @ ..] => {
											super::push(out, 38657);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									super::push(out, 32709);
									s.is_empty()
								}
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
			super::push(out, 4885);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 23862);
							s.is_empty()
						}
						['r', 'ü', 'f', 'n', s @ ..] => {
							super::push(out, 23863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 4886);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13249);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uge.
pub(crate) fn n1133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1131);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4887);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23864);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 23865);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 23866);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 23867);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 23868);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 23869);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32710);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 32711);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 23870);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 23871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13251);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4888);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13253);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13254);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13255);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 23872);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 32712);
									s.is_empty()
								}
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
			super::push(out, 4889);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4890);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 13256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4891);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 13257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 4892);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13258);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äge.
pub(crate) fn n1134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1132);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 4893);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 13259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4894);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ège.
pub(crate) fn n1135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1133);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 4895);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üge.
pub(crate) fn n1136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1134);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 4897);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4898);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13261);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13262);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13263);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4899);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4900);
			s.is_empty()
		}
		_ => false,
	}
}

