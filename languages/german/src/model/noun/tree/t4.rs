use flexion::Path;

/// Words ending -no.
pub(crate) fn n335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2440);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2441);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8880);
					match s {
						[] => true,
						['i', 'r', 'e', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 19061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8881);
					match s {
						[] => true,
						['i', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 19062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8882);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 19063);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 19064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 8883);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 19065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8884);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8885);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 19066);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 19067);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 19068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8886);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8887);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8888);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 19069);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 19070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2442);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 8889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2443);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2444);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2445);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8892);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 8893);
					match s {
						[] => true,
						['n', 'o', 'r', 'e', 'p', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 19071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8894);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19072);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 19073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8895);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 19074);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 28915);
									match s {
										[] => true,
										['a', 'r', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 36068);
											s.is_empty()
										}
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
					super::push(out, 8896);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8897);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 19075);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 28916);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 28917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8898);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8899);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 19076);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 19077);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 19078);
							match s {
								[] => true,
								['t', 't', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 28918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8900);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19079);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 19080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8901);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 19081);
							match s {
								[] => true,
								['e', 'r', 'a', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 28919);
									s.is_empty()
								}
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
			super::push(out, 2446);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2447);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2448);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 19082);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 19083);
							match s {
								[] => true,
								['k', 'a', s @ ..] => {
									super::push(out, 28920);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 36069);
											s.is_empty()
										}
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
					super::push(out, 8902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2449);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8903);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 19084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8904);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 19085);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 19086);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 28921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8905);
					match s {
						[] => true,
						['t', 't', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 19087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2450);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 8906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['í', s @ ..] => {
			super::push(out, 2451);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oo.
pub(crate) fn n336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2452);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2453);
			match s {
				[] => true,
				['a', 'd', 'n', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8909);
					s.is_empty()
				}
				['r', 'e', 't', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 8910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2454);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2455);
			match s {
				[] => true,
				['t', 'a', 't', '-', s @ ..] => {
					super::push(out, 8911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2456);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2457);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 8913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2458);
			match s {
				[] => true,
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 8914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2459);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 19089);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28922);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 19090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28923);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 28924);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 28925);
									s.is_empty()
								}
								['g', 'n', s @ ..] => {
									super::push(out, 28926);
									s.is_empty()
								}
								['h', s @ ..] => {
									match s {
										[] => true,
										['c', 'e', 'r', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 36070);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 36071);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'h', s @ ..] => {
									super::push(out, 28927);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 28928);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 36072);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 40564);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 36073);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 28929);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 36074);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 28930);
									s.is_empty()
								}
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
			super::push(out, 2460);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2461);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8915);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2462);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 8917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2463);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 8918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 2464);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 8919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2466);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 8920);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8921);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8922);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2467);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2468);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2469);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2470);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8925);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 19091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8926);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8927);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8928);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 19092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8929);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8930);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19093);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 19094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8931);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 19095);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 28932);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 28933);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 36075);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 19096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8932);
					s.is_empty()
				}
				['u', 'q', 'a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8933);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2471);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2472);
			s.is_empty()
		}
		['h', 'c', 'n', 'y', 's', '\u{1}', s @ ..] => {
			super::push(out, 2473);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2474);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2475);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2476);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2477);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2478);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8937);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8938);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2479);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 8940);
					match s {
						[] => true,
						['o', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 19097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8941);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 19098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8942);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 19099);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 19100);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 19101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2480);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8943);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8944);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8945);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 2481);
			s.is_empty()
		}
		['õ', s @ ..] => {
			super::push(out, 2482);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 2483);
			match s {
				[] => true,
				['b', 'r', 'u', s @ ..] => {
					super::push(out, 8946);
					match s {
						[] => true,
						['e', 'i', 'n', 'e', 'g', 'n', 'i', 'h', s @ ..] => {
							super::push(out, 19103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2484);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2485);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8947);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8948);
					match s {
						[] => true,
						['r', 'e', 'b', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 19104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2486);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8949);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8950);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2487);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8952);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 19105);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 19106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2488);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8953);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 19107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2489);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8955);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 19108);
							match s {
								[] => true,
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 28934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 19109);
							match s {
								[] => true,
								['x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 28935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 19110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8956);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2490);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2491);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8958);
					s.is_empty()
				}
				['i', 'h', 'c', 'c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8959);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8960);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19111);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 19112);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 19113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8961);
					match s {
						[] => true,
						['e', 'd', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 19114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 'p', 'h', 'c', 'u', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2492);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8963);
					match s {
						[] => true,
						['v', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 19115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 8964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2493);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2494);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8966);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 8967);
					match s {
						[] => true,
						['f', 'a', 'r', s @ ..] => {
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 28936);
									s.is_empty()
								}
								['g', 's', '\u{1}', s @ ..] => {
									super::push(out, 28937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8968);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 8969);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8970);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8971);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8972);
					match s {
						[] => true,
						['r', 'u', 'b', 's', 'k', 'c', 'ü', 't', 's', 'h', 'ü', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 19116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2495);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2496);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8974);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 19117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8975);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 19118);
							match s {
								[] => true,
								['o', 't', 't', 'o', '\u{1}', s @ ..] => {
									super::push(out, 28938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 19119);
							match s {
								[] => true,
								['m', 'a', 't', 'n', 'u', 'y', 'a', '\u{1}', s @ ..] => {
									super::push(out, 28939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 19120);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 36076);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 36077);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 36078);
											match s {
												[] => true,
												['r', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 40565);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 28940);
									s.is_empty()
								}
								['i', 'g', 'r', 'o', 's', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 28941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8976);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 19121);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 19122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 19123);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 19124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8977);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', s @ ..] => {
									super::push(out, 28943);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 28944);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36079);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 36080);
											match s {
												[] => true,
												['n', 'u', 'l', s @ ..] => {
													super::push(out, 40566);
													match s {
														[] => true,
														['h', 'a', 'z', '\u{1}', s @ ..] => super::t9::n45911(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2497);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8978);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 19125);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 28945);
									match s {
										[] => true,
										['z', 'a', 'r', 'a', 'p', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 36081);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 19126);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 28946);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 28947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 28948);
									s.is_empty()
								}
								['g', 'n', 'u', 't', s @ ..] => {
									super::push(out, 28949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'k', 'a', s @ ..] => {
							super::push(out, 19127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8979);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8980);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8981);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 19130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8982);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 19131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2498);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8983);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8984);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => super::t9::n30222(s, out),
								['s', s @ ..] => super::t9::n30223(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t9::n2503(s, out),
		['t', s @ ..] => super::t9::n2504(s, out),
		['u', s @ ..] => super::t9::n2505(s, out),
		['ä', s @ ..] => super::t9::n2506(s, out),
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2503);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2504);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2505);
			match s {
				[] => true,
				['i', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 2506);
			match s {
				[] => true,
				['o', 'r', 'p', 'i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 8997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vo.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 342);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2507);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2508);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 8998);
					match s {
						[] => true,
						['n', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 19149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xo.
pub(crate) fn n343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 343);
	match s {
		[] => true,
		['x', s @ ..] => {
			super::push(out, 2510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yo.
pub(crate) fn n344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 344);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2511);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2512);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8999);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2513);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2514);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 19150);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 19151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2515);
			match s {
				[] => true,
				['b', 'm', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19152);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 19153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zo.
pub(crate) fn n345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 345);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2516);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9001);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2517);
			match s {
				[] => true,
				['b', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2518);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2519);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 9004);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 9005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2520);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9006);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 19154);
							s.is_empty()
						}
						['l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 19155);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 19156);
							match s {
								[] => true,
								['r', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 28960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9007);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ño.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['e', 'p', 'a', 'l', 'a', 'j', '\u{1}', s @ ..] => {
			super::push(out, 2521);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word gesamthandsprinzip.
pub(crate) fn n43114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40568);
	s.is_empty()
}

/// The word retardierungsprinzip.
pub(crate) fn n43115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40569);
	s.is_empty()
}

/// The word meistausführungsprinzip.
pub(crate) fn n43116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40570);
	s.is_empty()
}

/// Words ending -tsprinzip.
pub(crate) fn n38095(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'e', 'k', 'g', s @ ..] => {
			super::push(out, 40571);
			s.is_empty()
		}
		['ä', 't', 'i', s @ ..] => {
			match s {
				[] => true,
				['r', 'a', 'd', s @ ..] => {
					super::push(out, 43084);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 43085);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lp.
pub(crate) fn n354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2552);
			match s {
				[] => true,
				['k', 's', '\u{1}', s @ ..] => {
					super::push(out, 9037);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 9038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2553);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2554);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 9040);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 9041);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mp.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2555);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9042);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9043);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9044);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 9045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2556);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2557);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9046);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 19161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2558);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9047);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9048);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 9049);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19163);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 19164);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 19165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -np.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	s.is_empty()
}

/// Words ending -op.
pub(crate) fn n357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2559);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 2560);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2561);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9051);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9052);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 19166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 2562);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2563);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 9053);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 19167);
							match s {
								[] => true,
								['i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 28965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9054);
					match s {
						[] => true,
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 19168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2564);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 19169);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 19170);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 19171);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 28966);
									match s {
										[] => true,
										['e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 36087);
											s.is_empty()
										}
										['i', 'e', 'l', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 36088);
											s.is_empty()
										}
										['n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 36089);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 28967);
									s.is_empty()
								}
								['l', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 28968);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 28969);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 36090);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 28970);
									match s {
										[] => true,
										['k', 'i', 'm', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 40572);
													s.is_empty()
												}
												['n', 'e', 'n', 'o', 'r', 't', 'k', 'e', 'l', 'e', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t9::n45914(s, out),
														['r', 'e', 't', 's', 'a', 'r', '\u{1}', s @ ..] => super::t9::n45915(s, out),
														['s', 'n', 'o', 'i', 's', 's', 'i', 'm', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => super::t9::n45916(s, out),
														_ => false,
													}
												}
												['t', 'h', 'c', 'i', 'l', '\u{1}', s @ ..] => {
													super::push(out, 40573);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'h', s @ ..] => {
											match s {
												[] => true,
												['n', 'e', 'h', 'c', 'o', 'w', '\u{1}', s @ ..] => {
													super::push(out, 40574);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 40575);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 36091);
											s.is_empty()
										}
										['y', s @ ..] => {
											match s {
												[] => true,
												['g', '\u{1}', s @ ..] => {
													super::push(out, 40576);
													s.is_empty()
												}
												['p', '\u{1}', s @ ..] => {
													super::push(out, 40577);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 28971);
									s.is_empty()
								}
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
			super::push(out, 2565);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9056);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9057);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 19172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 2566);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2567);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9058);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9059);
					match s {
						[] => true,
						['s', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 19173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19174);
							s.is_empty()
						}
						['e', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 19175);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 19176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2568);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 9062);
					match s {
						[] => true,
						['e', 'c', 'n', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 19177);
							s.is_empty()
						}
						['t', 'i', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 19178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9063);
					s.is_empty()
				}
				['t', 'i', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 9064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2569);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9065);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9066);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9067);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9068);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 28972);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 28973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 9069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2570);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 9070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19179);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 19180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2571);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9071);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 9072);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 9073);
					s.is_empty()
				}
				['k', s @ ..] => super::t9::n9182(s, out),
				['o', s @ ..] => super::t9::n9183(s, out),
				['p', s @ ..] => super::t9::n9184(s, out),
				['s', s @ ..] => super::t9::n9185(s, out),
				['t', s @ ..] => super::t9::n9186(s, out),
				_ => false,
			}
		}
		['w', s @ ..] => super::t9::n2576(s, out),
		_ => false,
	}
}

/// Words ending -pp.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 358);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2573);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9080);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 9081);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 19186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 9082);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9083);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 19187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9084);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 19188);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 19189);
							match s {
								[] => true,
								['ä', 'b', '-', s @ ..] => {
									super::push(out, 28974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9085);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19190);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 19191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9086);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 19192);
							s.is_empty()
						}
						['h', 'c', 's', 'g', '\u{1}', s @ ..] => {
							super::push(out, 19193);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 19194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28975);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 28976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 19195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28977);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 28978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9087);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2574);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9089);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9090);
					match s {
						[] => true,
						['h', 'c', 's', 'n', s @ ..] => {
							super::push(out, 19196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9091);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9092);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 19197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2575);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9095);
					match s {
						[] => true,
						['i', 'h', 'p', 'l', 'e', 'p', 'p', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 19200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9096);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9097);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9098);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 19201);
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 28979);
									s.is_empty()
								}
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
			super::push(out, 2576);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9099);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9100);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 19202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9101);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9102);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19203);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 19204);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 19205);
							match s {
								[] => true,
								['n', 'h', s @ ..] => {
									super::push(out, 28980);
									match s {
										[] => true,
										['o', 'l', 'l', 'a', 'n', 'i', 'm', 'o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 36092);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'u', 'l', 'l', 'e', 't', 's', 'n', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 36093);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 36094);
											match s {
												[] => true,
												['e', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 40578);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2577);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9104);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 19206);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 28981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9105);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 19207);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 19208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 2578);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rp.
pub(crate) fn n359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 359);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2579);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9107);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 28982);
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 28983);
									s.is_empty()
								}
								['t', 'k', 'e', '\u{1}', s @ ..] => {
									super::push(out, 28984);
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 28985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 9108);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sp.
pub(crate) fn n360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 360);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2580);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tp.
pub(crate) fn n361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 2582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -up.
pub(crate) fn n362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2583);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9109);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9110);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							super::push(out, 19209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9111);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9112);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 19210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 2584);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 9113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2585);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 9114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2586);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2587);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2588);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9116);
					match s {
						[] => true,
						['t', 'e', 'k', 'n', 'e', 't', 'a', 'm', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 19211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2589);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19212);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 19213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2590);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 9118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2591);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2592);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9119);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2594);
			match s {
				[] => true,
				['i', 's', 'n', 'e', 'b', s @ ..] => {
					super::push(out, 9122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2595);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vp.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 2596);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9123);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wp.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	s.is_empty()
}

/// Words ending -yp.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 2597);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 9124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2598);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9125);
					match s {
						[] => true,
						['l', 'i', 'w', '-', s @ ..] => {
							super::push(out, 19214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9126);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 19215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9127);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 19216);
							match s {
								[] => true,
								['a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 28986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9128);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 19217);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 28987);
									match s {
										[] => true,
										['r', 'e', 'u', 'g', 'a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 36095);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 19218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9129);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 9130);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ar.
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2604);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2605);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9132);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 19219);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 28988);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 28989);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 28990);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'i', s @ ..] => {
							super::push(out, 19220);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 19221);
							match s {
								[] => true,
								['l', 'e', 'g', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 28991);
									s.is_empty()
								}
								['s', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 28992);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 28993);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['r', 'a', 'b', 'r', s @ ..] => {
									super::push(out, 28994);
									s.is_empty()
								}
								['s', 'u', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 28995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9133);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 19222);
							s.is_empty()
						}
						['r', 'e', 'l', 'd', s @ ..] => {
							super::push(out, 19223);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 19224);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 28996);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 28997);
									s.is_empty()
								}
								['t', 'i', 'e', 'z', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 28998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9134);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 19225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2606);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 9135);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9136);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 19226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9137);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 19227);
							s.is_empty()
						}
						['e', 'f', 'f', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 19228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 9138);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 28999);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 29000);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 19229);
							match s {
								[] => true,
								['t', 's', '\u{1}', s @ ..] => {
									super::push(out, 29001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9139);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9140);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9141);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9142);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9143);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 19231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9144);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19232);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 29002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 19233);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 19234);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29003);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 29004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9145);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2607);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 9147);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9148);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9149);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9150);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 9151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2608);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19235);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 19236);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 29005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 19237);
							match s {
								[] => true,
								['o', 'd', 'b', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 29006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 19238);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 29007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 9153);
					match s {
						[] => true,
						['e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 19239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9154);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9155);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 19240);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 19241);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 29008);
									s.is_empty()
								}
								['q', s @ ..] => {
									super::push(out, 29009);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9156);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 19242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 9157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2609);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 9158);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 9159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2610);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2611);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9162);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9163);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9164);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19244);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 19245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t9::n2616(s, out),
		['i', s @ ..] => super::t9::n2617(s, out),
		['j', s @ ..] => super::t9::n2618(s, out),
		['k', s @ ..] => super::t9::n2619(s, out),
		['l', s @ ..] => super::t9::n2620(s, out),
		['m', s @ ..] => super::t9::n2621(s, out),
		['n', s @ ..] => super::t9::n2622(s, out),
		['o', s @ ..] => {
			super::push(out, 2619);
			s.is_empty()
		}
		['p', s @ ..] => super::t9::n2624(s, out),
		['r', s @ ..] => super::t9::n2625(s, out),
		['s', s @ ..] => super::t9::n2626(s, out),
		['t', s @ ..] => super::t9::n2627(s, out),
		['u', s @ ..] => super::t9::n2628(s, out),
		['v', s @ ..] => super::t9::n2629(s, out),
		['w', s @ ..] => super::t9::n2630(s, out),
		['y', s @ ..] => super::t9::n2631(s, out),
		['z', s @ ..] => super::t9::n2632(s, out),
		_ => false,
	}
}

/// Words ending -er.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2630);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2631);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2632);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19323);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 19324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 19325);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 19326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 9235);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19327);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 29049);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36113);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 29050);
									match s {
										[] => true,
										['h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 36114);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 29051);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 29052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 19328);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 19329);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 19330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9236);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 19331);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 19332);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 19333);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 29053);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 36115);
											s.is_empty()
										}
										['l', 'i', s @ ..] => {
											super::push(out, 36116);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 29054);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 36117);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 29055);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36118);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 36119);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 19334);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29056);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 29057);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 29058);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 29059);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 36120);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 29060);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 29061);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 29062);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 29063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 19335);
							match s {
								[] => true,
								['t', 'r', s @ ..] => {
									super::push(out, 29064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 19336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9237);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 19337);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 19338);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29065);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 29066);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 19339);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 29067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9238);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19340);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 29068);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36121);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 29069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 19341);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 29070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t9::n19895(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t9::n9348(s, out),
				['o', s @ ..] => {
					super::push(out, 9240);
					match s {
						[] => true,
						['h', 'p', s @ ..] => super::t9::n19900(s, out),
						['t', s @ ..] => super::t9::n19901(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t9::n9350(s, out),
				['u', s @ ..] => super::t9::n9351(s, out),
				['ä', s @ ..] => super::t9::n9352(s, out),
				['ö', s @ ..] => super::t9::n9353(s, out),
				['ü', s @ ..] => super::t9::n9354(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t9::n2637(s, out),
		['d', s @ ..] => super::t10::n2638(s, out),
		['e', s @ ..] => super::t10::n2639(s, out),
		['f', s @ ..] => super::t10::n2640(s, out),
		['g', s @ ..] => super::t10::n2641(s, out),
		['h', s @ ..] => super::t10::n2642(s, out),
		['i', s @ ..] => super::t10::n2643(s, out),
		['j', s @ ..] => super::t10::n2644(s, out),
		['k', s @ ..] => super::t10::n2645(s, out),
		['l', s @ ..] => super::t10::n2646(s, out),
		['m', s @ ..] => super::t10::n2647(s, out),
		['n', s @ ..] => super::t10::n2648(s, out),
		['o', s @ ..] => super::t10::n2649(s, out),
		['p', s @ ..] => super::t10::n2650(s, out),
		['r', s @ ..] => super::t10::n2651(s, out),
		['s', s @ ..] => super::t10::n2652(s, out),
		['t', s @ ..] => super::t10::n2653(s, out),
		['u', s @ ..] => super::t10::n2654(s, out),
		['v', s @ ..] => super::t10::n2655(s, out),
		['w', s @ ..] => super::t10::n2656(s, out),
		['x', s @ ..] => super::t10::n2657(s, out),
		['y', s @ ..] => super::t10::n2658(s, out),
		['z', s @ ..] => super::t10::n2659(s, out),
		['ß', s @ ..] => super::t10::n2660(s, out),
		['ä', s @ ..] => super::t10::n2661(s, out),
		_ => false,
	}
}

/// Words ending -hr.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2658);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9463);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 19998);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['g', 'n', 'u', 'l', 's', s @ ..] => {
													super::push(out, 40918);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 40919);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 36778);
											match s {
												[] => true,
												['h', 'c', 'u', 'l', s @ ..] => {
													super::push(out, 40920);
													s.is_empty()
												}
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
							super::push(out, 19999);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 29977);
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 36779);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20000);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 29978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 9464);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 20001);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 20002);
							match s {
								[] => true,
								['ü', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 29979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 20003);
							s.is_empty()
						}
						['n', 'e', 'h', 'c', 'o', s @ ..] => {
							super::push(out, 20004);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 20005);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'ü', s @ ..] => {
											super::push(out, 36780);
											s.is_empty()
										}
										['g', 'a', s @ ..] => {
											super::push(out, 36781);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 36782);
											s.is_empty()
										}
										['u', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 36783);
											s.is_empty()
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
								['g', 'n', 'u', 'r', s @ ..] => {
									super::push(out, 29980);
									match s {
										[] => true,
										['h', 'ü', 'f', 'n', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 36784);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', s @ ..] => {
									super::push(out, 29981);
									match s {
										[] => true,
										['i', 't', 'u', s @ ..] => {
											super::push(out, 36785);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 20006);
							match s {
								[] => true,
								['l', 'a', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 29982);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 29983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 20007);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 29984);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 36786);
											s.is_empty()
										}
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
					super::push(out, 9465);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 20008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9466);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t10::n2663(s, out),
		['o', s @ ..] => {
			super::push(out, 2660);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9473);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9474);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 9475);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9476);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 20028);
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 29993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9477);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20029);
							s.is_empty()
						}
						['f', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 20030);
							s.is_empty()
						}
						['r', 'e', 'm', 'm', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 20031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9479);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20032);
							match s {
								[] => true,
								['s', 'e', s @ ..] => {
									super::push(out, 29994);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9480);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9481);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20033);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 29995);
									match s {
										[] => true,
										['n', 'i', 'k', '\u{1}', s @ ..] => {
											super::push(out, 36798);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'r', s @ ..] => {
							super::push(out, 20034);
							match s {
								[] => true,
								['e', 'f', 'l', 'e', 'i', 'z', 'r', s @ ..] => {
									super::push(out, 29996);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 20035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9482);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 9483);
					match s {
						[] => true,
						['t', 'i', 'l', s @ ..] => {
							super::push(out, 20036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2661);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9484);
					match s {
						[] => true,
						['u', 'z', 'r', s @ ..] => {
							super::push(out, 20037);
							match s {
								[] => true,
								['e', 'k', s @ ..] => {
									super::push(out, 29997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9485);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20038);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 20039);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 29998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 20040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => super::t10::n9598(s, out),
				_ => false,
			}
		}
		['ä', s @ ..] => super::t10::n2666(s, out),
		['ö', s @ ..] => super::t10::n2667(s, out),
		['ü', s @ ..] => super::t10::n2668(s, out),
		_ => false,
	}
}

/// Words ending -or.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2680);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9515);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 20048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30000);
									s.is_empty()
								}
								['-', 'h', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 30001);
									s.is_empty()
								}
								['e', 'i', 'm', 'e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 30002);
									s.is_empty()
								}
								['h', 'c', 'a', 'r', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 30003);
									s.is_empty()
								}
								['m', 'i', 'e', 'h', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 30004);
									s.is_empty()
								}
								['n', 'e', 'g', 'o', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 30005);
									s.is_empty()
								}
								['o', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 30006);
									s.is_empty()
								}
								['r', 'e', 't', 'e', 'h', 't', 'a', 'k', 'z', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 30007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 20049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2681);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9517);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 20050);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 20051);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 20052);
							match s {
								[] => true,
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 30008);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20053);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 30009);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 20054);
							match s {
								[] => true,
								['a', 'm', 'l', s @ ..] => {
									super::push(out, 30010);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 30011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9518);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 20055);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 30012);
									match s {
										[] => true,
										['e', 'h', 't', '\u{1}', s @ ..] => {
											super::push(out, 36799);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20056);
							match s {
								[] => true,
								['r', 'o', 'k', 'l', s @ ..] => {
									super::push(out, 30013);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9519);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9520);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9521);
					match s {
						[] => true,
						['h', 'c', 'i', 'r', 'd', 'e', 'i', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 20058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9522);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 20059);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 20060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2682);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9523);
					match s {
						[] => true,
						['e', 'm', 'o', s @ ..] => {
							super::push(out, 20061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2683);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2684);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9524);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 20062);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 20063);
							s.is_empty()
						}
						['s', 'l', s @ ..] => {
							super::push(out, 20064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9525);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20065);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30014);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 20066);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 30015);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 30016);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 36800);
											s.is_empty()
										}
										['i', 'm', 'u', 'l', '\u{1}', s @ ..] => {
											super::push(out, 36801);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 36802);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 20067);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 30017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20068);
							s.is_empty()
						}
						['n', 'e', 's', 'u', 'e', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 20069);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 20070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2685);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9527);
					match s {
						[] => true,
						['a', 'r', 'o', 'n', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 20071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2686);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2687);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9529);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9530);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2688);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9532);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 20072);
							match s {
								[] => true,
								['c', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 30018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 9533);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2689);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9535);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 20073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9536);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 20074);
							match s {
								[] => true,
								['o', 'l', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 30019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9537);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9538);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 20075);
							match s {
								[] => true,
								['n', s @ ..] => super::t10::n31350(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t10::n20644(s, out),
						['t', s @ ..] => super::t10::n20645(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t10::n2694(s, out),
		['o', s @ ..] => super::t10::n2695(s, out),
		['p', s @ ..] => super::t10::n2696(s, out),
		['r', s @ ..] => super::t10::n2697(s, out),
		['s', s @ ..] => super::t10::n2698(s, out),
		['t', s @ ..] => super::t10::n2699(s, out),
		['u', s @ ..] => super::t10::n2700(s, out),
		['v', s @ ..] => super::t10::n2701(s, out),
		_ => false,
	}
}

/// Words ending -lirr.
pub(crate) fn n9698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9585);
	s.is_empty()
}

/// Words ending -wirr.
pub(crate) fn n9699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9586);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 20155);
			match s {
				[] => true,
				['g', 's', s @ ..] => {
					super::push(out, 30074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 20156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urr.
pub(crate) fn n2706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2702);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 9587);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sr.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 2703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tr.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 380);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 2704);
			match s {
				[] => true,
				['e', 'n', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 381);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2705);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2706);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 9589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9590);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 20158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9591);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 20159);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 20160);
							match s {
								[] => true,
								['n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 30075);
									s.is_empty()
								}
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
			super::push(out, 2707);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9592);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 30076);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 30077);
									s.is_empty()
								}
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
			super::push(out, 2708);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 9593);
					match s {
						[] => true,
						['b', 'o', '-', 'r', 'i', 'a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 20161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2709);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9594);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 9595);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 20162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 9596);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9597);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 20164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2710);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9599);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 20165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 9600);
					s.is_empty()
				}
				['g', 'a', 'y', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 9601);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9602);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 20166);
							s.is_empty()
						}
						['e', 'g', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 20167);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 20168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9603);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 20169);
							match s {
								[] => true,
								['e', 'g', 'n', 'i', 'm', s @ ..] => {
									super::push(out, 30078);
									match s {
										[] => true,
										['o', 'l', 'p', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 36838);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20170);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30079);
									match s {
										[] => true,
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 36839);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 20171);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 30080);
									match s {
										[] => true,
										['o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 36840);
											s.is_empty()
										}
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
					super::push(out, 9604);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 20172);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 20173);
							match s {
								[] => true,
								['n', 'o', 'j', '\u{1}', s @ ..] => {
									super::push(out, 30081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 20174);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 20175);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 30082);
									match s {
										[] => true,
										['c', 'r', 'e', 'k', s @ ..] => {
											super::push(out, 36841);
											s.is_empty()
										}
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
					super::push(out, 9605);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 20176);
							match s {
								[] => true,
								['o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 30083);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9606);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20177);
							match s {
								[] => true,
								['r', 'f', 'e', s @ ..] => {
									super::push(out, 30084);
									match s {
										[] => true,
										['d', 'n', 'u', 'h', '\u{1}', s @ ..] => {
											super::push(out, 36842);
											s.is_empty()
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
								['a', 'm', 'n', s @ ..] => {
									super::push(out, 30085);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 30086);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 36843);
											match s {
												[] => true,
												['n', 'n', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 40954);
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', s @ ..] => {
											super::push(out, 36844);
											s.is_empty()
										}
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
					super::push(out, 9607);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 20178);
							match s {
								[] => true,
								['o', 'r', 'r', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 30087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9608);
					match s {
						[] => true,
						['q', 'i', s @ ..] => {
							super::push(out, 20179);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 30088);
									s.is_empty()
								}
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
			super::push(out, 2711);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9609);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2712);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9611);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 20180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9612);
					match s {
						[] => true,
						['f', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', 'b', s @ ..] => {
									super::push(out, 30089);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 30090);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 36845);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t10::n9726(s, out),
				['u', s @ ..] => {
					super::push(out, 9614);
					match s {
						[] => true,
						['a', s @ ..] => super::t10::n20758(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t10::n2717(s, out),
		['k', s @ ..] => super::t10::n2718(s, out),
		['l', s @ ..] => super::t10::n2719(s, out),
		['m', s @ ..] => super::t10::n2720(s, out),
		['n', s @ ..] => super::t10::n2721(s, out),
		['o', s @ ..] => super::t10::n2722(s, out),
		['p', s @ ..] => super::t10::n2723(s, out),
		['s', s @ ..] => super::t10::n2724(s, out),
		['t', s @ ..] => super::t10::n2725(s, out),
		['v', s @ ..] => super::t10::n2726(s, out),
		['w', s @ ..] => super::t10::n2727(s, out),
		['z', s @ ..] => super::t10::n2728(s, out),
		['œ', s @ ..] => super::t10::n2729(s, out),
		_ => false,
	}
}

/// Words ending -yr.
pub(crate) fn n382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2726);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2727);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2728);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2729);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9652);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -är.
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2730);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9653);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 20226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9654);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9655);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20227);
							match s {
								[] => true,
								['l', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 30119);
									s.is_empty()
								}
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
			super::push(out, 2731);
			match s {
				[] => true,
				['i', 'g', 'i', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 9656);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2732);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2733);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2734);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2735);
			match s {
				[] => true,
				['o', 'i', s @ ..] => {
					match s {
						[] => true,
						['l', 'l', 'i', 'm', 'n', s @ ..] => {
							super::push(out, 20228);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30120);
									match s {
										[] => true,
										['t', 't', 'a', 'l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 36862);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 20229);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 30121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 20230);
							match s {
								[] => true,
								['k', 'i', s @ ..] => {
									super::push(out, 30122);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 36863);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2736);
			match s {
				[] => true,
				['s', 'e', 's', 's', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 9660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2737);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9661);
					match s {
						[] => true,
						['r', 'k', 'e', 's', 's', 't', 'a', s @ ..] => {
							super::push(out, 20231);
							match s {
								[] => true,
								['a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 30123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9662);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 30124);
									match s {
										[] => true,
										['m', 'a', s @ ..] => {
											super::push(out, 36864);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 30125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9663);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2738);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20232);
							s.is_empty()
						}
						['d', 'n', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 20233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ér.
pub(crate) fn n384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 384);
	s.is_empty()
}

/// Words ending -ör.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 385);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 2739);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2740);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9666);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 20234);
							match s {
								[] => true,
								['u', 'z', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 36865);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 36866);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 36867);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 36868);
											match s {
												[] => true,
												['g', 'n', 'u', 't', 'f', 'ü', 'l', '\u{1}', s @ ..] => {
													super::push(out, 40961);
													s.is_empty()
												}
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
							super::push(out, 20235);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 30126);
									match s {
										[] => true,
										['z', 'r', s @ ..] => {
											super::push(out, 36869);
											s.is_empty()
										}
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
					super::push(out, 9667);
					match s {
						[] => true,
						['e', 'v', 'i', s @ ..] => {
							super::push(out, 20236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2741);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9668);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9669);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2742);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 9670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2743);
			match s {
				[] => true,
				['i', 'r', 'f', 'e', 'd', 'n', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 9671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2744);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20238);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 30127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 'f', 'f', 'ö', 'l', '\u{1}', s @ ..] => {
							super::push(out, 20239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ür.
pub(crate) fn n386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 386);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 2745);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2746);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2747);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 9674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2748);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 2749);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9675);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ias.
pub(crate) fn n2761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2757);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9691);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 9692);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 20252);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9693);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 20254);
					s.is_empty()
				}
				['i', 'm', 'a', 'f', 'r', 'e', 't', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 20255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9694);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9695);
			match s {
				[] => true,
				['t', 'n', s @ ..] => {
					super::push(out, 20256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9696);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 20257);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kas.
pub(crate) fn n2762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2758);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 9697);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9698);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 9699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -las.
pub(crate) fn n2763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2759);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9700);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 20259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 9701);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9702);
			match s {
				[] => true,
				['b', 'b', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 20260);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 20261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9703);
			match s {
				[] => true,
				['d', 'l', s @ ..] => {
					super::push(out, 20262);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', s @ ..] => {
					super::push(out, 20263);
					match s {
						[] => true,
						['l', 'b', 's', s @ ..] => {
							super::push(out, 30132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'y', s @ ..] => {
					super::push(out, 20264);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 30133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 20265);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30134);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 36870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'b', s @ ..] => {
					super::push(out, 20266);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20267);
					match s {
						[] => true,
						['a', 'o', s @ ..] => {
							super::push(out, 30135);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 30136);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 36871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 30137);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 30138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9704);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20268);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 20269);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 30139);
							match s {
								[] => true,
								['i', 'l', 'o', '\u{1}', s @ ..] => {
									super::push(out, 36872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 30140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9705);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9706);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20271);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30141);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 30142);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 36873);
									match s {
										[] => true,
										['o', 'w', '\u{1}', s @ ..] => {
											super::push(out, 40962);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mas.
pub(crate) fn n2764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2760);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9707);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 20272);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9708);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nas.
pub(crate) fn n2765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2761);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9709);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20274);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 30143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9710);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -roas.
pub(crate) fn n9829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9711);
	s.is_empty()
}

/// Words ending -pas.
pub(crate) fn n2767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2763);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9712);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 20276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9713);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20277);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 20278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9714);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 20279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 9715);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ras.
pub(crate) fn n2768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2764);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9716);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 9717);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9718);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20281);
					match s {
						[] => true,
						['e', 't', 'n', s @ ..] => {
							super::push(out, 30144);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 30145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20282);
					match s {
						[] => true,
						['t', 'r', s @ ..] => {
							super::push(out, 30146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9719);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 20283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9720);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 20284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9721);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9722);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9723);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 20287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sas.
pub(crate) fn n2769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2765);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tas.
pub(crate) fn n2770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2766);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9725);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9726);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20288);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 20289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9727);
			match s {
				[] => true,
				['s', 'o', 'r', s @ ..] => {
					super::push(out, 20290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9728);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9729);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vas.
pub(crate) fn n2771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2767);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9730);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9731);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -was.
pub(crate) fn n2772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2768);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 9732);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xas.
pub(crate) fn n2773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2769);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9733);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yas.
pub(crate) fn n2774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2770);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 9734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bs.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2771);
			match s {
				[] => true,
				['r', 'k', 's', 'b', 'i', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2772);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9736);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 20291);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30147);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9737);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 30148);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'u', 'z', s @ ..] => {
											super::push(out, 36874);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 36875);
											match s {
												[] => true,
												['c', 'o', s @ ..] => {
													super::push(out, 40963);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2773);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2774);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9739);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 20292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cs.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2775);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9740);
					match s {
						[] => true,
						['o', 'n', 'e', s @ ..] => {
							super::push(out, 20293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ds.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2776);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2777);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 9741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2778);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2779);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9743);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 20294);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 20295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9744);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9745);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 20296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2780);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9746);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -es.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2781);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2782);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9747);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 30149);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 30150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9748);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2783);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2784);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9750);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9751);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9752);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 20297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2785);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 9754);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2786);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2787);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9757);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9758);
					match s {
						[] => true,
						['t', 'ü', s @ ..] => {
							super::push(out, 20298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2788);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20299);
							match s {
								[] => true,
								['f', 'l', s @ ..] => {
									super::push(out, 30151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 20300);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30152);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 30153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 20301);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 20302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2789);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9760);
					match s {
						[] => true,
						['a', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 20303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 9761);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 9762);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9763);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 20306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9764);
					match s {
						[] => true,
						['i', 'f', 'f', 'e', '\u{1}', s @ ..] => {
							super::push(out, 20307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 9765);
					match s {
						[] => true,
						['c', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 20308);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 20309);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 20310);
							s.is_empty()
						}
						['r', 'e', 'k', s @ ..] => {
							super::push(out, 20311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9766);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9767);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20312);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 20313);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 30154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 20314);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 20315);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 30155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9769);
					match s {
						[] => true,
						['x', s @ ..] => {
							super::push(out, 20316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 9770);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2790);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9771);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20318);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 20319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2791);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9772);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 20320);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 20321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 9773);
					match s {
						[] => true,
						['a', 'w', s @ ..] => {
							super::push(out, 20322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9774);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2792);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9776);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 20323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 9777);
					s.is_empty()
				}
				['e', 'h', 'c', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 9778);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9779);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 20324);
							s.is_empty()
						}
						['n', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 20325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9780);
					match s {
						[] => true,
						['a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 20326);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 20327);
							match s {
								[] => true,
								['c', 'i', 'm', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 30156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 20328);
							s.is_empty()
						}
						['ä', 'l', '\u{1}', s @ ..] => {
							super::push(out, 20329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2793);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9782);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9783);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9784);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20330);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 20331);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 20332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9785);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2794);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9786);
					match s {
						[] => true,
						['m', 'o', 'r', '\u{1}', s @ ..] => super::t10::n20918(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t10::n9906(s, out),
				['i', 'r', 'a', 'm', '\u{1}', s @ ..] => super::t10::n9907(s, out),
				['n', s @ ..] => super::t10::n9908(s, out),
				['r', s @ ..] => super::t10::n9909(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t10::n2799(s, out),
		['r', s @ ..] => super::t10::n2800(s, out),
		['s', s @ ..] => super::t10::n2801(s, out),
		['t', s @ ..] => super::t10::n2802(s, out),
		['u', s @ ..] => super::t10::n2803(s, out),
		['v', s @ ..] => super::t10::n2804(s, out),
		['z', s @ ..] => super::t10::n2805(s, out),
		['ß', s @ ..] => super::t10::n2806(s, out),
		_ => false,
	}
}

/// Words ending -gs.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2803);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2804);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 20366);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 20367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2805);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hs.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2806);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9820);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 20368);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 20369);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 30163);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 30164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 20370);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 30165);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 30166);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 30167);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 30168);
									match s {
										[] => true,
										['z', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['e', 'l', s @ ..] => super::t10::n46103(s, out),
														['h', s @ ..] => super::t10::n46104(s, out),
														_ => false,
													}
												}
												['s', 'r', s @ ..] => {
													super::push(out, 40965);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ß', s @ ..] => {
									super::push(out, 30169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9821);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 20371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9822);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 20372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9824);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 20374);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 20375);
							match s {
								[] => true,
								['l', 'h', s @ ..] => {
									super::push(out, 30170);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 20376);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 20377);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30171);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 30172);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 30173);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 30174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 9825);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 20378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2807);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2808);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9827);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20379);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 30175);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 30176);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 36878);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 20380);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 20381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9828);
					match s {
						[] => true,
						['r', 'e', 't', s @ ..] => {
							super::push(out, 20382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9830);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9831);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 20383);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 20384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9832);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9833);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2810);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9835);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 20385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2811);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2812);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9837);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 9838);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 9839);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20386);
							s.is_empty()
						}
						['m', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 20387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 9840);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20388);
							match s {
								[] => true,
								['r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 30177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 20389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9841);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 20390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 9842);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 20391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9843);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 20392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 9844);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 20393);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 30178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9845);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20394);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 20395);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 20396);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 30179);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 36879);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9846);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 20398);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 20399);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 20400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9847);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 20401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9848);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 20402);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 30180);
									s.is_empty()
								}
								_ => false,
							}
						}
						['a', s @ ..] => {
							super::push(out, 20403);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 30181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 20404);
							match s {
								[] => true,
								['e', s @ ..] => super::t10::n31525(s, out),
								_ => false,
							}
						}
						['d', s @ ..] => super::t10::n20989(s, out),
						['e', s @ ..] => super::t10::n20990(s, out),
						['f', s @ ..] => super::t10::n20991(s, out),
						['h', s @ ..] => super::t10::n20992(s, out),
						['k', s @ ..] => super::t10::n20993(s, out),
						['n', s @ ..] => super::t10::n20994(s, out),
						['p', s @ ..] => super::t10::n20995(s, out),
						['r', s @ ..] => super::t10::n20996(s, out),
						['s', s @ ..] => super::t10::n20997(s, out),
						['t', s @ ..] => super::t10::n20998(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t10::n9969(s, out),
				['w', s @ ..] => super::t10::n9970(s, out),
				['z', s @ ..] => super::t10::n9971(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t10::n2817(s, out),
		['g', s @ ..] => super::t10::n2818(s, out),
		['h', s @ ..] => super::t10::n2819(s, out),
		['k', s @ ..] => super::t10::n2820(s, out),
		['l', s @ ..] => super::t10::n2821(s, out),
		['m', s @ ..] => super::t10::n2822(s, out),
		['n', s @ ..] => super::t10::n2823(s, out),
		['o', s @ ..] => super::t10::n2824(s, out),
		['p', s @ ..] => super::t10::n2825(s, out),
		['r', s @ ..] => super::t10::n2826(s, out),
		['s', s @ ..] => super::t10::n2827(s, out),
		['t', s @ ..] => super::t10::n2828(s, out),
		['u', s @ ..] => super::t10::n2829(s, out),
		['v', s @ ..] => super::t10::n2830(s, out),
		['x', s @ ..] => super::t10::n2831(s, out),
		['z', s @ ..] => super::t10::n2832(s, out),
		_ => false,
	}
}

/// Words ending -ks.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2829);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2830);
			match s {
				[] => true,
				['a', 'l', 's', s @ ..] => {
					super::push(out, 9932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', s @ ..] => {
					super::push(out, 9933);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 20524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9934);
					match s {
						[] => true,
						['l', 'd', 'a', 'e', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 20525);
							s.is_empty()
						}
						['s', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 20526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2831);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9935);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 20527);
							match s {
								[] => true,
								['c', 's', 'i', s @ ..] => {
									super::push(out, 30253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 20528);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 30254);
									match s {
										[] => true,
										['t', 'a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 36927);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 20529);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 20530);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 20531);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 36928);
											s.is_empty()
										}
										['t', 't', 'u', 'b', 's', s @ ..] => {
											super::push(out, 36929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 20532);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 30255);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 30256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 20533);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 20534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2832);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2833);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2834);
			match s {
				[] => true,
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9937);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ls.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9938);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 9939);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 20535);
							match s {
								[] => true,
								['ä', s @ ..] => {
									super::push(out, 30257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2836);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9941);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20537);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 20538);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 20539);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30258);
									match s {
										[] => true,
										['h', 'c', 'a', 'r', 'd', '\u{1}', s @ ..] => {
											super::push(out, 36930);
											s.is_empty()
										}
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
					super::push(out, 9942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2837);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2838);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 2839);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 9945);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ms.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2840);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 9946);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9947);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 9948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2841);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9949);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9950);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2842);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2843);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 9951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2844);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9952);
					s.is_empty()
				}
				['-', 'm', 'u', 'i', 'm', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2845);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9954);
					match s {
						[] => true,
						['s', 'g', 'n', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 20541);
							s.is_empty()
						}
						['z', 'n', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 20542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 9955);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ns.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2846);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9956);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 20543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 9957);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9958);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 20545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9959);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 20546);
							match s {
								[] => true,
								['e', 'g', 'n', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 30259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20547);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 30260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 9960);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 20548);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 30261);
									s.is_empty()
								}
								['e', 's', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 30262);
									s.is_empty()
								}
								['h', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 30263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 9961);
					match s {
						[] => true,
						['i', 's', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 20549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9962);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 20550);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 20551);
							match s {
								[] => true,
								['g', 'a', 'o', 'k', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 30264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9963);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 20552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9964);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9965);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20553);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 20554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9966);
					match s {
						[] => true,
						['i', 'r', 'r', 'i', '\u{1}', s @ ..] => {
							super::push(out, 20555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 9967);
					match s {
						[] => true,
						['u', 'j', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 20556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 9968);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2847);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9969);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 20558);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 20559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 9970);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9971);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9972);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20560);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 20561);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 20562);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 20563);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 30265);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 30266);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 36931);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 36932);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9973);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 20565);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 20566);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 20567);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 30267);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'a', s @ ..] => {
							super::push(out, 20568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9974);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9975);
					match s {
						[] => true,
						['o', 'p', 'e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30268);
									s.is_empty()
								}
								['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 30269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9976);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 20570);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 30270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9977);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 20571);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 36933);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 36934);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 20572);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 20573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 9978);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 20574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 9979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2848);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9980);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 20575);
							match s {
								[] => true,
								['a', 'm', 'n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 30271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9981);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 20577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => super::t10::n10103(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2849);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9983);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t10::n2854(s, out),
		['u', s @ ..] => super::t10::n2855(s, out),
		['ö', s @ ..] => super::t10::n2856(s, out),
		_ => false,
	}
}

/// Words ending -os.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2853);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2854);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9988);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2855);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9990);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2856);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9992);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2857);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2858);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9995);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9996);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2859);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9998);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 20584);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 20585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9999);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10000);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20586);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 20587);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 20588);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30277);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 30278);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 20589);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 30279);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 36935);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 30280);
									s.is_empty()
								}
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
			super::push(out, 2860);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 10001);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2861);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10003);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2862);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10005);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10006);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20590);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 20591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10007);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20592);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 20593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'n', 'i', 'w', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 10008);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10009);
					s.is_empty()
				}
				['s', 'e', 'd', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 10010);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10011);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 10012);
					match s {
						[] => true,
						['t', 's', 'o', 'r', 'p', 'i', s @ ..] => {
							super::push(out, 20594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2863);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10013);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10014);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2864);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 10016);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10017);
					s.is_empty()
				}
				['o', 'r', 'h', s @ ..] => {
					super::push(out, 10018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2865);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 10019);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10020);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20595);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 20596);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 20597);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 20598);
							match s {
								[] => true,
								['e', 'k', 'c', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 30281);
									s.is_empty()
								}
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
			super::push(out, 2866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10021);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10022);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 20599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10023);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 20600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2867);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10024);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10025);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 20601);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 20602);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 20603);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 20604);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 30282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 10026);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 10027);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 10028);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10029);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 20605);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 20606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10030);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10031);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 10032);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 20608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2868);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10033);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 10034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2869);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 10035);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10036);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20609);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 20610);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 20611);
							s.is_empty()
						}
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
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2870);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 10037);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 10038);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10039);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10040);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 10041);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10042);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 20614);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30283);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 20615);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 20616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2871);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2872);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10043);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 20617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 10044);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 10045);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20618);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 30284);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 20619);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 20620);
							match s {
								[] => true,
								['o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 30285);
									s.is_empty()
								}
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
			super::push(out, 2873);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2874);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2875);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 10046);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 10047);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 10048);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 10049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2876);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2877);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10050);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10051);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 20622);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 30286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 20623);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 20624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2878);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2879);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 10052);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10053);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10054);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10055);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2880);
			match s {
				[] => true,
				['a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 10056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2881);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10057);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 20626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10058);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 30287);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 30288);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'a', s @ ..] => {
							super::push(out, 20627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2882);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2883);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 2884);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rs.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2885);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2886);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10061);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 20629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 10062);
					match s {
						[] => true,
						['p', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 20630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10063);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 10064);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20631);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 30289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 20632);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 30290);
									s.is_empty()
								}
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
			super::push(out, 2887);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2888);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2889);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 10067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 2890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2891);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 10069);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10070);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 20633);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 20634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ss.
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2892);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 2893);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 10071);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 2894);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10073);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 10074);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 10075);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 20635);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 20636);
							match s {
								[] => true,
								['n', 'i', 'k', 'l', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 30291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20637);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 30292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10076);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 20638);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 30293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 10077);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 10078);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 10079);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10080);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 20641);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 20642);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 20643);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 30294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 20644);
							match s {
								[] => true,
								['c', 'a', 'n', 's', 'i', 'e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 30295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 20645);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 20646);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 36936);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 36937);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 36938);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 30296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 20647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10081);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10082);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10083);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 20648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 10084);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 20649);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 30297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 20650);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 20651);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 20652);
							match s {
								[] => true,
								['n', 'e', 's', s @ ..] => {
									super::push(out, 30298);
									match s {
										[] => true,
										['g', 'n', 'u', 'g', 'r', 'o', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 36939);
											s.is_empty()
										}
										['t', 'ä', 't', 'i', 'd', 'i', 'u', 'q', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 36940);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 20653);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 20654);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 20655);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 30299);
									match s {
										[] => true,
										['k', 'e', s @ ..] => {
											super::push(out, 36941);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 30300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 20656);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 20657);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 36942);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 36943);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 20658);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 30301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 20659);
							match s {
								[] => true,
								['b', 'n', 'e', 'g', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 30302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10085);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 20660);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 20661);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 20662);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 20663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10086);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 20664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 10087);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 20665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 10088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t10::n2899(s, out),
		['i', s @ ..] => super::t10::n2900(s, out),
		['o', s @ ..] => super::t10::n2901(s, out),
		['u', s @ ..] => super::t10::n2902(s, out),
		['y', s @ ..] => {
			super::push(out, 2899);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 10124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 2900);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 2901);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ts.
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 405);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2902);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2903);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10125);
					match s {
						[] => true,
						['i', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30354);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 30355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 10126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2904);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10127);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2905);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2906);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2907);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10130);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 20726);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 20727);
							match s {
								[] => true,
								['c', 'e', s @ ..] => {
									super::push(out, 30356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10131);
					match s {
						[] => true,
						['h', 's', 'r', s @ ..] => {
							super::push(out, 20728);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 20729);
							s.is_empty()
						}
						_ => false,
					}
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

/// Words ending -us.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2909);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10132);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 10133);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 10134);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 10135);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 20730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 10136);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 20731);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 30357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 20732);
							match s {
								[] => true,
								['c', 'o', s @ ..] => {
									super::push(out, 30358);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 36974);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'u', 'f', 's', s @ ..] => {
									super::push(out, 30359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 20733);
							match s {
								[] => true,
								['e', 'k', s @ ..] => {
									super::push(out, 30360);
									match s {
										[] => true,
										['n', 'a', 'r', 'k', 'n', s @ ..] => {
											super::push(out, 36975);
											match s {
												[] => true,
												['e', 'f', s @ ..] => {
													super::push(out, 41003);
													s.is_empty()
												}
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
							super::push(out, 20734);
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									super::push(out, 30361);
									match s {
										[] => true,
										['r', 'ü', 'b', '\u{1}', s @ ..] => {
											super::push(out, 36976);
											s.is_empty()
										}
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
					super::push(out, 10137);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 20735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10138);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20736);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 30362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 20737);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 20738);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 20739);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 20740);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30363);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 30364);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['p', s @ ..] => {
													super::push(out, 41004);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 41005);
													s.is_empty()
												}
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
							super::push(out, 20741);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 20742);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 30365);
									match s {
										[] => true,
										['i', 'n', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 41006);
													s.is_empty()
												}
												['n', 'e', 'd', 'a', 'l', 'o', 'k', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 41007);
													s.is_empty()
												}
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
							super::push(out, 20743);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 30366);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 20744);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 20745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10139);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 20746);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 20747);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 36977);
											match s {
												[] => true,
												['e', 'm', s @ ..] => {
													super::push(out, 41008);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 36978);
											match s {
												[] => true,
												['i', 'h', '\u{1}', s @ ..] => {
													super::push(out, 41009);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 36979);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 20748);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 20749);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 20750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10140);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 20751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10141);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20752);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 20753);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 20754);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 20755);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 20756);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 20757);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 30367);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 30368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => super::t10::n2914(s, out),
		['c', s @ ..] => {
			super::push(out, 2911);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10156);
					match s {
						[] => true,
						['o', 'c', 'o', 'l', 'y', 'h', 'p', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 20778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10157);
					match s {
						[] => true,
						['l', 'l', 'i', s @ ..] => {
							super::push(out, 20779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10158);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => super::t10::n2916(s, out),
		['e', s @ ..] => super::t10::n2917(s, out),
		['f', s @ ..] => super::t10::n2918(s, out),
		['g', s @ ..] => super::t10::n2919(s, out),
		['h', s @ ..] => super::t10::n2920(s, out),
		['i', s @ ..] => super::t10::n2921(s, out),
		['j', s @ ..] => super::t10::n2922(s, out),
		['k', s @ ..] => super::t10::n2923(s, out),
		['l', s @ ..] => super::t10::n2924(s, out),
		['m', s @ ..] => super::t10::n2925(s, out),
		['n', s @ ..] => super::t10::n2926(s, out),
		['o', s @ ..] => super::t10::n2927(s, out),
		['p', s @ ..] => super::t10::n2928(s, out),
		['r', s @ ..] => super::t10::n2929(s, out),
		['s', s @ ..] => super::t10::n2930(s, out),
		['t', s @ ..] => super::t10::n2931(s, out),
		['v', s @ ..] => super::t10::n2932(s, out),
		['x', s @ ..] => super::t10::n2933(s, out),
		['ä', s @ ..] => super::t10::n2934(s, out),
		_ => false,
	}
}

/// Words ending -ws.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 407);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2931);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2932);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ys.
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2933);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10271);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2934);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2935);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2936);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ás.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äs.
pub(crate) fn n410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2938);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 10273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10274);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ås.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	s.is_empty()
}

/// Words ending -ös.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 2940);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2941);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üs.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 2942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -at.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2943);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 10275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20918);
							s.is_empty()
						}
						['r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 20919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10276);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 20920);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 30489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10277);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 20921);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 30490);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 37038);
											match s {
												[] => true,
												['á', 'r', 'g', 'e', 's', 'i', 'v', '\u{1}', s @ ..] => {
													super::push(out, 41107);
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', s @ ..] => {
											super::push(out, 37039);
											s.is_empty()
										}
										['s', 'u', 'g', s @ ..] => {
											super::push(out, 37040);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 41108);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 37041);
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													super::push(out, 41109);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 30491);
									match s {
										[] => true,
										['ü', s @ ..] => {
											super::push(out, 37042);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 30492);
									s.is_empty()
								}
								['f', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 30493);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 30494);
									match s {
										[] => true,
										['a', 'i', 'r', s @ ..] => {
											super::push(out, 37043);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 30495);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 41110);
													match s {
														[] => true,
														['u', s @ ..] => super::t10::n46191(s, out),
														_ => false,
													}
												}
												['h', s @ ..] => {
													super::push(out, 41111);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 41112);
													match s {
														[] => true,
														['e', 't', 'r', 'a', 'p', '\u{1}', s @ ..] => super::t10::n46192(s, out),
														_ => false,
													}
												}
												['n', 'n', s @ ..] => {
													super::push(out, 41113);
													match s {
														[] => true,
														['i', 'b', '\u{1}', s @ ..] => super::t10::n46193(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 41114);
													match s {
														[] => true,
														['i', s @ ..] => super::t10::n46194(s, out),
														['t', s @ ..] => super::t10::n46195(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 37044);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 41115);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 30496);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 30497);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 37045);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 41116);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 41117);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 37046);
											s.is_empty()
										}
										['t', 'h', 'c', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 37047);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2944);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10278);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 20922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10279);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 20923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10280);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 20924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10281);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 20925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10282);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 20926);
							match s {
								[] => true,
								['n', 'm', s @ ..] => {
									super::push(out, 30498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 20927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t10::n2949(s, out),
		['d', s @ ..] => super::t10::n2950(s, out),
		['e', s @ ..] => super::t10::n2951(s, out),
		['f', s @ ..] => super::t10::n2952(s, out),
		['g', s @ ..] => super::t10::n2953(s, out),
		['h', s @ ..] => super::t10::n2954(s, out),
		['i', s @ ..] => super::t10::n2955(s, out),
		['k', s @ ..] => super::t10::n2956(s, out),
		['l', s @ ..] => super::t10::n2957(s, out),
		['m', s @ ..] => super::t10::n2958(s, out),
		['n', s @ ..] => super::t10::n2959(s, out),
		['o', s @ ..] => super::t10::n2960(s, out),
		['p', s @ ..] => super::t11::n2961(s, out),
		['q', s @ ..] => {
			super::push(out, 2958);
			s.is_empty()
		}
		['r', s @ ..] => super::t11::n2963(s, out),
		['s', s @ ..] => super::t11::n2964(s, out),
		['t', s @ ..] => super::t11::n2965(s, out),
		['u', s @ ..] => super::t11::n2966(s, out),
		['v', s @ ..] => super::t11::n2967(s, out),
		['w', s @ ..] => {
			super::push(out, 2964);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10407);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -feestadt.
pub(crate) fn n43794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41183);
	s.is_empty()
}

/// The word idealstadt.
pub(crate) fn n43795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41184);
	s.is_empty()
}

/// Words ending -kanstadt.
pub(crate) fn n43796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41185);
	s.is_empty()
}

/// Words ending -et.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 419);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2972);
			match s {
				[] => true,
				['t', 'i', 'l', 'a', 'n', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 10417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2973);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10418);
					match s {
						[] => true,
						['h', 'p', 'l', 'a', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 30679);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 37198);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 37199);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 30680);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 37200);
											s.is_empty()
										}
										['e', 'n', 'u', 'r', '\u{1}', s @ ..] => {
											super::push(out, 37201);
											s.is_empty()
										}
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
					super::push(out, 10419);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 21159);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 21160);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 30682);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 37202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 21161);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10420);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 21162);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 21163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10421);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 21164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2974);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10422);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10423);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 10424);
					match s {
						[] => true,
						['m', 'o', 'c', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 21165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2975);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2976);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10425);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 30684);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 30685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 10426);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 21166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10427);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10428);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 21167);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 21168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 10429);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 21169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2977);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10430);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 10431);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 21170);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 21171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10432);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 21172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2978);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10433);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10434);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 10435);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10436);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 21173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10437);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 21174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10438);
					match s {
						[] => true,
						['ö', s @ ..] => {
							super::push(out, 21175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2979);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10439);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21176);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 30686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 21177);
							match s {
								[] => true,
								['t', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 30687);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 21178);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 21179);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 30688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'b', 'e', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 21180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 10440);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10441);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 21181);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 21182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2980);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10442);
					match s {
						[] => true,
						['e', s @ ..] => super::t11::n21795(s, out),
						['l', s @ ..] => {
							super::push(out, 21184);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30690);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 10443);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 21185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10444);
					s.is_empty()
				}
				['r', s @ ..] => super::t11::n10568(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t11::n2986(s, out),
		['k', s @ ..] => super::t11::n2987(s, out),
		['l', s @ ..] => super::t11::n2988(s, out),
		['m', s @ ..] => super::t11::n2989(s, out),
		['n', s @ ..] => super::t11::n2990(s, out),
		['o', s @ ..] => {
			super::push(out, 2986);
			match s {
				[] => true,
				['p', s @ ..] => super::t11::n10607(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t11::n2992(s, out),
		['r', s @ ..] => super::t11::n2993(s, out),
		['s', s @ ..] => super::t11::n2994(s, out),
		['t', s @ ..] => super::t11::n2995(s, out),
		['u', s @ ..] => super::t11::n2996(s, out),
		['v', s @ ..] => super::t11::n2997(s, out),
		['w', s @ ..] => super::t11::n2998(s, out),
		['z', s @ ..] => super::t11::n2999(s, out),
		_ => false,
	}
}

/// Words ending -ft.
pub(crate) fn n420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 420);
	match s {
		[] => true,
		['a', s @ ..] => super::t11::n3000(s, out),
		['e', s @ ..] => {
			super::push(out, 2996);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 10510);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 21266);
							match s {
								[] => true,
								['c', 'i', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 30743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 21267);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 21268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10511);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 21269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2997);
			match s {
				[] => true,
				['a', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 10512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2998);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 10513);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 21270);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 37240);
											s.is_empty()
										}
										['n', 'n', s @ ..] => {
											super::push(out, 37241);
											s.is_empty()
										}
										['s', 'i', s @ ..] => {
											super::push(out, 37242);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 37243);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 21271);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 30744);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 21272);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 30745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21273);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 21274);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 21275);
							s.is_empty()
						}
						['i', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 21276);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 21277);
							s.is_empty()
						}
						['m', 'u', 'a', 'r', 't', 'l', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 21278);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 21279);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['n', 'o', 's', 'r', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 37244);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 37245);
											s.is_empty()
										}
										['t', 'r', 'e', 'd', 'n', 'i', 'h', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 37246);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'p', 'e', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 21280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10515);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 21281);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 30746);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 30747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 21282);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => super::t11::n39377(s, out),
										['i', s @ ..] => super::t11::n39378(s, out),
										['l', 'a', 's', s @ ..] => super::t11::n39379(s, out),
										['m', 'a', s @ ..] => super::t11::n39380(s, out),
										['n', s @ ..] => super::t11::n39381(s, out),
										['r', s @ ..] => super::t11::n39382(s, out),
										['s', 'g', 'n', s @ ..] => super::t11::n39383(s, out),
										['t', s @ ..] => super::t11::n39384(s, out),
										['u', 'a', s @ ..] => super::t11::n39385(s, out),
										['z', s @ ..] => super::t11::n39386(s, out),
										['ß', s @ ..] => super::t11::n39387(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t11::n10639(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t11::n3004(s, out),
		['n', s @ ..] => super::t11::n3005(s, out),
		['o', s @ ..] => super::t11::n3006(s, out),
		['r', s @ ..] => super::t11::n3007(s, out),
		['u', s @ ..] => super::t11::n3008(s, out),
		['ä', s @ ..] => super::t11::n3009(s, out),
		['ö', s @ ..] => super::t11::n3010(s, out),
		_ => false,
	}
}

/// Words ending -ht.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 422);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3008);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10534);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10535);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 30779);
									match s {
										[] => true,
										['o', 't', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 37270);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'l', 'o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 30780);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 30781);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 30782);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 30783);
									match s {
										[] => true,
										['t', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 37271);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 30784);
									match s {
										[] => true,
										['e', 'i', 'v', 'a', 'l', 'k', '\u{1}', s @ ..] => {
											super::push(out, 37272);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ß', 'i', 'e', 'w', 'h', 'c', 's', 'f', s @ ..] => {
									super::push(out, 30785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t11::n3014(s, out),
		['g', s @ ..] => {
			super::push(out, 3010);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10545);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 21359);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 21360);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 21361);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 21362);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 21363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -it.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3011);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21364);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 21365);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 21366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10547);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 10548);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 21367);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 21368);
							match s {
								[] => true,
								['k', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 30888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10549);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 21369);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 21370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3012);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10550);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 30889);
									match s {
										[] => true,
										['a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 37402);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 30890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 10551);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10552);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 21371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10553);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 10554);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10555);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21373);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 21374);
							match s {
								[] => true,
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 30891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10556);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10557);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 21375);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 21376);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 21377);
							match s {
								[] => true,
								['i', 'k', s @ ..] => {
									super::push(out, 30892);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37403);
											s.is_empty()
										}
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
					super::push(out, 10558);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 21378);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 21379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3013);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10560);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 10561);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10562);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 21380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3014);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10563);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 10564);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 21381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 10565);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 10566);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 21382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10567);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 21383);
							match s {
								[] => true,
								['x', s @ ..] => {
									super::push(out, 30893);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 21384);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 37404);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 37405);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 37406);
											s.is_empty()
										}
										['n', 'e', 't', 'a', s @ ..] => {
											super::push(out, 37407);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['g', 'n', 'u', 'r', s @ ..] => {
													super::push(out, 41351);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 41352);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10568);
					match s {
						[] => true,
						['a', 'w', 'n', 'n', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 21385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10569);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21386);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 30894);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 30895);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37408);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 30896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 21387);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 21388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30897);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10570);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 30898);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 30899);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 30900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10571);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 21389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10572);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 10573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t11::n3020(s, out),
		['f', s @ ..] => super::t11::n3021(s, out),
		['g', s @ ..] => super::t11::n3022(s, out),
		['h', s @ ..] => super::t11::n3023(s, out),
		['i', s @ ..] => super::t11::n3024(s, out),
		['k', s @ ..] => super::t11::n3025(s, out),
		['l', s @ ..] => super::t11::n3026(s, out),
		['m', s @ ..] => super::t11::n3027(s, out),
		['n', s @ ..] => super::t11::n3028(s, out),
		['o', s @ ..] => super::t11::n3029(s, out),
		['p', s @ ..] => super::t11::n3030(s, out),
		['r', s @ ..] => super::t11::n3031(s, out),
		['s', s @ ..] => super::t11::n3032(s, out),
		['t', s @ ..] => super::t11::n3033(s, out),
		['u', s @ ..] => super::t11::n3034(s, out),
		['v', s @ ..] => super::t11::n3035(s, out),
		['w', s @ ..] => super::t11::n3036(s, out),
		['x', s @ ..] => super::t11::n3037(s, out),
		['y', s @ ..] => super::t11::n3038(s, out),
		['z', s @ ..] => super::t11::n3039(s, out),
		_ => false,
	}
}

/// Words ending -kt.
pub(crate) fn n424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3035);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10710);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 21597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10711);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 10712);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21598);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 21599);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 31103);
									match s {
										[] => true,
										['t', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 37582);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 31104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10713);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10714);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 31105);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 31106);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 31107);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 10715);
					match s {
						[] => true,
						['s', 'g', s @ ..] => {
							super::push(out, 21600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10716);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21601);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 21602);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 31108);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 31109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 21603);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 31110);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 21604);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 31111);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 31112);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['e', 'l', s @ ..] => {
													super::push(out, 41531);
													s.is_empty()
												}
												['n', 'e', 'z', s @ ..] => {
													super::push(out, 41532);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10717);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 21605);
							match s {
								[] => true,
								['b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 31113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10718);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 21606);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 21607);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 21608);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 31114);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 31115);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 21609);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 31116);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 37583);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 37584);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 31117);
									match s {
										[] => true,
										['k', 't', 'e', s @ ..] => {
											super::push(out, 37585);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 21610);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 31118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 10719);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 21611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t11::n3041(s, out),
		['i', s @ ..] => {
			super::push(out, 3037);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10726);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 21628);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 31133);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 31134);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 37610);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 21629);
							match s {
								[] => true,
								['n', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 37611);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 37612);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 37613);
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 41566);
													s.is_empty()
												}
												['e', 'c', s @ ..] => {
													super::push(out, 41567);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 37614);
											match s {
												[] => true,
												['l', s @ ..] => super::t11::n44238(s, out),
												_ => false,
											}
										}
										['t', s @ ..] => super::t11::n39823(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t11::n10852(s, out),
				['v', s @ ..] => super::t11::n10853(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t11::n3043(s, out),
		['o', s @ ..] => super::t11::n3044(s, out),
		['r', s @ ..] => super::t11::n3045(s, out),
		['u', s @ ..] => super::t11::n3046(s, out),
		_ => false,
	}
}

/// Words ending -lt.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 425);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3042);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10731);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 21641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10732);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 21642);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 21643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 10733);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 21644);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 31161);
									s.is_empty()
								}
								['g', s @ ..] => super::t11::n32637(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => super::t11::n22289(s, out),
						['p', s @ ..] => {
							super::push(out, 21646);
							match s {
								[] => true,
								['s', 'a', '\u{1}', s @ ..] => {
									super::push(out, 31163);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t11::n22291(s, out),
						['s', s @ ..] => {
							super::push(out, 21648);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['a', 'h', 's', 'g', 'a', s @ ..] => super::t11::n39885(s, out),
										['b', s @ ..] => super::t11::n39886(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t11::n22293(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t11::n10861(s, out),
				['n', s @ ..] => super::t11::n10862(s, out),
				['p', s @ ..] => super::t11::n10863(s, out),
				['s', s @ ..] => super::t11::n10864(s, out),
				['t', s @ ..] => super::t11::n10865(s, out),
				['w', s @ ..] => super::t11::n10866(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t11::n3048(s, out),
		['i', s @ ..] => super::t11::n3049(s, out),
		['o', s @ ..] => super::t11::n3050(s, out),
		['u', s @ ..] => super::t11::n3051(s, out),
		['ü', s @ ..] => super::t11::n3052(s, out),
		_ => false,
	}
}

/// Words ending -leramt.
pub(crate) fn n10886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10759);
	s.is_empty()
}

/// Words ending -samt.
pub(crate) fn n10887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10760);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21690);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 21691);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 31180);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 41627);
									s.is_empty()
								}
								['u', 'b', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 43639);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 43640);
											s.is_empty()
										}
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
					super::push(out, 31181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 21692);
			match s {
				[] => true,
				['n', 'u', 'r', 'e', s @ ..] => {
					super::push(out, 31182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 21693);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 21694);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 21695);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 31183);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tamt.
pub(crate) fn n10888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10761);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 21696);
			match s {
				[] => true,
				['u', 'a', 'h', 's', s @ ..] => {
					super::push(out, 31184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uamt.
pub(crate) fn n10889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10762);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 21697);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zamt.
pub(crate) fn n10890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10763);
	s.is_empty()
}

/// Words ending -dmt.
pub(crate) fn n3054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3049);
	s.is_empty()
}

/// Words ending -imt.
pub(crate) fn n3055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3050);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 10764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mmt.
pub(crate) fn n3056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3051);
	s.is_empty()
}

/// Words ending -umt.
pub(crate) fn n3057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3052);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 10765);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nt.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3053);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10766);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 10767);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 31185);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 31186);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 37681);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 37682);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 37683);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 31187);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 37684);
											s.is_empty()
										}
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
					super::push(out, 10768);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 10769);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21698);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 21699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 10770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21700);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 21701);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 31188);
									match s {
										[] => true,
										['t', '\u{1}', s @ ..] => {
											super::push(out, 37685);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 21702);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 31189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 21703);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 21704);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 31190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 10771);
					match s {
						[] => true,
						['c', 'n', s @ ..] => {
							super::push(out, 21705);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 31191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10772);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 21706);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 21707);
							match s {
								[] => true,
								['i', 'n', 'g', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 31192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 21708);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 21709);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 31193);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 37686);
											s.is_empty()
										}
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
					super::push(out, 10773);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 21710);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 31194);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 37687);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 21711);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 31195);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 21712);
							match s {
								[] => true,
								['e', 't', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 31196);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 21713);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 31197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 21714);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 31198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10774);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 21716);
							s.is_empty()
						}
						['o', s @ ..] => super::t11::n22363(s, out),
						['u', s @ ..] => {
							super::push(out, 21718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t11::n10902(s, out),
				['n', s @ ..] => super::t11::n10903(s, out),
				['p', s @ ..] => super::t11::n10904(s, out),
				['r', s @ ..] => super::t11::n10905(s, out),
				['s', s @ ..] => super::t11::n10906(s, out),
				['t', s @ ..] => super::t11::n10907(s, out),
				['u', s @ ..] => super::t11::n10908(s, out),
				['v', s @ ..] => super::t11::n10909(s, out),
				['w', s @ ..] => super::t11::n10910(s, out),
				['y', s @ ..] => super::t11::n10911(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t11::n3059(s, out),
		['h', s @ ..] => super::t11::n3060(s, out),
		['i', s @ ..] => super::t11::n3061(s, out),
		['o', s @ ..] => super::t11::n3062(s, out),
		['u', s @ ..] => super::t11::n3063(s, out),
		_ => false,
	}
}

/// Words ending -ot.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3059);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10817);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 10818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3060);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10819);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 10820);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 21826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['a', s @ ..] => {
					super::push(out, 10821);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 21827);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 21828);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10822);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 31306);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 37760);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 41681);
													s.is_empty()
												}
												['e', 'm', s @ ..] => {
													super::push(out, 41682);
													s.is_empty()
												}
												['h', 'c', 's', 'i', s @ ..] => {
													super::push(out, 41683);
													s.is_empty()
												}
												['k', s @ ..] => {
													super::push(out, 41684);
													s.is_empty()
												}
												['l', 'e', 'g', s @ ..] => {
													super::push(out, 41685);
													s.is_empty()
												}
												['n', 'e', 'l', 'l', 'e', 't', 's', '\u{1}', s @ ..] => {
													super::push(out, 41686);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 37761);
											match s {
												[] => true,
												['h', 'c', 'r', 'i', 'k', '\u{1}', s @ ..] => {
													super::push(out, 41687);
													s.is_empty()
												}
												['k', s @ ..] => {
													super::push(out, 41688);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 31307);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 31308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10823);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 21829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10824);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 21830);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 21831);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31309);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 21832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 21833);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 21834);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10825);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 21835);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 31312);
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 37762);
											s.is_empty()
										}
										['r', 'e', 'h', s @ ..] => {
											super::push(out, 37763);
											s.is_empty()
										}
										['s', 't', s @ ..] => {
											super::push(out, 37764);
											s.is_empty()
										}
										['t', 't', s @ ..] => {
											super::push(out, 37765);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 37766);
											s.is_empty()
										}
										['z', 't', s @ ..] => {
											super::push(out, 37767);
											s.is_empty()
										}
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
					super::push(out, 10826);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3061);
			match s {
				[] => true,
				['r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 10827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3062);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10828);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 21837);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 21838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10829);
					match s {
						[] => true,
						['a', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 21839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3063);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 10830);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10831);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 10832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3064);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10833);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 21840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10834);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3065);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10836);
					s.is_empty()
				}
				['r', 'g', s @ ..] => {
					super::push(out, 10837);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 3066);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3067);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t11::n10967(s, out),
				['e', s @ ..] => super::t11::n10968(s, out),
				['i', s @ ..] => super::t11::n10969(s, out),
				['k', s @ ..] => super::t11::n10970(s, out),
				['l', s @ ..] => super::t11::n10971(s, out),
				['n', s @ ..] => super::t11::n10972(s, out),
				['r', s @ ..] => super::t11::n10973(s, out),
				['s', s @ ..] => super::t11::n10974(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t11::n3073(s, out),
		['m', s @ ..] => super::t11::n3074(s, out),
		['n', s @ ..] => super::t11::n3075(s, out),
		['o', s @ ..] => super::t11::n3076(s, out),
		['p', s @ ..] => super::t11::n3077(s, out),
		['r', s @ ..] => super::t11::n3078(s, out),
		['s', s @ ..] => super::t11::n3079(s, out),
		['t', s @ ..] => super::t11::n3080(s, out),
		['u', s @ ..] => super::t11::n3081(s, out),
		['v', s @ ..] => super::t11::n3082(s, out),
		['x', s @ ..] => super::t11::n3083(s, out),
		['y', s @ ..] => super::t11::n3084(s, out),
		_ => false,
	}
}

/// Words ending -pt.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3080);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10887);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10888);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 10889);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 21900);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 21901);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 31350);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 21902);
							match s {
								[] => true,
								['o', 'k', 'n', s @ ..] => {
									super::push(out, 31351);
									match s {
										[] => true,
										['e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 37791);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3081);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 21903);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 31352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 21904);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37792);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 37793);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 37794);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3082);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 10890);
					s.is_empty()
				}
				['u', 's', s @ ..] => {
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 21905);
							s.is_empty()
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 21906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3083);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10891);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 31353);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 31354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10892);
					s.is_empty()
				}
				['ä', 'h', '\u{1}', s @ ..] => {
					super::push(out, 10893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rt.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 430);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3084);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 10894);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 21907);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 21908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 10895);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21909);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 21910);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 31355);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37795);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 21911);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 21912);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 21913);
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									super::push(out, 31356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 21914);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 31357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'r', s @ ..] => {
							super::push(out, 21915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 10896);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21916);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 21917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10897);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 10898);
					match s {
						[] => true,
						['f', 'o', 'h', s @ ..] => {
							super::push(out, 21918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 10899);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 10900);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21919);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 21920);
							match s {
								[] => true,
								['l', 'i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 31358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 21921);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31359);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 31360);
									match s {
										[] => true,
										['i', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 37796);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 31361);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 37797);
											match s {
												[] => true,
												['c', 'i', 'e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 41702);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', 'i', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 21922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10901);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10902);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10903);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21923);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 31362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 21924);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 31363);
									match s {
										[] => true,
										['a', 'r', 'f', 's', s @ ..] => {
											super::push(out, 37798);
											s.is_empty()
										}
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
					super::push(out, 10904);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 21925);
							match s {
								[] => true,
								['e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 31364);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 31365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 21926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 10905);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 21927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10906);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 31366);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 31367);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 31368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10907);
					match s {
						[] => true,
						['g', 'n', 'u', 'z', s @ ..] => {
							super::push(out, 21928);
							match s {
								[] => true,
								['t', 'u', s @ ..] => {
									super::push(out, 31369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t11::n11038(s, out),
				['u', s @ ..] => {
					super::push(out, 10909);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 21935);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 21936);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 21937);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 31382);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => super::t11::n11040(s, out),
				['z', s @ ..] => {
					super::push(out, 10911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t11::n3090(s, out),
		['h', s @ ..] => super::t11::n3091(s, out),
		['i', s @ ..] => super::t11::n3092(s, out),
		['o', s @ ..] => super::t11::n3093(s, out),
		['u', s @ ..] => super::t11::n3094(s, out),
		_ => false,
	}
}

/// Words ending -st.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 431);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3090);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10956);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 10957);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 10958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 10959);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 22036);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 22037);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 31483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 10960);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 22038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10961);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 22039);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 22040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 10962);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 22041);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 31484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', s @ ..] => {
							super::push(out, 22042);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 31485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 22043);
							match s {
								[] => true,
								['m', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 31486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 31487);
									s.is_empty()
								}
								['u', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 31488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'd', s @ ..] => {
							super::push(out, 22044);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 31489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 31490);
									match s {
										[] => true,
										['c', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 37901);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 31491);
									match s {
										[] => true,
										['a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 37902);
											s.is_empty()
										}
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
					super::push(out, 10963);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 22045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10964);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 22046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10965);
					match s {
						[] => true,
						['a', s @ ..] => super::t11::n22706(s, out),
						['b', s @ ..] => super::t11::n22707(s, out),
						['d', s @ ..] => {
							super::push(out, 22049);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 22050);
							match s {
								[] => true,
								['p', s @ ..] => super::t11::n33007(s, out),
								_ => false,
							}
						}
						['g', s @ ..] => super::t11::n22710(s, out),
						['k', s @ ..] => super::t11::n22711(s, out),
						['l', s @ ..] => super::t11::n22712(s, out),
						['n', s @ ..] => super::t11::n22713(s, out),
						['p', s @ ..] => super::t11::n22714(s, out),
						['r', s @ ..] => super::t11::n22715(s, out),
						['s', s @ ..] => super::t11::n22716(s, out),
						['t', s @ ..] => super::t11::n22717(s, out),
						['z', s @ ..] => super::t11::n22718(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t11::n11096(s, out),
				['n', s @ ..] => super::t11::n11097(s, out),
				['o', s @ ..] => super::t11::n11098(s, out),
				['p', s @ ..] => super::t11::n11099(s, out),
				['r', s @ ..] => super::t11::n11100(s, out),
				['s', s @ ..] => super::t11::n11101(s, out),
				['t', s @ ..] => super::t11::n11102(s, out),
				['u', s @ ..] => super::t11::n11103(s, out),
				['z', s @ ..] => super::t11::n11104(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t11::n3096(s, out),
		['e', s @ ..] => super::t11::n3097(s, out),
		['g', s @ ..] => super::t11::n3098(s, out),
		['h', s @ ..] => super::t11::n3099(s, out),
		['i', s @ ..] => super::t11::n3100(s, out),
		['l', s @ ..] => super::t11::n3101(s, out),
		['n', s @ ..] => super::t11::n3102(s, out),
		['o', s @ ..] => super::t11::n3103(s, out),
		['p', s @ ..] => super::t11::n3104(s, out),
		['r', s @ ..] => super::t11::n3105(s, out),
		['s', s @ ..] => super::t11::n3106(s, out),
		['u', s @ ..] => super::t11::n3107(s, out),
		['y', s @ ..] => super::t11::n3108(s, out),
		['ä', s @ ..] => super::t11::n3109(s, out),
		['ü', s @ ..] => super::t11::n3110(s, out),
		_ => false,
	}
}

