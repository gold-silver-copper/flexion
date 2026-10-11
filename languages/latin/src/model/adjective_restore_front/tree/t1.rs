use flexion::Path;

/// Words ending -sa.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1844);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1845);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7931);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1846);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4774);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7933);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4775);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7935);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7936);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', 'e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10190);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10191);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 't', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10192);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10193);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 357);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1849);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4777);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1850);
					match s {
						[] => true,
						['a', 'r', 'i', 'c', s @ ..] => {
							super::push(out, 4779);
							match s {
								[] => true,
								['i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7937);
									s.is_empty()
								}
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
			super::push(out, 358);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1851);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1852);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4780);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4781);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7938);
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
								['g', s @ ..] => {
									super::push(out, 7939);
									s.is_empty()
								}
								['i', 'o', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7940);
									s.is_empty()
								}
								['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7941);
									match s {
										[] => true,
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1853);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4782);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1854);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1855);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4784);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10194);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10195);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 360);
			match s {
				[] => true,
				['a', s @ ..] => super::t3::n1887(s, out),
				['e', s @ ..] => super::t3::n1888(s, out),
				['i', s @ ..] => super::t3::n1889(s, out),
				['o', s @ ..] => super::t3::n1890(s, out),
				['u', s @ ..] => super::t3::n1891(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t3::n362(s, out),
		['y', s @ ..] => super::t3::n363(s, out),
		_ => false,
	}
}

/// Words ending -ta.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 363);
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
		['h', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1866);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1867);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4806);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 365);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 366);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 367);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1869);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4808);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1871);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4810);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4811);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7953);
									s.is_empty()
								}
								['h', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10198);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10199);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 7954);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4812);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4813);
							s.is_empty()
						}
						['v', 'i', 'r', 'i', 'd', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4814);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 4815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1872);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1873);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1875);
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 4816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1876);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							super::push(out, 4817);
							match s {
								[] => true,
								['u', 'r', s @ ..] => {
									super::push(out, 7955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ua.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1877);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4818);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7956);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4819);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 7958);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7959);
									match s {
										[] => true,
										['n', 'a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t3::n13912(s, out),
														['s', s @ ..] => super::t3::n13913(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 11404);
													s.is_empty()
												}
												['t', 'u', s @ ..] => {
													match s {
														[] => true,
														['r', s @ ..] => super::t3::n13914(s, out),
														['s', s @ ..] => super::t3::n13915(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 10200);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 7960);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4820);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', s @ ..] => {
											super::push(out, 10201);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11405);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11406);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 10202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1878);
					match s {
						[] => true,
						['p', 'a', 'b', s @ ..] => {
							super::push(out, 4822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 371);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1879);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4823);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 4824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1880);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1881);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4825);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4826);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7961);
									match s {
										[] => true,
										['r', 'i', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 11407);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 11408);
													s.is_empty()
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
										['r', s @ ..] => {
											super::push(out, 10203);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10204);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 372);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1882);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4827);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1884);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4829);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4830);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 7962);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10205);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10206);
											s.is_empty()
										}
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
					super::push(out, 1885);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4831);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4832);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4833);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 7963);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 7964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4834);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t3::n1918(s, out),
				['o', s @ ..] => super::t3::n1919(s, out),
				['u', s @ ..] => super::t3::n1920(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t3::n376(s, out),
		['t', s @ ..] => {
			super::push(out, 376);
			s.is_empty()
		}
		['x', s @ ..] => super::t3::n378(s, out),
		_ => false,
	}
}

/// Words ending -va.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 378);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1892);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1893);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1895);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1896);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1897);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1898);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1900);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7970);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1901);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7972);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7973);
									s.is_empty()
								}
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
				['l', s @ ..] => {
					super::push(out, 1902);
					s.is_empty()
				}
				['n', 'c', 'u', 'l', 'a', 'r', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1903);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xa.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1905);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -za.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['g', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 1906);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dnasitpab.
pub(crate) fn n13098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11410);
	s.is_empty()
}

/// Words ending -snasitpab.
pub(crate) fn n13099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11411);
	s.is_empty()
}

/// Words ending -uab.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['b', 'a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4859);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 4860);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vab.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	s.is_empty()
}

/// Words ending -eb.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 397);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1923);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 398);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1925);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4861);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7985);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7986);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10212);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 10213);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['l', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 11412);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 11413);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11414);
													s.is_empty()
												}
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
							super::push(out, 4862);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7987);
									match s {
										[] => true,
										['o', 's', 'i', s @ ..] => {
											super::push(out, 10214);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', 'u', 's', '\u{1}', s @ ..] => super::t3::n13918(s, out),
														['s', '\u{1}', s @ ..] => super::t3::n13919(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													match s {
														[] => true,
														['r', 'u', 's', '\u{1}', s @ ..] => super::t3::n13920(s, out),
														['s', '\u{1}', s @ ..] => super::t3::n13921(s, out),
														_ => false,
													}
												}
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
							super::push(out, 4863);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7988);
									match s {
										[] => true,
										['n', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10215);
											s.is_empty()
										}
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
					super::push(out, 1926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1927);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4864);
							match s {
								[] => true,
								['i', 'c', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', 'd', s @ ..] => {
											super::push(out, 10216);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 10217);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4865);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4866);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1928);
					match s {
						[] => true,
						['a', 'l', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1929);
					match s {
						[] => true,
						['f', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4869);
							s.is_empty()
						}
						['g', 'n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7989);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7990);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'o', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1930);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1931);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 7991);
									s.is_empty()
								}
								['i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7992);
									s.is_empty()
								}
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
			super::push(out, 402);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 403);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1933);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1934);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ib.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 404);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['o', 'l', 'o', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 't', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['u', 'a', s @ ..] => {
					super::push(out, 1935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1936);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7993);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 7994);
									s.is_empty()
								}
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
			super::push(out, 408);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1937);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1939);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1940);
					match s {
						[] => true,
						['a', 'r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4871);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4872);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4873);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4874);
							match s {
								[] => true,
								['e', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4875);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 4876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1941);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1942);
					match s {
						[] => true,
						['s', 't', 'r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1943);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1944);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4878);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4879);
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
		['n', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1946);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1947);
					match s {
						[] => true,
						['t', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1948);
					match s {
						[] => true,
						['n', 'c', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 413);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['p', 'i', s @ ..] => {
					super::push(out, 1949);
					match s {
						[] => true,
						['n', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 415);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lb.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1950);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1951);
					match s {
						[] => true,
						['p', 'h', 'e', 'm', 'a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7996);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7997);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 7998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1952);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1953);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ob.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1954);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4883);
							s.is_empty()
						}
						['o', 's', 'l', 'o', 'v', 'e', 'n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1957);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1958);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['b', 'y', s @ ..] => {
					super::push(out, 1960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['e', 'a', 'l', s @ ..] => {
					super::push(out, 1962);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7999);
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
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1963);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1964);
					match s {
						[] => true,
						['l', 'l', s @ ..] => {
							super::push(out, 4885);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rb.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4886);
							s.is_empty()
						}
						['h', 'i', 'a', s @ ..] => {
							super::push(out, 4887);
							match s {
								[] => true,
								['t', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1965);
					match s {
						[] => true,
						['i', 'l', 'i', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8002);
									s.is_empty()
								}
								['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 8003);
									s.is_empty()
								}
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
			super::push(out, 425);
			match s {
				[] => true,
				['v', 'i', 's', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['m', 'o', s @ ..] => {
					super::push(out, 1966);
					match s {
						[] => true,
						['s', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['k', 'l', 'y', 'n', 'i', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1968);
					s.is_empty()
				}
				['n', 'n', s @ ..] => {
					super::push(out, 1969);
					s.is_empty()
				}
				['s', 'q', 'u', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1970);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ub.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1971);
					match s {
						[] => true,
						['n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1972);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1973);
					s.is_empty()
				}
				['h', 'e', 'n', 'a', 'v', 'i', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1974);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1975);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4891);
							match s {
								[] => true,
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4892);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8005);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8006);
									s.is_empty()
								}
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
			super::push(out, 432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yb.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 433);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 434);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bacac.
pub(crate) fn n5158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4893);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10219);
					match s {
						[] => true,
						['e', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', 's', s @ ..] => {
					super::push(out, 10220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tacac.
pub(crate) fn n5160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4895);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 10221);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10222);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ocac.
pub(crate) fn n2020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1980);
	s.is_empty()
}

/// Words ending -ucac.
pub(crate) fn n2021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1981);
	s.is_empty()
}

/// Words ending -dac.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1982);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1983);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1984);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8007);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eac.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1985);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 4896);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10223);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10224);
									s.is_empty()
								}
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
			super::push(out, 1986);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4897);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8009);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1987);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iac.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	s.is_empty()
}

/// Words ending -lac.
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1988);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4898);
					match s {
						[] => true,
						['i', 't', 'o', 's', 'i', 's', s @ ..] => {
							super::push(out, 8011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4899);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10225);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10226);
									s.is_empty()
								}
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
			super::push(out, 1989);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4900);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8012);
							s.is_empty()
						}
						['r', 'i', s @ ..] => {
							super::push(out, 8013);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 10227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4901);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1990);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4903);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4904);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4905);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1991);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1992);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4907);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4908);
					match s {
						[] => true,
						['o', 'r', 'n', 'i', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10228);
									s.is_empty()
								}
								['c', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10229);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4909);
					match s {
						[] => true,
						['a', 't', 'u', 's', s @ ..] => {
							super::push(out, 8015);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1993);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1994);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1995);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1996);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mac.
pub(crate) fn n445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['e', 'n', s @ ..] => {
			super::push(out, 1997);
			match s {
				[] => true,
				['a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 4911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nac.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1998);
			match s {
				[] => true,
				['d', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4912);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4913);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4914);
					match s {
						[] => true,
						['i', 'u', s @ ..] => {
							super::push(out, 8016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1999);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2000);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 4916);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2001);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4918);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2002);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4920);
					match s {
						[] => true,
						['e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 8017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2003);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2004);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4921);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10232);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10233);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10234);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8018);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2005);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4923);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4924);
					s.is_empty()
				}
				['t', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4925);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pac.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 443);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2006);
			match s {
				[] => true,
				['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 4926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2007);
			match s {
				[] => true,
				['n', 's', s @ ..] => {
					super::push(out, 4927);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2008);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2009);
			match s {
				[] => true,
				['a', 'r', 'i', 'c', s @ ..] => {
					super::push(out, 4928);
					match s {
						[] => true,
						['i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2010);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4929);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8022);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4930);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8024);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 8025);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11995);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11996);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 11417);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 10235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rac.
pub(crate) fn n448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2011);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4932);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4933);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2012);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4935);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4937);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4939);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8026);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4940);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8028);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4941);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2015);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2016);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2017);
			match s {
				[] => true,
				['l', 'i', 'n', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8030);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2018);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2019);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2020);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4943);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4944);
					match s {
						[] => true,
						['l', 'a', 'g', 'i', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10236);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10237);
									s.is_empty()
								}
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
			super::push(out, 2021);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sac.
pub(crate) fn n449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 445);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2022);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4945);
					match s {
						[] => true,
						['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4946);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4947);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4948);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2023);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4950);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8033);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2025);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2026);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 4951);
					match s {
						[] => true,
						['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', 'e', s @ ..] => {
					super::push(out, 4952);
					match s {
						[] => true,
						['n', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4953);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 8037);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 8038);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11418);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11419);
											s.is_empty()
										}
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
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tac.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2029);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2030);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4955);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2031);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2032);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2033);
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					super::push(out, 4957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uac.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2034);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2035);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8039);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8040);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11420);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11421);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2036);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4958);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4959);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4960);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vac.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2037);
			match s {
				[] => true,
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 4961);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ec.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 449);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 450);
			match s {
				[] => true,
				['a', 'r', 'b', 'e', 'r', 'g', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2038);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2039);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 451);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2041);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4962);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 4963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2042);
					match s {
						[] => true,
						['b', s @ ..] => {
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
								['r', 'a', s @ ..] => {
									super::push(out, 8041);
									match s {
										[] => true,
										['t', 'i', s @ ..] => {
											super::push(out, 10238);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 11422);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11423);
													s.is_empty()
												}
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
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
								['i', 'o', 'r', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 452);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2043);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4964);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4965);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8042);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10239);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10240);
											s.is_empty()
										}
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
					super::push(out, 2044);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2045);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4966);
							match s {
								[] => true,
								['r', 'i', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 10241);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10242);
											s.is_empty()
										}
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
					super::push(out, 2046);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4967);
							match s {
								[] => true,
								['n', 'n', s @ ..] => {
									super::push(out, 8043);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4968);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4969);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4970);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 8045);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 8046);
									match s {
										[] => true,
										['l', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 11424);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 11425);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 11426);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'l', 'e', 'n', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4971);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t3::n458(s, out),
		['s', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['s', 'a', 't', s @ ..] => {
					super::push(out, 2055);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8052);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8053);
									s.is_empty()
								}
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
			super::push(out, 456);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2056);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 457);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hc.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2058);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 4985);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10245);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10246);
											s.is_empty()
										}
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
					super::push(out, 2059);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4986);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4987);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 460);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2060);
					s.is_empty()
				}
				['m', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['a', 'm', 's', 'u', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2062);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4989);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2063);
					s.is_empty()
				}
				['n', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2064);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2065);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4991);
							s.is_empty()
						}
						['u', 'r', 'g', 'i', 'u', s @ ..] => {
							super::push(out, 4992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['o', 'r', 'o', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['p', 't', 'e', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['l', 'e', 'd', s @ ..] => {
					super::push(out, 2068);
					match s {
						[] => true,
						['o', 'c', 'h', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'g', s @ ..] => {
					super::push(out, 2069);
					match s {
						[] => true,
						['r', 'a', 'p', 'h', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2070);
					match s {
						[] => true,
						['s', 't', 'i', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10247);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 11427);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11428);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 10248);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 8054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2071);
					match s {
						[] => true,
						['o', 'c', 'o', 'c', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2072);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', 't', 'h', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8055);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8056);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['a', 'r', 'p', s @ ..] => {
													match s {
														[] => true,
														['o', 's', '\u{1}', s @ ..] => super::t3::n13924(s, out),
														['u', 's', '\u{1}', s @ ..] => super::t3::n13925(s, out),
														_ => false,
													}
												}
												['h', s @ ..] => {
													super::push(out, 11429);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'o', 't', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10249);
											s.is_empty()
										}
										['p', 't', 'e', 'r', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10250);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', 'l', 'l', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8057);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ic.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4997);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4998);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 4999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2073);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2074);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2076);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2077);
					match s {
						[] => true,
						['i', 'n', s @ ..] => {
							super::push(out, 5000);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5002);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 8059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5003);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 10251);
											match s {
												[] => true,
												['a', 'v', 'i', 'g', 'a', s @ ..] => {
													match s {
														[] => true,
														['n', 'd', s @ ..] => super::t3::n13926(s, out),
														['t', 'u', 'r', s @ ..] => super::t3::n13927(s, out),
														_ => false,
													}
												}
												['e', s @ ..] => {
													super::push(out, 11430);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 10252);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 10253);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 11431);
													match s {
														[] => true,
														['l', 'l', 'a', 't', s @ ..] => super::t3::n13928(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 469);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2079);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5004);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'o', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2080);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lc.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['d', 'o', 's', 'p', 'o', 'r', 'i', 'o', 'i', 'd', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2081);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2082);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5007);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8060);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8061);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 10254);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2083);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2084);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5009);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8062);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5010);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8064);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5011);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 8066);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8067);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 10255);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 10256);
											s.is_empty()
										}
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
					super::push(out, 2085);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2086);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2087);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2088);
					match s {
						[] => true,
						['i', 'p', s @ ..] => {
							super::push(out, 5012);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 8068);
									s.is_empty()
								}
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
			super::push(out, 472);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2089);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2090);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2092);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2093);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8069);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2094);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2095);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 474);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2097);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2098);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nc.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	s.is_empty()
}

/// Words ending -oc.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2100);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5013);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8071);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2101);
					match s {
						[] => true,
						['u', 'n', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 8073);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2102);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2103);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5015);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5016);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2104);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 5018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2105);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2106);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2107);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2108);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2109);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8075);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', 'a', s @ ..] => {
			match s {
				[] => true,
				['l', 't', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2110);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['c', 'i', 'f', 'e', 'r', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2112);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5019);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 480);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2114);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2115);
					match s {
						[] => true,
						['t', 'a', 'b', s @ ..] => {
							super::push(out, 5021);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8077);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8078);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2116);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5022);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5023);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2117);
					match s {
						[] => true,
						['r', 'e', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 5025);
							s.is_empty()
						}
						['s', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 482);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2118);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5027);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5028);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5029);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10257);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10258);
											s.is_empty()
										}
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
					super::push(out, 2119);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5030);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t3::n2166(s, out),
				['o', s @ ..] => {
					super::push(out, 2121);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5037);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5038);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5039);
							match s {
								[] => true,
								['a', 'd', s @ ..] => {
									super::push(out, 8092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t3::n489(s, out),
		['n', s @ ..] => super::t3::n490(s, out),
		['o', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2148);
					match s {
						[] => true,
						['o', 's', 'i', 's', s @ ..] => {
							super::push(out, 5140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2149);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['u', s @ ..] => super::t3::n2197(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t3::n494(s, out),
		['s', s @ ..] => {
			super::push(out, 490);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2162);
					s.is_empty()
				}
				['i', s @ ..] => super::t3::n2210(s, out),
				['o', s @ ..] => super::t3::n2211(s, out),
				['t', s @ ..] => super::t3::n2212(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t3::n497(s, out),
		['x', s @ ..] => super::t3::n498(s, out),
		_ => false,
	}
}

/// Words ending -rc.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2167);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2168);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2169);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2170);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5149);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 495);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2172);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5151);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5152);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8297);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10409);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10410);
											s.is_empty()
										}
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
					super::push(out, 2173);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8298);
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
									super::push(out, 8299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2174);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2175);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8300);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8301);
									match s {
										[] => true,
										['o', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10411);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11551);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11552);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2176);
					match s {
						[] => true,
						['a', 't', 'i', s @ ..] => {
							super::push(out, 5153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2177);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2178);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5154);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 496);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2181);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2182);
					match s {
						[] => true,
						['i', 'n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8302);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8303);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 11553);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 11554);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2183);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2184);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5156);
							match s {
								[] => true,
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8304);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5157);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5158);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8305);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['c', 'i', 't', 'u', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2187);
					match s {
						[] => true,
						['i', 'a', 'b', s @ ..] => {
							super::push(out, 5161);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8307);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2188);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5162);
							match s {
								[] => true,
								['l', 'i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10412);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', s @ ..] => super::t3::n13302(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => super::t3::n9098(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t3::n5455(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t3::n2236(s, out),
				['r', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => super::t3::n2237(s, out),
				['s', s @ ..] => super::t3::n2238(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t3::n504(s, out),
		_ => false,
	}
}

/// Words ending -uc.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2192);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8310);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2193);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5164);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 502);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2195);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5166);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2197);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8312);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8313);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10413);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10414);
											s.is_empty()
										}
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
					super::push(out, 2198);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							super::push(out, 5168);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 10415);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10416);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 504);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2199);
					match s {
						[] => true,
						['t', 'a', 'b', s @ ..] => {
							super::push(out, 5169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2200);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8314);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8315);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11556);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11557);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['i', 't', s @ ..] => {
					super::push(out, 2202);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8316);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8317);
									s.is_empty()
								}
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
			super::push(out, 507);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2203);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5170);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5171);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5172);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8318);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8319);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10417);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10418);
											s.is_empty()
										}
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
					super::push(out, 2204);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2205);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2206);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2207);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 5173);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10419);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10420);
											s.is_empty()
										}
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
					super::push(out, 2208);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 5174);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10421);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10422);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 508);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2209);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['a', 'n', 'e', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yc.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['s', 'c', 'e', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 8320);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 5175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 511);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 512);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 513);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 514);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 515);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 516);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 518);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ed.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 530);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['i', 'l', 'i', 's', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2225);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5179);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5180);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5181);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'a', 'n', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2226);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2227);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5183);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5184);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5185);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8324);
									match s {
										[] => true,
										['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10425);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5186);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5187);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2228);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5189);
							match s {
								[] => true,
								['i', 'l', 'i', 's', s @ ..] => {
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
						['d', s @ ..] => {
							super::push(out, 5190);
							match s {
								[] => true,
								['e', 'n', 'd', s @ ..] => {
									super::push(out, 8325);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5191);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5192);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5193);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8327);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2229);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5195);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', s @ ..] => {
											super::push(out, 10426);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11558);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11559);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 10427);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'a', 'n', s @ ..] => {
									super::push(out, 8329);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10428);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10429);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5196);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10430);
											match s {
												[] => true,
												['n', 's', s @ ..] => {
													super::push(out, 11560);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 11561);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 10431);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 8330);
									match s {
										[] => true,
										['i', 'r', 'o', 's', 't', 'r', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 10432);
											s.is_empty()
										}
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
					super::push(out, 2230);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5197);
							s.is_empty()
						}
						['l', s @ ..] => super::t3::n5498(s, out),
						['q', s @ ..] => super::t3::n5499(s, out),
						['r', s @ ..] => super::t3::n5500(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n2282(s, out),
				['u', s @ ..] => super::t3::n2283(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t3::n538(s, out),
		['f', s @ ..] => super::t3::n539(s, out),
		['g', s @ ..] => super::t3::n540(s, out),
		['h', s @ ..] => super::t3::n541(s, out),
		['i', s @ ..] => super::t3::n542(s, out),
		['l', s @ ..] => super::t3::n543(s, out),
		['m', s @ ..] => super::t3::n544(s, out),
		['n', s @ ..] => super::t3::n545(s, out),
		['o', s @ ..] => super::t3::n546(s, out),
		['p', s @ ..] => super::t3::n547(s, out),
		['r', s @ ..] => super::t3::n548(s, out),
		['s', s @ ..] => super::t3::n549(s, out),
		['t', s @ ..] => super::t3::n550(s, out),
		['u', s @ ..] => super::t3::n551(s, out),
		['v', s @ ..] => super::t3::n552(s, out),
		['x', s @ ..] => super::t3::n553(s, out),
		_ => false,
	}
}

/// Words ending -id.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['e', 't', 'i', 'c', 'o', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2308);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5329);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2310);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5331);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 550);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2313);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5333);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5334);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2315);
					match s {
						[] => true,
						['o', 'n', s @ ..] => {
							super::push(out, 5336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2316);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5337);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8427);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8428);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 10475);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5338);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8429);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8430);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2317);
					match s {
						[] => true,
						['n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2318);
					match s {
						[] => true,
						['s', 'c', 'a', 'l', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2320);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2321);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8431);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8432);
									s.is_empty()
								}
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
			super::push(out, 553);
			match s {
				[] => true,
				['r', 'e', 'c', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5342);
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 10476);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10477);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5343);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5344);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8433);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 10478);
											s.is_empty()
										}
										['u', 'r', s @ ..] => {
											super::push(out, 10479);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5345);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5346);
							match s {
								[] => true,
								['a', 'c', 't', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10480);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10481);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5347);
							match s {
								[] => true,
								['g', 'i', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 10482);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10483);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8434);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10484);
											s.is_empty()
										}
										['u', 'r', s @ ..] => {
											super::push(out, 10485);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 555);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2323);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2324);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2325);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5348);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8435);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8436);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5349);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5350);
							match s {
								[] => true,
								['t', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5351);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5352);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5353);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8438);
									s.is_empty()
								}
								['g', s @ ..] => super::t3::n9260(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t3::n562(s, out),
		['m', s @ ..] => super::t3::n563(s, out),
		['n', s @ ..] => super::t3::n564(s, out),
		['o', s @ ..] => super::t3::n565(s, out),
		['p', s @ ..] => super::t3::n566(s, out),
		['r', s @ ..] => super::t3::n567(s, out),
		['s', s @ ..] => super::t3::n568(s, out),
		['t', s @ ..] => super::t3::n569(s, out),
		['u', s @ ..] => super::t3::n570(s, out),
		['v', s @ ..] => super::t3::n571(s, out),
		_ => false,
	}
}

/// Words ending -tod.
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 574);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5426);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5427);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10516);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ud.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 576);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 577);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2370);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8477);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2371);
					match s {
						[] => true,
						['t', 'a', 'b', s @ ..] => {
							super::push(out, 5428);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8479);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8480);
									s.is_empty()
								}
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
			super::push(out, 578);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2372);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 5429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2373);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 579);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 580);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 581);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5430);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 8481);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8482);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8483);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11595);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11596);
													s.is_empty()
												}
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
							super::push(out, 5431);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5432);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8484);
									match s {
										[] => true,
										['r', 'e', s @ ..] => {
											super::push(out, 10518);
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
										['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5433);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 10519);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10520);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 582);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 583);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['d', 'e', s @ ..] => {
					match s {
						[] => true,
						['c', 'e', 'n', 'n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5434);
							s.is_empty()
						}
						['n', 'n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5435);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 5436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 585);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2375);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5437);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5438);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 8485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2376);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2377);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 587);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ffe.
pub(crate) fn n611(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2394);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5460);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5461);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 5462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2395);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5463);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5464);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5465);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5466);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8490);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2396);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2397);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5467);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 8492);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8493);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10521);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5468);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8494);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5469);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5470);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10523);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2398);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5472);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5473);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5474);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8496);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2399);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5476);
					match s {
						[] => true,
						['c', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10525);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5477);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11597);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11598);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 10527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2400);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5479);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5480);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8498);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10528);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ge.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2401);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 5481);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2402);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 606);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 607);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ie.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 608);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2404);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2405);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8499);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8500);
									s.is_empty()
								}
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
			super::push(out, 610);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 611);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2406);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8501);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8502);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10530);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10531);
											s.is_empty()
										}
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
					super::push(out, 2407);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8503);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -le.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 612);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2408);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5483);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5484);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 10532);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10533);
											s.is_empty()
										}
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
					super::push(out, 2409);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5485);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2410);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2411);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2412);
					match s {
						[] => true,
						['t', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2414);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8505);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8506);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2415);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5488);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10534);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10535);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2416);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2417);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2418);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2419);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2420);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5490);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2421);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8508);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8509);
									s.is_empty()
								}
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
			super::push(out, 614);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 615);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2422);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2423);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2424);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2425);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 5492);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10536);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10537);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5494);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2426);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2428);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2429);
					match s {
						[] => true,
						['a', 't', 'u', 'r', s @ ..] => {
							super::push(out, 5496);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8510);
									s.is_empty()
								}
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
			super::push(out, 617);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2430);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5497);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8511);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5498);
							match s {
								[] => true,
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10538);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10539);
											s.is_empty()
										}
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
					super::push(out, 2431);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 2432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 618);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2433);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5499);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8513);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2434);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -me.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2436);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2437);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5500);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8515);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 5501);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5502);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2438);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 621);
			match s {
				[] => true,
				['l', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2441);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2442);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5504);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8517);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8518);
									match s {
										[] => true,
										['c', 'a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t3::n14055(s, out),
														['s', s @ ..] => super::t3::n14056(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													match s {
														[] => true,
														['r', s @ ..] => super::t3::n14057(s, out),
														['s', s @ ..] => super::t3::n14058(s, out),
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
									super::push(out, 8519);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5505);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8520);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8521);
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
							super::push(out, 5506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2443);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2444);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2445);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8522);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8523);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8524);
									s.is_empty()
								}
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
			super::push(out, 623);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2446);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8525);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2447);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5507);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 624);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2448);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2449);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2450);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 625);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2452);
					match s {
						[] => true,
						['y', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8527);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8528);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 626);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 627);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2455);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2456);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5509);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5510);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8529);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5511);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5512);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8531);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8532);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2457);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8533);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ne.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2458);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2459);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2460);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5513);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8535);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8536);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 629);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5515);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 5516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 631);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 632);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2462);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2463);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8537);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 633);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2465);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8539);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8540);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8541);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2467);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8542);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8543);
									s.is_empty()
								}
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
			super::push(out, 634);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 635);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2468);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2469);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2470);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2471);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5517);
							match s {
								[] => true,
								['i', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8544);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2472);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2473);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pe.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2474);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5518);
							match s {
								[] => true,
								['e', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8545);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2476);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2477);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2478);
					match s {
						[] => true,
						['e', 'p', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2479);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 639);
			match s {
				[] => true,
				['t', 'a', 't', s @ ..] => {
					super::push(out, 2481);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8546);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8547);
									s.is_empty()
								}
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
			super::push(out, 640);
			match s {
				[] => true,
				['l', 'a', 't', s @ ..] => {
					super::push(out, 2482);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8548);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8549);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2483);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5520);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5521);
							match s {
								[] => true,
								['c', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 10540);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10541);
											s.is_empty()
										}
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
					super::push(out, 2484);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5522);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2485);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2486);
					match s {
						[] => true,
						['i', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2487);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5525);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8550);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8551);
									s.is_empty()
								}
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
			super::push(out, 643);
			match s {
				[] => true,
				['c', 'a', 'c', 'e', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2488);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2489);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2490);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2492);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2493);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8552);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8553);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 10542);
											s.is_empty()
										}
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
					super::push(out, 2494);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2496);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5526);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5527);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5528);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 646);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2498);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2499);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2500);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8555);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 11599);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11600);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 11601);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 11602);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11603);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2501);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2502);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2503);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2504);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2505);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 647);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 648);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 649);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2507);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2508);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8556);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2509);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2510);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2511);
					match s {
						[] => true,
						['l', 'e', 'n', 't', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8558);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8559);
									s.is_empty()
								}
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
			super::push(out, 650);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 651);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 652);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['e', 'n', 't', 'i', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2513);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2514);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 655);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5530);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8560);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -te.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 656);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2515);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 657);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 659);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 660);
			match s {
				[] => true,
				['h', 'a', 'r', 'i', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['g', s @ ..] => {
			super::push(out, 661);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2517);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2518);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2519);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 5532);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ve.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2521);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2522);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2523);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5534);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8562);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5535);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 10543);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10544);
											s.is_empty()
										}
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
					super::push(out, 2524);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5536);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5537);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2525);
					match s {
						[] => true,
						['o', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 8564);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 664);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2527);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2528);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2529);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2530);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8566);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8567);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2531);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8569);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8570);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2532);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8571);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8572);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10545);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10546);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 666);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2533);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8573);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2534);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5539);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8575);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5540);
							match s {
								[] => true,
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10547);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10548);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 5541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 667);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xe.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 668);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2536);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5542);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5543);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5544);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8577);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2537);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2538);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2539);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2540);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5545);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2541);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2542);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5547);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8579);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8580);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 5548);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2543);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2544);
					match s {
						[] => true,
						['c', 't', 'o', 'r', 'a', 't', s @ ..] => {
							super::push(out, 5550);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10549);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10550);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 669);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2545);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2546);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5551);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5552);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2547);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5554);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10551);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 10552);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5555);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5556);
							s.is_empty()
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 5557);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8581);
									s.is_empty()
								}
								['u', 'r', s @ ..] => {
									super::push(out, 8582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2548);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5558);
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 10553);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10554);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5559);
							match s {
								[] => true,
								['d', 'e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10555);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10556);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10557);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11604);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11605);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2549);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5560);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5561);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5562);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5563);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 5564);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2550);
					match s {
						[] => true,
						['e', s @ ..] => super::t3::n5924(s, out),
						['i', s @ ..] => super::t3::n5925(s, out),
						['u', s @ ..] => {
							super::push(out, 5568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t3::n2619(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n677(s, out),
		['f', s @ ..] => super::t3::n678(s, out),
		['h', s @ ..] => super::t3::n679(s, out),
		['i', s @ ..] => super::t3::n680(s, out),
		['o', s @ ..] => super::t3::n681(s, out),
		['p', s @ ..] => super::t3::n682(s, out),
		['q', s @ ..] => super::t3::n683(s, out),
		['s', s @ ..] => super::t3::n684(s, out),
		['t', s @ ..] => super::t3::n685(s, out),
		['u', s @ ..] => super::t3::n686(s, out),
		_ => false,
	}
}

/// Words ending -otacirbaf.
pub(crate) fn n13406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11636);
	s.is_empty()
}

/// Words ending -utacirbaf.
pub(crate) fn n13407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11637);
	s.is_empty()
}

/// Words ending -llaf.
pub(crate) fn n2684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2615);
	match s {
		[] => true,
		['a', 'c', s @ ..] => {
			super::push(out, 5667);
			match s {
				[] => true,
				['i', 'o', s @ ..] => {
					super::push(out, 8648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5668);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -slaf.
pub(crate) fn n2685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2616);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5669);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8649);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 8650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5670);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -maf.
pub(crate) fn n692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 685);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2617);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2618);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5671);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5672);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 't', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5673);
					match s {
						[] => true,
						['i', 'a', 'r', 'i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10608);
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2619);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8651);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2620);
			match s {
				[] => true,
				['l', 'a', 'b', s @ ..] => {
					super::push(out, 5674);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -naf.
pub(crate) fn n693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 686);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2621);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2622);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2623);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2624);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -raf.
pub(crate) fn n694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 687);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2625);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8653);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2626);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2627);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2628);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -saf.
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 688);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2629);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2630);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2631);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5675);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8655);
							s.is_empty()
						}
						['g', 'a', 'n', s @ ..] => {
							super::push(out, 8656);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10609);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10610);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5676);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -taf.
pub(crate) fn n696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 689);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2632);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2633);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2634);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5677);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5678);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5679);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5680);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5681);
					match s {
						[] => true,
						['c', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10611);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10612);
									s.is_empty()
								}
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
			super::push(out, 2635);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5682);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8657);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5683);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5684);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5685);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vaf.
pub(crate) fn n698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2636);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2637);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ef.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8659);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 693);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2638);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5687);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5688);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8661);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 5689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2639);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5690);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8663);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8664);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8665);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 695);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2640);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8667);
									s.is_empty()
								}
								['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2642);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 697);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 698);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2643);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5693);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2644);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2645);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2646);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5695);
							match s {
								[] => true,
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10613);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10614);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 5696);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5697);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5698);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8669);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8670);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2647);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2648);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5700);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 5701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2649);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5702);
							match s {
								[] => true,
								['t', 'i', s @ ..] => {
									super::push(out, 8671);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5703);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2650);
					match s {
						[] => true,
						['i', 'l', 'i', 's', s @ ..] => {
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
				['v', s @ ..] => {
					super::push(out, 2651);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 8672);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8673);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 699);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2652);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2653);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2654);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8674);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t3::n14064(s, out),
														['s', s @ ..] => super::t3::n14065(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 11638);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => super::t3::n12039(s, out),
										_ => false,
									}
								}
								['v', s @ ..] => super::t3::n9569(s, out),
								_ => false,
							}
						}
						['u', s @ ..] => super::t3::n6082(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t3::n707(s, out),
		['u', s @ ..] => super::t3::n708(s, out),
		_ => false,
	}
}

/// Words ending -if.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2658);
					s.is_empty()
				}
				['u', 'l', 'a', 'r', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 702);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2660);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2661);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2663);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5713);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5714);
							match s {
								[] => true,
								['i', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10618);
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
						['n', s @ ..] => {
							super::push(out, 5715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2664);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2665);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5716);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 704);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2666);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2667);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2668);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5718);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2669);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5720);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 5721);
							s.is_empty()
						}
						['f', 'o', 'r', 'm', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5722);
							s.is_empty()
						}
						['p', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 707);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2670);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2671);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2672);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2673);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5724);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'v', s @ ..] => {
									super::push(out, 8677);
									s.is_empty()
								}
								['u', 'r', s @ ..] => {
									super::push(out, 8678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 5725);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10619);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10620);
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
		['s', s @ ..] => {
			super::push(out, 710);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2675);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2676);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 711);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lf.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 712);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2678);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2679);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2680);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5726);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5727);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10621);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 10622);
											match s {
												[] => true,
												['o', 's', s @ ..] => {
													match s {
														[] => true,
														['i', s @ ..] => super::t3::n14066(s, out),
														['u', s @ ..] => super::t3::n14067(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2681);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5729);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5730);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 8679);
									match s {
										[] => true,
										['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10623);
											s.is_empty()
										}
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
					super::push(out, 2682);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2683);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5731);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8680);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5732);
							match s {
								[] => true,
								['r', 'u', s @ ..] => {
									super::push(out, 8682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2684);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5733);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5734);
							match s {
								[] => true,
								['p', 'u', s @ ..] => {
									super::push(out, 8684);
									match s {
										[] => true,
										['n', 'c', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10624);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'l', s @ ..] => {
							super::push(out, 5735);
							match s {
								[] => true,
								['i', 'n', 'e', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8685);
									s.is_empty()
								}
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
			super::push(out, 713);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2685);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2686);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 714);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2688);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2689);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5736);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8686);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8687);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 10625);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 10626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 716);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2690);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2691);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2692);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2693);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2694);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2695);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -of.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 717);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2696);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 718);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2698);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 719);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 2700);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5738);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 720);
			match s {
				[] => true,
				['i', 'a', 't', 'i', s @ ..] => {
					super::push(out, 2701);
					match s {
						[] => true,
						['c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 721);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 722);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2702);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5741);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5742);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5743);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8688);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8689);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2703);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2704);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2705);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5744);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 8690);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11639);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11640);
													s.is_empty()
												}
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
							super::push(out, 5745);
							match s {
								[] => true,
								['c', 'a', s @ ..] => {
									super::push(out, 8691);
									s.is_empty()
								}
								['d', 'a', 'n', s @ ..] => {
									super::push(out, 8692);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10627);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10628);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'e', 'n', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 8693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5746);
							match s {
								[] => true,
								['n', 's', 'i', s @ ..] => {
									super::push(out, 8694);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10629);
											s.is_empty()
										}
										['i', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 11641);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11642);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 10630);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2706);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2707);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2708);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5748);
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
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5749);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8695);
									s.is_empty()
								}
								['n', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11643);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11644);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 10631);
											match s {
												[] => true,
												['i', 's', s @ ..] => {
													super::push(out, 11645);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 11646);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 723);
			match s {
				[] => true,
				['s', 'a', s @ ..] => {
					super::push(out, 2709);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5750);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 724);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rf.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 726);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2710);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5752);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2711);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2712);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5754);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5755);
							match s {
								[] => true,
								['a', 'n', 's', s @ ..] => {
									super::push(out, 8696);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10632);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5756);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10633);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10634);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 8697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2713);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2714);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2715);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 727);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2717);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2718);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5757);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8698);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 5758);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 2719);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2720);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					super::push(out, 2721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 728);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2722);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5760);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5761);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8700);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2723);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5762);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8702);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8703);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10635);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5763);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10636);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 10637);
											s.is_empty()
										}
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
					super::push(out, 2724);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 5764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2725);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2726);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 2727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 729);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2728);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8704);
									s.is_empty()
								}
								['u', 'o', s @ ..] => {
									super::push(out, 8705);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 11647);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 11648);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2729);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2730);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5765);
							match s {
								[] => true,
								['l', 'i', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10638);
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
						['i', s @ ..] => {
							super::push(out, 5766);
							match s {
								[] => true,
								['f', 'e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
				['i', s @ ..] => {
					super::push(out, 2731);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2732);
					s.is_empty()
				}
				['n', s @ ..] => super::t3::n2806(s, out),
				['s', s @ ..] => super::t3::n2807(s, out),
				['t', s @ ..] => super::t3::n2808(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uf.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 731);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5770);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5771);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10642);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10643);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 732);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2736);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5772);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5773);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2737);
					match s {
						[] => true,
						['t', 'i', s @ ..] => {
							super::push(out, 5775);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10644);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10645);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 733);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 734);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2738);
					match s {
						[] => true,
						['g', 'i', 'n', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'e', s @ ..] => {
					super::push(out, 2739);
					match s {
						[] => true,
						['s', 'c', 'e', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 5777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 735);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2740);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5778);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5779);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5780);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2741);
					match s {
						[] => true,
						['g', 'a', 'b', s @ ..] => {
							super::push(out, 5781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 736);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2742);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5782);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2743);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2744);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 8712);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8713);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2745);
					match s {
						[] => true,
						['r', 'a', 't', s @ ..] => {
							super::push(out, 5785);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8715);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8716);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 737);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2747);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5787);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5788);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2748);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2749);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2750);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5790);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5791);
							match s {
								[] => true,
								['s', 'i', 's', s @ ..] => {
									super::push(out, 8717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2751);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2752);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 738);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2754);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5793);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8718);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5794);
							match s {
								[] => true,
								['s', 'c', 'e', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 8720);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2755);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2756);
					match s {
						[] => true,
						['i', 'g', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 8721);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8722);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2758);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2759);
					match s {
						[] => true,
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ih.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 791);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2828);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 792);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5855);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5856);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8758);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8759);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['m', 'a', 't', s @ ..] => {
					super::push(out, 2831);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8760);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8761);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['a', 'r', 'i', 's', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 795);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['n', 'i', 'b', s @ ..] => {
					super::push(out, 2833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 797);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2834);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2835);
					match s {
						[] => true,
						['u', 't', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8762);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10655);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10656);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 8763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 798);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2838);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 5858);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8764);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2839);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oh.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 799);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2840);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2841);
					match s {
						[] => true,
						['c', 'i', 'd', 'i', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2843);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2844);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2845);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2846);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8766);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 10657);
											match s {
												[] => true,
												['i', 'l', 'i', s @ ..] => {
													match s {
														[] => true,
														['o', s @ ..] => super::t3::n14068(s, out),
														['s', s @ ..] => super::t3::n14069(s, out),
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
									super::push(out, 8767);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8768);
									s.is_empty()
								}
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
			super::push(out, 802);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2847);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2848);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2849);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5861);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2850);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2851);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5863);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8769);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5864);
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 's', s @ ..] => {
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
				['t', s @ ..] => {
					super::push(out, 2852);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5865);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8771);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['o', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10658);
											s.is_empty()
										}
										['u', 'r', s @ ..] => {
											super::push(out, 10659);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5866);
							match s {
								[] => true,
								['n', 's', 'i', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 10660);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10661);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['p', 'i', 't', 'a', 'l', s @ ..] => {
					super::push(out, 2853);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8772);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uh.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2854);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5868);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8773);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8774);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8775);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5869);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10662);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10663);
											s.is_empty()
										}
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
					super::push(out, 2855);
					match s {
						[] => true,
						['c', 't', 'u', s @ ..] => {
							super::push(out, 5870);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5871);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2856);
					match s {
						[] => true,
						['d', 'u', s @ ..] => {
							super::push(out, 5872);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yh.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5873);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 10664);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 10665);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 5874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 805);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5875);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 806);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2857);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2858);
					match s {
						[] => true,
						['d', 'e', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8780);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['e', 'r', 'g', s @ ..] => {
					super::push(out, 2860);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2861);
					match s {
						[] => true,
						['o', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rutatnei.
pub(crate) fn n13432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11656);
	s.is_empty()
}

/// Words ending -sutatnei.
pub(crate) fn n13433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11657);
	s.is_empty()
}

/// Words ending -ongi.
pub(crate) fn n2972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2880);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 5890);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5891);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8792);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10668);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -li.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 824);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2881);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8793);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8794);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 5892);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5893);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5894);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 5895);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8796);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10670);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10671);
											s.is_empty()
										}
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
					super::push(out, 2882);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2883);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5897);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 5898);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5899);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5900);
							s.is_empty()
						}
						['m', 'i', 't', s @ ..] => {
							super::push(out, 5901);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5902);
							s.is_empty()
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 5903);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2884);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2885);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5905);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 8797);
									match s {
										[] => true,
										['r', 'i', 's', s @ ..] => {
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
								['u', s @ ..] => {
									super::push(out, 8798);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10672);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10673);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mi.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 826);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2886);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5906);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5907);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2887);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2888);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2889);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5909);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 5910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 828);
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5911);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5912);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8799);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10674);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10675);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 829);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2890);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5913);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5914);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8800);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10676);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11658);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11659);
													match s {
														[] => true,
														['i', 'm', 'u', 's', '\u{1}', s @ ..] => super::t3::n14070(s, out),
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
									super::push(out, 8801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5915);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2891);
					match s {
						[] => true,
						['m', 'o', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 't', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5917);
							match s {
								[] => true,
								['s', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'a', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 11660);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 11661);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 10677);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2892);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2893);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5919);
							s.is_empty()
						}
						['l', 'a', 't', 'i', s @ ..] => {
							super::push(out, 5920);
							match s {
								[] => true,
								['c', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8803);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2894);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5922);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8804);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 10678);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10679);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t3::n841(s, out),
		['u', s @ ..] => {
			super::push(out, 831);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ni.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2902);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5948);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2903);
					match s {
						[] => true,
						['q', 'u', 'a', 'l', s @ ..] => {
							super::push(out, 5950);
							match s {
								[] => true,
								['i', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10694);
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
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2904);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2905);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5951);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5952);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5953);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2906);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8825);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 8826);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 8827);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8828);
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
									super::push(out, 8829);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11669);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11670);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2907);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5955);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 8830);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 8831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 5956);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5957);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 833);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					super::push(out, 2909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['a', s @ ..] => super::t3::n3003(s, out),
				['e', s @ ..] => super::t3::n3004(s, out),
				['h', s @ ..] => super::t3::n3005(s, out),
				['i', s @ ..] => super::t3::n3006(s, out),
				['l', s @ ..] => super::t3::n3007(s, out),
				['o', s @ ..] => super::t3::n3008(s, out),
				['r', s @ ..] => super::t3::n3009(s, out),
				['u', s @ ..] => super::t3::n3010(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t3::n846(s, out),
		['e', s @ ..] => super::t3::n847(s, out),
		['f', s @ ..] => super::t3::n848(s, out),
		['g', s @ ..] => super::t3::n849(s, out),
		['h', s @ ..] => super::t3::n850(s, out),
		['i', s @ ..] => super::t3::n851(s, out),
		['l', s @ ..] => super::t3::n852(s, out),
		['m', s @ ..] => super::t4::n853(s, out),
		['n', s @ ..] => super::t4::n854(s, out),
		['o', s @ ..] => super::t4::n855(s, out),
		['p', s @ ..] => super::t4::n856(s, out),
		['q', s @ ..] => super::t4::n857(s, out),
		['r', s @ ..] => super::t4::n858(s, out),
		['s', s @ ..] => super::t4::n859(s, out),
		['t', s @ ..] => super::t4::n860(s, out),
		['u', s @ ..] => super::t4::n861(s, out),
		['v', s @ ..] => super::t4::n862(s, out),
		_ => false,
	}
}

/// Words ending -oi.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3013);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6181);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6182);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 9006);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3014);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6183);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6184);
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9008);
									s.is_empty()
								}
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
			super::push(out, 853);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3015);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3016);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6185);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3018);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ri.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3019);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['d', 'e', 's', 'c', 'e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 3021);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3023);
					match s {
						[] => true,
						['t', 'i', 'o', 'n', 'a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9009);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 9010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3024);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6187);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6188);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9011);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 6189);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6190);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6191);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9013);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 9014);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 9015);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['q', s @ ..] => {
							super::push(out, 6192);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6193);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6194);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 6195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3025);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6196);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6197);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9017);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10802);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10803);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6198);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9018);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3026);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6199);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3027);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -si.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 859);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 860);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6201);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 862);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6203);
							s.is_empty()
						}
						['h', 'e', 'l', 'i', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ti.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 864);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6205);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6207);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 6208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 866);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 867);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6209);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ui.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3028);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3029);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['u', 'n', 'd', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6211);
							s.is_empty()
						}
						['i', 's', s @ ..] => {
							super::push(out, 6212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3031);
					s.is_empty()
				}
				['i', 'c', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6213);
							s.is_empty()
						}
						['t', 'o', s @ ..] => {
							super::push(out, 6214);
							match s {
								[] => true,
								['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9021);
									s.is_empty()
								}
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
			super::push(out, 871);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3032);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6215);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6216);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10804);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10805);
											s.is_empty()
										}
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
					super::push(out, 3033);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 872);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6217);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6218);
							match s {
								[] => true,
								['i', 'n', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3035);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3036);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6219);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6220);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 9023);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3037);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6222);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6223);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10806);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10807);
											s.is_empty()
										}
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
					super::push(out, 3038);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 6224);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10808);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 10809);
											s.is_empty()
										}
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
					super::push(out, 3039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3040);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3041);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 9024);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9025);
									match s {
										[] => true,
										['i', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10810);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 9026);
									s.is_empty()
								}
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
			super::push(out, 876);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 877);
			match s {
				[] => true,
				['e', 'n', 'i', 's', s @ ..] => {
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
		['x', s @ ..] => {
			super::push(out, 878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -isoirobal.
pub(crate) fn n13498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11699);
	s.is_empty()
}

/// Words ending -usoirobal.
pub(crate) fn n13499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11700);
	s.is_empty()
}

