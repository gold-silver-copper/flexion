use flexion::Path;

/// Words ending -tto.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2283);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3777);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', 'r', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['m', 'o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['p', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3778);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 5271);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 5441);
													match s {
														[] => true,
														['s', s @ ..] => super::t2::n8225(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2285);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3779);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4799);
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
		_ => false,
	}
}

/// Words ending -uto.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['c', 'e', 'x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 930);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 932);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 933);
			match s {
				[] => true,
				['m', 'i', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 934);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['m', 'a', s @ ..] => {
					super::push(out, 2292);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3782);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xto.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 936);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -æto.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 937);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2294);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -instinguo.
pub(crate) fn n8089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5442);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fluo.
pub(crate) fn n1006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 951);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2308);
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2309);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2310);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3792);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4804);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3793);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2311);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4806);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4807);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5273);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3797);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4808);
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
		['m', s @ ..] => {
			super::push(out, 2312);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3798);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2313);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3799);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3800);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2314);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3801);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2315);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3802);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4811);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5274);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5275);
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
						['t', s @ ..] => {
							super::push(out, 4812);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5276);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 5277);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5278);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5443);
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
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2316);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3803);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cumluo.
pub(crate) fn n4883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3805);
	match s {
		[] => true,
		['r', 'i', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pluo.
pub(crate) fn n1011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 956);
	s.is_empty()
}

/// Words ending -rluo.
pub(crate) fn n1012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 957);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2321);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nuo.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 958);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 962);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2327);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2328);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2329);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 3808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3809);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3810);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -puo.
pub(crate) fn n219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 965);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2332);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -quo.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 966);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2334);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 3812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4815);
									s.is_empty()
								}
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
			super::push(out, 969);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2335);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3813);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3814);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4816);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3815);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 4817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3816);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4818);
									s.is_empty()
								}
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
			super::push(out, 970);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2336);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ruo.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 971);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 972);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2337);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3817);
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
				['o', s @ ..] => {
					super::push(out, 2338);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3818);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4819);
									match s {
										[] => true,
										['p', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 973);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2339);
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
		['g', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 977);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 978);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 979);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2344);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2345);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3820);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3821);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3822);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4820);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 5279);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4821);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5280);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4822);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3823);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5444);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 5445);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 5281);
											match s {
												[] => true,
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2346);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -suo.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 218);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 980);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 981);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 982);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tuo.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2348);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['e', 'f', 'f', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'r', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 984);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2350);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2351);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2352);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2353);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2354);
					match s {
						[] => true,
						['s', 'a', s @ ..] => {
							super::push(out, 3830);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4823);
									s.is_empty()
								}
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
			super::push(out, 986);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2355);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4824);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4825);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5282);
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
								['d', s @ ..] => {
									super::push(out, 4826);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5283);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4827);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5284);
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
								['x', s @ ..] => {
									super::push(out, 4828);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5285);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 987);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2356);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 988);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2357);
					s.is_empty()
				}
				['m', 'o', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2358);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xuo.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 989);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word subvolvo.
pub(crate) fn n8099(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word inservo.
pub(crate) fn n4932(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word perservo.
pub(crate) fn n4933(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -urvo.
pub(crate) fn n1062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1004);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2380);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2381);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uvo.
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1005);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2382);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2383);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word comprecor.
pub(crate) fn n8100(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -icor.
pub(crate) fn n1084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1021);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 2400);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2401);
			match s {
				[] => true,
				['r', 'o', 'c', s @ ..] => {
					super::push(out, 3855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2402);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 4837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2403);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3856);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2404);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3857);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4839);
							match s {
								[] => true,
								['a', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ocor.
pub(crate) fn n1085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1022);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2405);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2406);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3858);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rcor.
pub(crate) fn n1086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1023);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2407);
			match s {
				[] => true,
				['m', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', 'o', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -scor.
pub(crate) fn n1087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1024);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2408);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4840);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4841);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3860);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3861);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2409);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 3862);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2410);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3863);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4843);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5290);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4844);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5291);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4845);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 5292);
									match s {
										[] => true,
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4846);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 5293);
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
						['n', s @ ..] => {
							super::push(out, 4847);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5294);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3864);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4848);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5295);
									match s {
										[] => true,
										['p', 'x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3865);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4849);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 5296);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 5446);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 5482);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8251(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['m', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4850);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5297);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3866);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4851);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4852);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5298);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 4853);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t2::n4987(s, out),
				['t', s @ ..] => super::t2::n4988(s, out),
				['v', s @ ..] => super::t2::n4989(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dor.
pub(crate) fn n239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 234);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1025);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1026);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1027);
			match s {
				[] => true,
				['u', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3870);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4857);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2411);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3872);
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
				['o', s @ ..] => {
					super::push(out, 2412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2413);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3873);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eor.
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 235);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1031);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2416);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2417);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2418);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3875);
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1033);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2419);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2420);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3877);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 4858);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5303);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4859);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4860);
									s.is_empty()
								}
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
			super::push(out, 1034);
			match s {
				[] => true,
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2421);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3878);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3879);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -for.
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 236);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1035);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gor.
pub(crate) fn n242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2422);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3881);
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
				['u', s @ ..] => {
					super::push(out, 2423);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3882);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hor.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1040);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 2424);
					match s {
						[] => true,
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1042);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ior.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2425);
					s.is_empty()
				}
				['r', 'e', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1044);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2426);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3883);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 4863);
									match s {
										[] => true,
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3884);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 4864);
									match s {
										[] => true,
										['o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2427);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3885);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3886);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['a', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['d', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', 'n', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 5304);
											match s {
												[] => true,
												['r', 't', 'n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['p', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', s @ ..] => {
													match s {
														[] => true,
														['e', 'a', 'r', 'p', '\u{1}', s @ ..] => super::t2::n8229(s, out),
														['n', 'i', '\u{1}', s @ ..] => super::t2::n8230(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2428);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2429);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3888);
							match s {
								[] => true,
								['l', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5305);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 5306);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 5307);
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
				['r', s @ ..] => {
					super::push(out, 2430);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3889);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1045);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2431);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n1109(s, out),
		['n', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2435);
					match s {
						[] => true,
						['u', 'l', 'a', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n1111(s, out),
		['t', s @ ..] => super::t2::n1112(s, out),
		['v', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2445);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lor.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 240);
	match s {
		[] => true,
		['a', 'p', 's', 'i', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', 'g', 'u', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					super::push(out, 2446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'g', 'u', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 't', s @ ..] => {
			super::push(out, 1051);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1052);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3920);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3921);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['n', 'a', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'e', 'g', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 4893);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5317);
											match s {
												[] => true,
												['n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'o', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3922);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 4894);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2447);
					match s {
						[] => true,
						['o', 'm', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', 'a', 'b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'a', 'r', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2448);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3923);
							match s {
								[] => true,
								['r', 'g', 'd', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3924);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4895);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 4896);
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
		_ => false,
	}
}

/// Words ending -nor.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 242);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1054);
			s.is_empty()
		}
		['g', 'i', 'd', s @ ..] => {
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2450);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 't', 'a', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'u', 'b', 'u', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['b', 'l', 'a', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'l', 'a', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2451);
					s.is_empty()
				}
				['p', 'o', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2452);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3925);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4897);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2453);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3926);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4898);
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
		['o', s @ ..] => {
			super::push(out, 1056);
			match s {
				[] => true,
				['c', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'r', 'e', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'u', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -por.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['s', 'u', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ror.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1057);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2454);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3927);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4899);
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
						['o', s @ ..] => {
							super::push(out, 3928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2455);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3929);
							match s {
								[] => true,
								['e', 't', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', 'i', 'r', 'o', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2456);
					match s {
						[] => true,
						['e', 'v', 'e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 't', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2457);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2458);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2459);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 3930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1059);
			match s {
				[] => true,
				['m', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1060);
			match s {
				[] => true,
				['m', 'e', s @ ..] => {
					super::push(out, 2460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'u', 'c', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1061);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1062);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2461);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sor.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1063);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tor.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['a', 'r', 'g', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1064);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2462);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3931);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4901);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4902);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4903);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 5318);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2463);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3933);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4904);
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
				['u', s @ ..] => {
					super::push(out, 2464);
					match s {
						[] => true,
						['l', 'b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1066);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2465);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3934);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 3935);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2467);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2468);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3936);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2469);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3938);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4906);
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
						['d', s @ ..] => {
							super::push(out, 3939);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4907);
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
						['e', s @ ..] => {
							super::push(out, 3940);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4908);
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
						['i', s @ ..] => {
							super::push(out, 3941);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3942);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4910);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4911);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3943);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4913);
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
				['p', s @ ..] => {
					super::push(out, 2470);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2471);
					match s {
						[] => true,
						['g', 'i', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1067);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1068);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2473);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 3944);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4914);
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
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['e', 't', 'n', 'a', s @ ..] => {
					super::push(out, 2475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1070);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2476);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2477);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2478);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2479);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2480);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2481);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2482);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2483);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uor.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1071);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2484);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2485);
					match s {
						[] => true,
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3946);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1072);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 1073);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2488);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2489);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3949);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4915);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4916);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5319);
											s.is_empty()
										}
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
					super::push(out, 2490);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1074);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3951);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3952);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4917);
									s.is_empty()
								}
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
			super::push(out, 1075);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2492);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2493);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 3953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2494);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vor.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1076);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fico.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['e', 'l', 'a', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', 'o', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2552);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3982);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3983);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4924);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5323);
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
								['e', 'l', 'p', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2553);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3985);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4925);
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
				['l', s @ ..] => {
					super::push(out, 2554);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3986);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4926);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 5324);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2555);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3987);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3988);
							match s {
								[] => true,
								['a', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['x', s @ ..] => {
											super::push(out, 5325);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 5450);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8233(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'r', 'u', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2556);
					match s {
						[] => true,
						['c', 'a', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3989);
							match s {
								[] => true,
								['c', 'e', 'l', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4927);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5326);
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
				['s', s @ ..] => {
					match s {
						[] => true,
						['l', 'a', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2557);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3990);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4928);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5327);
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
						['e', s @ ..] => {
							super::push(out, 3991);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 4929);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'u', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'e', 'a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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

/// Words ending -emico.
pub(crate) fn n1245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1145);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2561);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2562);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -imico.
pub(crate) fn n1246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1146);
	s.is_empty()
}

/// Words ending -rmico.
pub(crate) fn n1247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1147);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2563);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word pumico.
pub(crate) fn n1248(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -nico.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3994);
							match s {
								[] => true,
								['o', 'c', 'x', s @ ..] => {
									super::push(out, 4930);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 5328);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word pico.
pub(crate) fn n319(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rico.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1148);
			match s {
				[] => true,
				['l', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3995);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1149);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2565);
					match s {
						[] => true,
						['n', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 3997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2566);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3998);
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
				['u', s @ ..] => {
					super::push(out, 2567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1150);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2568);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2569);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3999);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2570);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4001);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4002);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2571);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2572);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4004);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2573);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 4005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1151);
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					super::push(out, 2574);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4006);
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
				['t', 'c', 'u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1152);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2575);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sico.
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 2576);
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
		['r', 'o', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tico.
pub(crate) fn n322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 315);
	match s {
		[] => true,
		['n', 'e', s @ ..] => {
			match s {
				[] => true,
				['h', 't', 'u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1154);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2577);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word toxico.
pub(crate) fn n323(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -esco.
pub(crate) fn n339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 330);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1174);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2588);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4013);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
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
						['h', s @ ..] => {
							super::push(out, 4014);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2589);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4936);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 4937);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5330);
											s.is_empty()
										}
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
					super::push(out, 2590);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4016);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2591);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4017);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4018);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4019);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4939);
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
			super::push(out, 1175);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2592);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4020);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2593);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4021);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4022);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2594);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4023);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4024);
							match s {
								[] => true,
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2595);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2596);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2597);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4943);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2598);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4026);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4945);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5331);
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
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 4946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4947);
									s.is_empty()
								}
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
			super::push(out, 1176);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2599);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4028);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4029);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4948);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4949);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5332);
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
								['m', s @ ..] => {
									super::push(out, 4950);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5333);
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
				['i', s @ ..] => {
					super::push(out, 2600);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4030);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t2::n3084(s, out),
				['r', s @ ..] => super::t2::n3085(s, out),
				['u', s @ ..] => super::t2::n3086(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1177);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t2::n1288(s, out),
		['i', s @ ..] => super::t2::n1289(s, out),
		['l', s @ ..] => super::t2::n1290(s, out),
		['m', s @ ..] => super::t2::n1291(s, out),
		['n', s @ ..] => super::t2::n1292(s, out),
		['p', s @ ..] => super::t2::n1293(s, out),
		['r', s @ ..] => super::t2::n1294(s, out),
		['s', s @ ..] => super::t2::n1295(s, out),
		['t', s @ ..] => super::t2::n1296(s, out),
		['u', s @ ..] => super::t2::n1297(s, out),
		['v', s @ ..] => super::t3::n1298(s, out),
		_ => false,
	}
}

/// Words ending -rcisco.
pub(crate) fn n3141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2658);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -scisco.
pub(crate) fn n3142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2659);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4136);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4137);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5027);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5028);
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
		['n', s @ ..] => {
			super::push(out, 4138);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4139);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5029);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word lucisco.
pub(crate) fn n5375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4140);
	s.is_empty()
}

/// Words ending -disco.
pub(crate) fn n1300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1190);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2661);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2662);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2663);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2664);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2665);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4141);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fisco.
pub(crate) fn n1301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1191);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 2666);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4142);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gisco.
pub(crate) fn n1302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1192);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 2667);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hisco.
pub(crate) fn n1303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1193);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2668);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4143);
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
		['m', s @ ..] => {
			super::push(out, 2669);
			match s {
				[] => true,
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lisco.
pub(crate) fn n1304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1194);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 2670);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2671);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4144);
					match s {
						[] => true,
						['p', 'a', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -misco.
pub(crate) fn n1305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1195);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2672);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4145);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5030);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5031);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2673);
			match s {
				[] => true,
				['o', 'd', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5032);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 5033);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5034);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5035);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nisco.
pub(crate) fn n1306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1196);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2674);
			match s {
				[] => true,
				['m', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4146);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4147);
					match s {
						[] => true,
						['q', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pisco.
pub(crate) fn n1307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1197);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2675);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2676);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5037);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tisco.
pub(crate) fn n1308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1198);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2677);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2678);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4151);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2679);
			match s {
				[] => true,
				['l', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2680);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4153);
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

/// Words ending -visco.
pub(crate) fn n1309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1199);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2681);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 4154);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -osco.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1200);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2682);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4155);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4157);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['c', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['a', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2684);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2685);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2686);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1201);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2687);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4158);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4159);
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
				['p', s @ ..] => {
					super::push(out, 2688);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4160);
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
				['x', s @ ..] => {
					super::push(out, 2689);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -usco.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1202);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2690);
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
		['f', s @ ..] => {
			super::push(out, 1203);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2691);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 4162);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1204);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2693);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2694);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2695);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -umido.
pub(crate) fn n1401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1256);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 2725);
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

/// Words ending -pido.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['a', 'l', 'i', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', 'i', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rido.
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1257);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 1258);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1259);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 2726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1260);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sido.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1261);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1262);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2728);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2729);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4184);
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
		['d', s @ ..] => {
			super::push(out, 1263);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1264);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1265);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1266);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2733);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2734);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1267);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2735);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tido.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vido.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1269);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2736);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1270);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2737);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2738);
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

/// Words ending -hendo.
pub(crate) fn n1424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1276);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2747);
			match s {
				[] => true,
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pendo.
pub(crate) fn n1426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1278);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2750);
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2751);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4186);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4187);
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
		['i', s @ ..] => {
			super::push(out, 2752);
			match s {
				[] => true,
				['c', 'c', 'o', 'l', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4188);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5041);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2753);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4190);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5042);
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
		['n', 'i', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2754);
			match s {
				[] => true,
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rendo.
pub(crate) fn n1427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1279);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 2755);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4191);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5043);
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
		['p', s @ ..] => {
			super::push(out, 2756);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tendo.
pub(crate) fn n1428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1280);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2757);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2758);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2759);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4192);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4194);
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
		['n', s @ ..] => {
			super::push(out, 2760);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2761);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4195);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
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
				['e', s @ ..] => {
					super::push(out, 4196);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 5045);
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
				['o', s @ ..] => {
					super::push(out, 4197);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2762);
			match s {
				[] => true,
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2763);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 2764);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vendo.
pub(crate) fn n1429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1281);
	match s {
		[] => true,
		['e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -indo.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1282);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1283);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2765);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4198);
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

/// Words ending -ondo.
pub(crate) fn n382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2766);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2767);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4199);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -undo.
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1286);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1287);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4200);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1288);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2769);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2770);
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1289);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2771);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4203);
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
		['f', s @ ..] => {
			super::push(out, 1290);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2772);
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2773);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4204);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 4205);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4206);
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
				['f', s @ ..] => {
					super::push(out, 2774);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2775);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2776);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5046);
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
								['t', s @ ..] => {
									super::push(out, 5047);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5389);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1291);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1292);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2777);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 4207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1293);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2779);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2780);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2781);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4208);
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
				['n', s @ ..] => {
					super::push(out, 2782);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2783);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2784);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2785);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 1294);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2786);
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

/// Words ending -dluceo.
pub(crate) fn n3427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2832);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4233);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5054);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lluceo.
pub(crate) fn n3429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2834);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4234);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4235);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4236);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5056);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5057);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word inluceo.
pub(crate) fn n3430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2835);
	s.is_empty()
}

/// The word superluceo.
pub(crate) fn n3431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2836);
	s.is_empty()
}

/// Words ending -sluceo.
pub(crate) fn n3432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2837);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 4237);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word muceo.
pub(crate) fn n1514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1346);
	s.is_empty()
}

/// Words ending -ndeo.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2859);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2860);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1358);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2861);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2862);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4259);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4260);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5066);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5393);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 5067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4261);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5068);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5069);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5394);
											match s {
												[] => true,
												['r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4262);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2863);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4263);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5071);
									s.is_empty()
								}
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
			super::push(out, 1359);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2864);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4264);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2865);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4265);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5073);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5074);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5395);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5396);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5471);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5075);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5397);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2866);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4267);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4268);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5076);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5398);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 5077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4269);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdeo.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 414);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1361);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2869);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4271);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4272);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5078);
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
						['d', s @ ..] => {
							super::push(out, 4273);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4274);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5080);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5399);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 5081);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5400);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5082);
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
						['m', s @ ..] => {
							super::push(out, 4275);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5083);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5401);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2870);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udeo.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2871);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2872);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', s @ ..] => {
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
				['l', s @ ..] => {
					super::push(out, 2873);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1364);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1365);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2874);
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

/// The word superfulgeo.
pub(crate) fn n8196(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -soleo.
pub(crate) fn n1565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1397);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2927);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word voleo.
pub(crate) fn n3528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2928);
	s.is_empty()
}

/// Words ending -pleo.
pub(crate) fn n445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 434);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1399);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1400);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1401);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 1402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orreo.
pub(crate) fn n1617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1447);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2976);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4329);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4330);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5103);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4331);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4332);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 4333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2977);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4334);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4335);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4336);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5105);
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
				['x', s @ ..] => {
					super::push(out, 4337);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -raetorqueo.
pub(crate) fn n8197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5472);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word circumtorqueo.
pub(crate) fn n8198(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word pertorqueo.
pub(crate) fn n8199(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word attorqueo.
pub(crate) fn n8200(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word conferveo.
pub(crate) fn n8201(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -inigo.
pub(crate) fn n1716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1525);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rigo.
pub(crate) fn n525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 512);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1526);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1527);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3032);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4359);
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
				['n', s @ ..] => {
					super::push(out, 3033);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4360);
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
		['b', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1528);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1529);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1530);
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1531);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1532);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3037);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5126);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5410);
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
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3038);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1533);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3039);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sigo.
pub(crate) fn n526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 513);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1534);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3040);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tigo.
pub(crate) fn n527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 514);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1535);
			match s {
				[] => true,
				['m', 'm', s @ ..] => {
					super::push(out, 3041);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4363);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1536);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1537);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3042);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vigo.
pub(crate) fn n528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 515);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1538);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4365);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xigo.
pub(crate) fn n529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 516);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1539);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word confingo.
pub(crate) fn n3700(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word perfingo.
pub(crate) fn n3701(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -lingo.
pub(crate) fn n1743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1550);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3054);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4374);
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
		['e', s @ ..] => {
			super::push(out, 3055);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'o', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mingo.
pub(crate) fn n1744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1551);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3056);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4375);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ningo.
pub(crate) fn n1745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1552);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3057);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pingo.
pub(crate) fn n1746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1553);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3058);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3059);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4376);
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
		['m', s @ ..] => {
			super::push(out, 3060);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4377);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3061);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3062);
			match s {
				[] => true,
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 3063);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4378);
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

/// Words ending -ringo.
pub(crate) fn n1747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1554);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 3064);
			match s {
				[] => true,
				['c', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4379);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4380);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5132);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5411);
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
				['n', s @ ..] => {
					super::push(out, 4381);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3065);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4382);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tingo.
pub(crate) fn n1748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1555);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3066);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3067);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4383);
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
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3068);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4384);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4385);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5133);
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
		['p', 'o', '\u{1}', s @ ..] => {
			super::push(out, 3069);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3070);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4386);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3071);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4387);
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

/// Words ending -ongo.
pub(crate) fn n533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 520);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1556);
			match s {
				[] => true,
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ungo.
pub(crate) fn n534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 521);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1557);
			match s {
				[] => true,
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1558);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3072);
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
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1559);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3073);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -befacio.
pub(crate) fn n5839(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cefacio.
pub(crate) fn n5840(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'l', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'd', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -defacio.
pub(crate) fn n5841(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gefacio.
pub(crate) fn n5842(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'r', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'p', 'x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -alefacio.
pub(crate) fn n5843(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mefacio.
pub(crate) fn n5844(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'r', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nefacio.
pub(crate) fn n5845(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pefacio.
pub(crate) fn n5846(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'o', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 't', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -refacio.
pub(crate) fn n5847(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'a', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'o', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'u', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'd', 'b', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uefacio.
pub(crate) fn n5848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4401);
	match s {
		[] => true,
		['c', 'a', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['g', 'n', 'a', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['q', 'i', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5413);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5474);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5486);
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
				['s', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vefacio.
pub(crate) fn n5849(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ifacio.
pub(crate) fn n3785(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'l', 'o', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'c', 'u', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['v', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lfacio.
pub(crate) fn n3786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3095);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4402);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5414);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5415);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5475);
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
						['x', s @ ..] => {
							super::push(out, 5416);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5476);
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
		['o', s @ ..] => {
			super::push(out, 4403);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word arfacio.
pub(crate) fn n3787(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -isfacio.
pub(crate) fn n3788(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4404);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iacio.
pub(crate) fn n1794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1583);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3096);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4405);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3097);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4406);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lacio.
pub(crate) fn n1795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1584);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3098);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3099);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -macio.
pub(crate) fn n1796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1585);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3100);
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

/// Words ending -racio.
pub(crate) fn n1797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1586);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sacio.
pub(crate) fn n1798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1587);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ccio.
pub(crate) fn n560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 546);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1588);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1589);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3103);
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

/// Words ending -ecio.
pub(crate) fn n561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 547);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 1590);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -icio.
pub(crate) fn n562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 548);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1591);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1592);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3104);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1593);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1594);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3105);
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
		['e', s @ ..] => {
			super::push(out, 1595);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1596);
			match s {
				[] => true,
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3106);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4407);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3107);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4408);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1597);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3108);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1598);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3109);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3111);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4411);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4412);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4413);
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
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1599);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3114);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4415);
							match s {
								[] => true,
								['r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1600);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1601);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3115);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1602);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'm', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 1603);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3118);
					match s {
						[] => true,
						['t', s @ ..] => super::t3::n5895(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n3831(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t3::n1815(s, out),
		_ => false,
	}
}

/// Words ending -lcio.
pub(crate) fn n563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 549);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1605);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4418);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1606);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3121);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ncio.
pub(crate) fn n564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 550);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1607);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3123);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1608);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1609);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1610);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ocio.
pub(crate) fn n565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 551);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1611);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1612);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3129);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 4422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1613);
			match s {
				[] => true,
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rcio.
pub(crate) fn n566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 552);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1615);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -scio.
pub(crate) fn n567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 553);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1616);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1617);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1618);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3131);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3132);
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
		['n', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ucio.
pub(crate) fn n568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 554);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1620);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3134);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4424);
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
				['s', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1622);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3136);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xcio.
pub(crate) fn n569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 555);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1623);
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

/// The word instabilio.
pub(crate) fn n8209(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word constabilio.
pub(crate) fn n8210(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -llio.
pub(crate) fn n592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 577);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1670);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3192);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1671);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5154);
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
			super::push(out, 1672);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3194);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5155);
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
		_ => false,
	}
}

/// Words ending -olio.
pub(crate) fn n593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 578);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1673);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1674);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3195);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1675);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3196);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3197);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4468);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3198);
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plio.
pub(crate) fn n594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 579);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1676);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulio.
pub(crate) fn n595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 580);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 1677);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3200);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -supervenio.
pub(crate) fn n8211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5477);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ignio.
pub(crate) fn n1903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1687);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inio.
pub(crate) fn n602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 587);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1688);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1689);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3219);
					s.is_empty()
				}
				['l', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3220);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1690);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mnio.
pub(crate) fn n603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 588);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1691);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1692);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 3222);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nnio.
pub(crate) fn n604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 589);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1693);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4487);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3224);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4489);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4490);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1694);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3225);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4491);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5163);
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
				['r', s @ ..] => {
					super::push(out, 3226);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4492);
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4493);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4494);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5164);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1695);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -onio.
pub(crate) fn n605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 590);
	match s {
		[] => true,
		['x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1696);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnio.
pub(crate) fn n606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 591);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1697);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -unio.
pub(crate) fn n607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1698);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1699);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1700);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3231);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1701);
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

/// Words ending -onsipio.
pub(crate) fn n6069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4499);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mpio.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 597);
	match s {
		[] => true,
		['i', '\u{1}', s @ ..] => {
			super::push(out, 1713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -opio.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 598);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 1714);
			s.is_empty()
		}
		['r', 'p', 'p', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1715);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1716);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3241);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ppio.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 599);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1717);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3242);
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

/// Words ending -rpio.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 600);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1718);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 3243);
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

/// Words ending -spio.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 601);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1719);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3244);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -upio.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 602);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1720);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3245);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3246);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3247);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xpio.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rrio.
pub(crate) fn n626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1741);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4511);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4512);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5168);
									s.is_empty()
								}
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
			super::push(out, 1742);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1743);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3271);
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
		['u', s @ ..] => {
			super::push(out, 1744);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3272);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4513);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3273);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3274);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4514);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3275);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4515);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -etrio.
pub(crate) fn n1963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1745);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 3276);
			match s {
				[] => true,
				['m', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntrio.
pub(crate) fn n1964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1746);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3277);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utrio.
pub(crate) fn n1965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1747);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 3278);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4516);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4518);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5170);
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

/// Words ending -urio.
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 613);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1748);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3279);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1749);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3281);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3282);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4520);
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
		['f', s @ ..] => {
			super::push(out, 1750);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1751);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3284);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1752);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1753);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3286);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4521);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4522);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5171);
									s.is_empty()
								}
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
			super::push(out, 1754);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3288);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							super::push(out, 4523);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'e', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3289);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4524);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 5173);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 5174);
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
				['i', s @ ..] => {
					super::push(out, 3290);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4525);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3291);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3292);
					match s {
						[] => true,
						['i', 'r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4527);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4528);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3293);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4529);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5176);
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
		['x', s @ ..] => {
			super::push(out, 1755);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3294);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -abrenuntio.
pub(crate) fn n8212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5478);
	s.is_empty()
}

/// Words ending -adripartio.
pub(crate) fn n8213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5479);
	match s {
		[] => true,
		['u', 'q', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word impertio.
pub(crate) fn n6149(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word inpertio.
pub(crate) fn n6150(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word dispertio.
pub(crate) fn n6151(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -tertio.
pub(crate) fn n4102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3312);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4533);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stio.
pub(crate) fn n640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1777);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3313);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4534);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4535);
							match s {
								[] => true,
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5180);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4536);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5181);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5182);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4537);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5183);
									match s {
										[] => true,
										['p', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1778);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3315);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ttio.
pub(crate) fn n641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1779);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3316);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1780);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3317);
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
		['u', s @ ..] => {
			super::push(out, 1781);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5184);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utio.
pub(crate) fn n642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1782);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3320);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1783);
			match s {
				[] => true,
				['c', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', 'e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1784);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3321);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4539);
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
		['g', s @ ..] => {
			super::push(out, 1785);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3322);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4540);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1786);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3323);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4541);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4542);
							s.is_empty()
						}
						['g', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1787);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3324);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word praeservio.
pub(crate) fn n8214(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -epello.
pub(crate) fn n4170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3361);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4573);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4574);
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

/// Words ending -mpello.
pub(crate) fn n4171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3362);
	match s {
		[] => true,
		['i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -npello.
pub(crate) fn n4172(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ppello.
pub(crate) fn n4174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3363);
	s.is_empty()
}

/// Words ending -rpello.
pub(crate) fn n4175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3364);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 5196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5197);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -spello.
pub(crate) fn n4176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3365);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word expello.
pub(crate) fn n4177(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -sello.
pub(crate) fn n2054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1830);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3366);
			match s {
				[] => true,
				['e', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tello.
pub(crate) fn n2055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1831);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3367);
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

/// The word duello.
pub(crate) fn n2056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1832);
	s.is_empty()
}

/// Words ending -vello.
pub(crate) fn n2057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1833);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3368);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3369);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3370);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4575);
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
		['i', s @ ..] => {
			super::push(out, 3371);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4576);
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
		['n', s @ ..] => {
			super::push(out, 3372);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4577);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3373);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4578);
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

/// Words ending -illo.
pub(crate) fn n681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	match s {
		[] => true,
		['b', 'i', 'r', 'c', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1834);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3374);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4579);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 5198);
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
				['c', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'u', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'u', 'm', 'r', 'u', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ollo.
pub(crate) fn n682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 660);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1836);
			match s {
				[] => true,
				['c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3375);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3377);
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ullo.
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 661);
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3378);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'a', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word intervolo.
pub(crate) fn n8215(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word instimulo.
pub(crate) fn n8216(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -apulo.
pub(crate) fn n2110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1869);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3399);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 3400);
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

/// Words ending -opulo.
pub(crate) fn n2111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1870);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3401);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3402);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4595);
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4596);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -upulo.
pub(crate) fn n2112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1871);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3403);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sulo.
pub(crate) fn n704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 682);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1872);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3404);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tulo.
pub(crate) fn n705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 683);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1873);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1874);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1875);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3405);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3406);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5210);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 5211);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5429);
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
								['x', s @ ..] => {
									super::push(out, 5212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3407);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mlino.
pub(crate) fn n2210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1936);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 3455);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4617);
					match s {
						[] => true,
						['r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nlino.
pub(crate) fn n2211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1937);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rlino.
pub(crate) fn n2212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1938);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3457);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4620);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mino.
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 739);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 'c', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1939);
			match s {
				[] => true,
				['n', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 't', s @ ..] => {
			match s {
				[] => true,
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['l', 'l', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4621);
							s.is_empty()
						}
						['r', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pino.
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 740);
	match s {
		[] => true,
		['r', 'a', 'c', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sino.
pub(crate) fn n772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 742);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1940);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1941);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3459);
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

/// Words ending -tino.
pub(crate) fn n773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 743);
	match s {
		[] => true,
		['a', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1942);
			match s {
				[] => true,
				['i', 'r', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1943);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3460);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1944);
			match s {
				[] => true,
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1945);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uino.
pub(crate) fn n774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 744);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1946);
			match s {
				[] => true,
				['n', 'a', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vino.
pub(crate) fn n775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 745);
	match s {
		[] => true,
		['i', 'd', 'd', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tono.
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 756);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1971);
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1972);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1973);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3476);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1974);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3478);
					s.is_empty()
				}
				['x', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1975);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3480);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4629);
							match s {
								[] => true,
								['r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1976);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1977);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['r', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3483);
					s.is_empty()
				}
				['x', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3484);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gero.
pub(crate) fn n831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 793);
	match s {
		[] => true,
		['b', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2039);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3540);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3541);
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
		['i', s @ ..] => {
			super::push(out, 2040);
			match s {
				[] => true,
				['b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3543);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4661);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4662);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 5223);
									s.is_empty()
								}
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
			super::push(out, 2041);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2042);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2043);
			match s {
				[] => true,
				['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tolero.
pub(crate) fn n4503(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4663);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4664);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5224);
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

/// Words ending -mero.
pub(crate) fn n834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 796);
	match s {
		[] => true,
		['o', 'l', 'g', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2046);
			match s {
				[] => true,
				['n', 'b', s @ ..] => {
					super::push(out, 3544);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nero.
pub(crate) fn n835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 797);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2047);
			match s {
				[] => true,
				['g', 'e', 'd', s @ ..] => {
					super::push(out, 3545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'o', 'v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2048);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pero.
pub(crate) fn n837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 798);
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', 't', 'b', s @ ..] => {
					super::push(out, 3546);
					s.is_empty()
				}
				['i', 'd', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2049);
			match s {
				[] => true,
				['r', 'p', 'e', s @ ..] => {
					super::push(out, 3547);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4666);
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
		['s', 'e', s @ ..] => {
			super::push(out, 2050);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3548);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sero.
pub(crate) fn n838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 799);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2052);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2053);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2054);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2055);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2056);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2057);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3553);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 4669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2058);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3554);
					match s {
						[] => true,
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2059);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3555);
					s.is_empty()
				}
				['i', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4670);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4671);
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
		['x', s @ ..] => {
			super::push(out, 2060);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3556);
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

/// Words ending -tero.
pub(crate) fn n839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 800);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2061);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2062);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3557);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5225);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 5226);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5430);
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
								['e', s @ ..] => {
									super::push(out, 5227);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5431);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2063);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2064);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2065);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3558);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2066);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2067);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2068);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3560);
					match s {
						[] => true,
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4673);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 4674);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3561);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4675);
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
		['o', s @ ..] => {
			super::push(out, 2069);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3562);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
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
				['a', s @ ..] => {
					super::push(out, 3563);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 2070);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3565);
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

/// Words ending -vero.
pub(crate) fn n840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 801);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2071);
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2072);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word exero.
pub(crate) fn n2395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2073);
	s.is_empty()
}

/// The word expectoro.
pub(crate) fn n8219(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ocurro.
pub(crate) fn n4583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3605);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 4691);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rcurro.
pub(crate) fn n4584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3606);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 5233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5234);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -scurro.
pub(crate) fn n4585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3607);
	match s {
		[] => true,
		['b', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xcurro.
pub(crate) fn n4586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3608);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5235);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5433);
							match s {
								[] => true,
								['p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -surro.
pub(crate) fn n2438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2102);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 3609);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olito.
pub(crate) fn n2577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2211);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3705);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 3706);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4738);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5250);
							match s {
								[] => true,
								['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4739);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4740);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5251);
							match s {
								[] => true,
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mito.
pub(crate) fn n945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 893);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2212);
			match s {
				[] => true,
				['l', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4741);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4742);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2213);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3707);
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
		['r', 'o', 'd', s @ ..] => {
			match s {
				[] => true,
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -anito.
pub(crate) fn n2582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2214);
	match s {
		[] => true,
		['v', '\u{1}', s @ ..] => {
			super::push(out, 3708);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inito.
pub(crate) fn n2584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2215);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 3709);
			match s {
				[] => true,
				['o', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word sonito.
pub(crate) fn n2585(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -pito.
pub(crate) fn n947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 895);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2216);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4743);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2217);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2218);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3710);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3711);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4745);
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
		['s', s @ ..] => {
			super::push(out, 2219);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4746);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4747);
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

/// Words ending -rito.
pub(crate) fn n948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 896);
	match s {
		[] => true,
		['g', 'i', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'q', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2220);
			match s {
				[] => true,
				['i', 'u', s @ ..] => {
					super::push(out, 3712);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 4748);
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

/// Words ending -sito.
pub(crate) fn n949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 897);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2221);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2222);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'f', 'e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2223);
			match s {
				[] => true,
				['u', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4749);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2224);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3715);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4751);
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
				['e', s @ ..] => {
					super::push(out, 3716);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4752);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5254);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3717);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4754);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5255);
									match s {
										[] => true,
										['l', 'i', 'c', 'i', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4755);
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
				['u', s @ ..] => {
					super::push(out, 3718);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tito.
pub(crate) fn n950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 898);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2226);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3720);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3721);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4757);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5256);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 5436);
											match s {
												[] => true,
												['a', 'm', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4758);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3722);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'c', 'c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'r', 'c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uito.
pub(crate) fn n951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 899);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 2227);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 2228);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4759);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4760);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5258);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5259);
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
						['d', s @ ..] => {
							super::push(out, 4761);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5260);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4762);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4763);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4764);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 5437);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 5438);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 5480);
													match s {
														[] => true,
														['i', '\u{1}', s @ ..] => super::t3::n8249(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2229);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2230);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3724);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4766);
							match s {
								[] => true,
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vito.
pub(crate) fn n952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 900);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 's', 'b', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word subausculto.
pub(crate) fn n8223(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -exsisto.
pub(crate) fn n6663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4793);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5268);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5269);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5440);
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

/// Words ending -ntisto.
pub(crate) fn n4830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3770);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 4794);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xisto.
pub(crate) fn n2663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2272);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			super::push(out, 3771);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -msto.
pub(crate) fn n976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 922);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 2273);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3772);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nsto.
pub(crate) fn n977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 923);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2274);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2275);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3773);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -osto.
pub(crate) fn n978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 924);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 2276);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3774);
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
		['t', '\u{1}', s @ ..] => {
			super::push(out, 2277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rsto.
pub(crate) fn n979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 925);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2278);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3775);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4795);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4796);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3776);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4797);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -usto.
pub(crate) fn n980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 926);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 2279);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'c', 'e', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xsto.
pub(crate) fn n981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 927);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2280);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -supermitto.
pub(crate) fn n8225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5481);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word reminiscor.
pub(crate) fn n8251(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -riscor.
pub(crate) fn n4987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3867);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 4854);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5300);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tiscor.
pub(crate) fn n4988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3868);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4855);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5301);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -viscor.
pub(crate) fn n4989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3869);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4856);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5302);
					match s {
						[] => true,
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word praetergredior.
pub(crate) fn n8229(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word intergredior.
pub(crate) fn n8230(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -lior.
pub(crate) fn n1109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1046);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2432);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2433);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3891);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 3892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2434);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3893);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4866);
							match s {
								[] => true,
								['r', 'u', 'a', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rior.
pub(crate) fn n1111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1048);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2436);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3895);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3896);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4868);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5308);
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
						['x', s @ ..] => {
							super::push(out, 4869);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3897);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3898);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3899);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3900);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4871);
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
				['h', s @ ..] => {
					super::push(out, 3901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3902);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3903);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4874);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4875);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5310);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4876);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3904);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4877);
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
				['p', s @ ..] => {
					super::push(out, 3905);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 3906);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4879);
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
		['u', s @ ..] => {
			super::push(out, 2438);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3907);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3908);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tior.
pub(crate) fn n1112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1049);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2439);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4882);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4883);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4884);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4885);
							match s {
								[] => true,
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2440);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3911);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4886);
							s.is_empty()
						}
						['p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4887);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4888);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5311);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2441);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2442);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3913);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4889);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4891);
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5314);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2443);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3914);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3915);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3916);
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
		['r', s @ ..] => {
			super::push(out, 2444);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3917);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 5315);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5447);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5316);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 5448);
											s.is_empty()
										}
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
					super::push(out, 3918);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4892);
							match s {
								[] => true,
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word excarnifico.
pub(crate) fn n8233(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ndesco.
pub(crate) fn n3084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2601);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4032);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4951);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4952);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4033);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4953);
					match s {
						[] => true,
						['p', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5451);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 5452);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5483);
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
				['r', s @ ..] => {
					super::push(out, 4954);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 5335);
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
		['o', s @ ..] => {
			super::push(out, 4034);
			match s {
				[] => true,
				['r', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdesco.
pub(crate) fn n3085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2602);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4035);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4955);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4956);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5336);
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
				['x', s @ ..] => {
					super::push(out, 4957);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4036);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4037);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -udesco.
pub(crate) fn n3086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2603);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 4038);
			match s {
				[] => true,
				['c', 'e', s @ ..] => {
					super::push(out, 4959);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5338);
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

/// Words ending -gesco.
pub(crate) fn n1288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1178);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2605);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4039);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4960);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4961);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5340);
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
						['f', s @ ..] => {
							super::push(out, 4962);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4040);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4963);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5342);
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
		['l', s @ ..] => {
			super::push(out, 2606);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4041);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4042);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2607);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2608);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4043);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iesco.
pub(crate) fn n1289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1179);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 2609);
			match s {
				[] => true,
				['q', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4964);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4965);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5343);
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
						['n', s @ ..] => {
							super::push(out, 4966);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4967);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5344);
									match s {
										[] => true,
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lesco.
pub(crate) fn n1290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1180);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2610);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4044);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4045);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4968);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4969);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5345);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5346);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5453);
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
				['o', s @ ..] => {
					super::push(out, 4046);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4971);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5347);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5348);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5454);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4972);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5349);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5350);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5455);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2611);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4047);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4973);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4048);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							super::push(out, 4974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2612);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4049);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5352);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5353);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'x', s @ ..] => {
							super::push(out, 4975);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4050);
					match s {
						[] => true,
						['v', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4051);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4052);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4978);
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
		['o', s @ ..] => {
			super::push(out, 2613);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4053);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4054);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4055);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4980);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4981);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4056);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4982);
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
				['s', s @ ..] => {
					super::push(out, 4057);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4983);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4984);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 4058);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mesco.
pub(crate) fn n1291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1181);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2615);
			match s {
				[] => true,
				['g', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 4059);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4060);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5356);
									s.is_empty()
								}
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
			super::push(out, 2616);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 4061);
					match s {
						[] => true,
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2617);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4062);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4063);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4064);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4065);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nesco.
pub(crate) fn n1292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1182);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2618);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4066);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4068);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 4069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2619);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4988);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4989);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5357);
									s.is_empty()
								}
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
			super::push(out, 2620);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2621);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2622);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 4072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2623);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4073);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pesco.
pub(crate) fn n1293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1183);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2624);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4990);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5358);
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
		['m', s @ ..] => {
			super::push(out, 2625);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4074);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4991);
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
		['n', s @ ..] => {
			super::push(out, 2626);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4075);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 4992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2627);
			match s {
				[] => true,
				['o', 't', 'b', s @ ..] => {
					super::push(out, 4076);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4993);
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
		['s', s @ ..] => {
			super::push(out, 2628);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4077);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 4994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2629);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 4078);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -resco.
pub(crate) fn n1294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1184);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2630);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4079);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4080);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4995);
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
				['l', s @ ..] => {
					super::push(out, 4081);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4996);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4997);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 5359);
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
				['m', s @ ..] => {
					super::push(out, 4082);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4083);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4084);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4085);
					match s {
						[] => true,
						['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 4086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2631);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4998);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4999);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5360);
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
			super::push(out, 2632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4087);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4088);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4089);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4090);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4091);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4092);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5001);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5361);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5456);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5002);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5362);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4093);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 4094);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2633);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4095);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5363);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5457);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5364);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5365);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 5458);
											s.is_empty()
										}
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
					super::push(out, 4096);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4097);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2634);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4098);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2635);
			match s {
				[] => true,
				['v', 'e', s @ ..] => {
					super::push(out, 4100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2636);
			match s {
				[] => true,
				['l', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5005);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5366);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5006);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2637);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4101);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5368);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 5459);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5369);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5460);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5370);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5461);
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
								['r', s @ ..] => {
									super::push(out, 5371);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 5372);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t3::n3121(s, out),
		['u', s @ ..] => super::t3::n3122(s, out),
		_ => false,
	}
}

/// Words ending -sesco.
pub(crate) fn n1295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1185);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 2640);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tesco.
pub(crate) fn n1296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1186);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2641);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4107);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5009);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5010);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5373);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2642);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4109);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4110);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5011);
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
				['o', s @ ..] => {
					super::push(out, 4111);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2643);
			match s {
				[] => true,
				['b', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2644);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4113);
					match s {
						[] => true,
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4114);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							super::push(out, 5012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2645);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4115);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2646);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4116);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5013);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5014);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5374);
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
						['r', s @ ..] => {
							super::push(out, 5015);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2647);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4117);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4118);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4119);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4120);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5016);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 5017);
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

/// Words ending -uesco.
pub(crate) fn n1297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1187);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 2648);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4121);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5018);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5462);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 5463);
											match s {
												[] => true,
												['o', '\u{1}', s @ ..] => {
													super::push(out, 5484);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 5464);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5019);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5375);
									s.is_empty()
								}
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
			super::push(out, 2649);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 2650);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4123);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5376);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 5377);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5465);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2651);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4124);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5021);
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
				['n', s @ ..] => {
					super::push(out, 4125);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

