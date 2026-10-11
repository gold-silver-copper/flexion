use flexion::Path;

/// Words ending -andeln.
pub(crate) fn n2600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2211);
	match s {
		[] => true,
		['b', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4453);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4454);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6193);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6194);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6195);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7685);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 7686);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['a', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', 'e', 'l', 'g', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7687);
									match s {
										[] => true,
										['o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6196);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6197);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', 'n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 7688);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7802);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 7803);
											match s {
												[] => true,
												['c', 'a', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6198);
					match s {
						[] => true,
						['s', 'i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'm', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7689);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7292);
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
				['ß', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4455);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6199);
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6200);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 6201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6202);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6203);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6204);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6205);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6206);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udeln.
pub(crate) fn n930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 870);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2217);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4459);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2218);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2219);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4460);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 't', 'n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2220);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2221);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4462);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4463);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6210);
							s.is_empty()
						}
						['s', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2222);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4464);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ädeln.
pub(crate) fn n931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 871);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4465);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4466);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4467);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4468);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4469);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ödeln.
pub(crate) fn n932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 872);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tüdeln.
pub(crate) fn n933(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2223);
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 2224);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word wiederspiegeln.
pub(crate) fn n14227(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word widerspiegeln.
pub(crate) fn n14228(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ugeln.
pub(crate) fn n944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 883);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 2235);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 6222);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ögeln.
pub(crate) fn n946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 885);
	match s {
		[] => true,
		['v', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4491);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ügeln.
pub(crate) fn n947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 886);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4492);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4493);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4494);
					match s {
						[] => true,
						['t', 'a', 'l', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2238);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 4495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6224);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6225);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2239);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6226);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 7690);
											match s {
												[] => true,
												['e', 'i', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 7691);
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
							super::push(out, 6227);
							match s {
								[] => true,
								['o', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6228);
							match s {
								[] => true,
								['o', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2240);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word umschmeicheln.
pub(crate) fn n14231(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word einschmeicheln.
pub(crate) fn n14232(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word weiterentwickeln.
pub(crate) fn n14233(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rdunkeln.
pub(crate) fn n9445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6248);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -funkeln.
pub(crate) fn n5933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4517);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6249);
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

/// Words ending -rkeln.
pub(crate) fn n953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 892);
	s.is_empty()
}

/// Words ending -ukeln.
pub(crate) fn n954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 893);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6251);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äkeln.
pub(crate) fn n955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 894);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4519);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -erkrümeln.
pub(crate) fn n9480(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word verkuppeln.
pub(crate) fn n9502(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word entkuppeln.
pub(crate) fn n9503(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -äppeln.
pub(crate) fn n2687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2271);
	match s {
		[] => true,
		['p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6277);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 4537);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rüppeln.
pub(crate) fn n2688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2272);
	s.is_empty()
}

/// Words ending -speln.
pub(crate) fn n964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 901);
	match s {
		[] => true,
		['a', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4538);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word busseln.
pub(crate) fn n5988(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word abbusseln.
pub(crate) fn n5989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4552);
	s.is_empty()
}

/// Words ending -üsseln.
pub(crate) fn n2700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2281);
	match s {
		[] => true,
		['l', 'h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6288);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6289);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rätseln.
pub(crate) fn n971(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2282);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -useln.
pub(crate) fn n972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 908);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4554);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öseln.
pub(crate) fn n973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 909);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6290);
							match s {
								[] => true,
								['e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4556);
					match s {
						[] => true,
						['f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ötteln.
pub(crate) fn n2718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2291);
	match s {
		[] => true,
		['p', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6301);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ütteln.
pub(crate) fn n2719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2292);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 4564);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7325);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 7326);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 7327);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7328);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4565);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6302);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6303);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 6304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uteln.
pub(crate) fn n981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 916);
	match s {
		[] => true,
		['e', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4566);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uwandern.
pub(crate) fn n9612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6329);
	match s {
		[] => true,
		['z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -endern.
pub(crate) fn n2748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2311);
	match s {
		[] => true,
		['g', 's', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4588);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4589);
			s.is_empty()
		}
		['t', 'e', 'k', 'r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4590);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -indern.
pub(crate) fn n2749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2312);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6330);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 6331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4591);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 6332);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ondern.
pub(crate) fn n2750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2313);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6333);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6334);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -undern.
pub(crate) fn n2751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2314);
	match s {
		[] => true,
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6335);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ändern.
pub(crate) fn n2752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2315);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4592);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4593);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4594);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4595);
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

/// Words ending -ündern.
pub(crate) fn n2753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2316);
	s.is_empty()
}

/// Words ending -odern.
pub(crate) fn n1003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 936);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 2317);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4596);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
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
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4597);
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
		_ => false,
	}
}

/// Words ending -rdern.
pub(crate) fn n1004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 937);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2318);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4598);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4599);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6336);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6337);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 6338);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 6339);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6340);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6341);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7336);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 7337);
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
						['s', s @ ..] => {
							super::push(out, 6342);
							match s {
								[] => true,
								['u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 2319);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6343);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6344);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 7703);
											match s {
												[] => true,
												['e', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udern.
pub(crate) fn n1005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 938);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2320);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6345);
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
				['l', s @ ..] => {
					super::push(out, 4600);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7338);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2321);
			match s {
				[] => true,
				['l', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6346);
							s.is_empty()
						}
						['h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6347);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7339);
									match s {
										[] => true,
										['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7340);
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6348);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 7704);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 7705);
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
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', 'c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2322);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4601);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ädern.
pub(crate) fn n1006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 939);
	s.is_empty()
}

/// Words ending -ödern.
pub(crate) fn n1007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 940);
	s.is_empty()
}

/// Words ending -üdern.
pub(crate) fn n1008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 941);
	s.is_empty()
}

/// The word versteigern.
pub(crate) fn n14237(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word zwangsversteigern.
pub(crate) fn n14238(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rgern.
pub(crate) fn n1021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	match s {
		[] => true,
		['ä', s @ ..] => {
			super::push(out, 2339);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4621);
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
		['ü', s @ ..] => {
			super::push(out, 2340);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6367);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ögern.
pub(crate) fn n1023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 956);
	match s {
		[] => true,
		['z', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4622);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4623);
					match s {
						[] => true,
						['u', 'a', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word aufspeichern.
pub(crate) fn n14239(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word zwischenspeichern.
pub(crate) fn n14240(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ausspeichern.
pub(crate) fn n14241(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ästern.
pub(crate) fn n2852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2393);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6430);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üstern.
pub(crate) fn n2853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2394);
	match s {
		[] => true,
		['l', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6431);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6432);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ttern.
pub(crate) fn n1061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 988);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2395);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4677);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2396);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4679);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7366);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7367);
									match s {
										[] => true,
										['a', 'n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7368);
									match s {
										[] => true,
										['e', 't', 'n', 'u', 'n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 4680);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7369);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7370);
									match s {
										[] => true,
										['e', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 4681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2397);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4682);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 7371);
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
					super::push(out, 4683);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6433);
							s.is_empty()
						}
						['p', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7372);
									match s {
										[] => true,
										['e', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6434);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6435);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6436);
							s.is_empty()
						}
						['e', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 6437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 4684);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6438);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6439);
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
		['o', s @ ..] => {
			super::push(out, 2398);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6440);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2399);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6442);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 6443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 2400);
			match s {
				[] => true,
				['l', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6445);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6446);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6447);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6448);
							match s {
								[] => true,
								['i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 2401);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 2402);
			match s {
				[] => true,
				['f', s @ ..] => super::t5::n6228(s, out),
				['h', s @ ..] => {
					super::push(out, 4688);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utern.
pub(crate) fn n1062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 989);
	match s {
		[] => true,
		['ä', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4689);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4690);
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

/// The word untersteuern.
pub(crate) fn n13839(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word versteuern.
pub(crate) fn n13840(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ssteuern.
pub(crate) fn n9853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6464);
	match s {
		[] => true,
		['u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 7377);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äuern.
pub(crate) fn n1065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 992);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4705);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4706);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4707);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -treiben.
pub(crate) fn n2986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2466);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4769);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4770);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4771);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4772);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4773);
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4774);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4775);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6490);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6491);
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
		['r', s @ ..] => {
			super::push(out, 4776);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6492);
					match s {
						[] => true,
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', s @ ..] => {
							match s {
								[] => true,
								['i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4777);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4778);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mtragen.
pub(crate) fn n3391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2757);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntragen.
pub(crate) fn n3392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2758);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4976);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6590);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4977);
			match s {
				[] => true,
				['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4978);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6591);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6592);
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
		['o', 'v', 'a', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtragen.
pub(crate) fn n3393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2759);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4979);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6593);
					s.is_empty()
				}
				['b', 'ü', '\u{1}', s @ ..] => {
					super::push(out, 6594);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6595);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6596);
					match s {
						[] => true,
						['n', 'u', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 6597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4980);
			match s {
				[] => true,
				['p', 'm', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stragen.
pub(crate) fn n3394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2760);
	match s {
		[] => true,
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ttragen.
pub(crate) fn n3395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2761);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4981);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utragen.
pub(crate) fn n3396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2762);
	match s {
		[] => true,
		['z', s @ ..] => {
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

/// Words ending -zeigen.
pub(crate) fn n1434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1288);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2831);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2832);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2833);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2834);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2835);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5044);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 6630);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5045);
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
		_ => false,
	}
}

/// Words ending -ringen.
pub(crate) fn n1466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1311);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2882);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2883);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5083);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 5084);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 5085);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6657);
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
					super::push(out, 5086);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5087);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6658);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 't', 'r', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5088);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6659);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5089);
					match s {
						[] => true,
						['l', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5090);
					match s {
						[] => true,
						['i', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6660);
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
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5091);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6661);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6662);
							match s {
								[] => true,
								['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6663);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5092);
					match s {
						[] => true,
						['a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6664);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7435);
									s.is_empty()
								}
								['b', 'ü', '\u{1}', s @ ..] => {
									super::push(out, 7436);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 7437);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ä', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 7438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6665);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5093);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5094);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6666);
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
				['u', s @ ..] => {
					super::push(out, 5095);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 6667);
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
				['y', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2884);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5096);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5097);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5098);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5099);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5100);
					match s {
						[] => true,
						['o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5101);
					match s {
						[] => true,
						['u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 5102);
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
					super::push(out, 5103);
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
		['h', s @ ..] => super::t5::n3579(s, out),
		['m', s @ ..] => super::t5::n3580(s, out),
		['p', s @ ..] => super::t5::n3581(s, out),
		['r', s @ ..] => super::t5::n3582(s, out),
		['w', s @ ..] => super::t5::n3583(s, out),
		_ => false,
	}
}

/// The word hochkriechen.
pub(crate) fn n14258(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word durchkriechen.
pub(crate) fn n14259(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -anbrechen.
pub(crate) fn n10500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6706);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'h', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inbrechen.
pub(crate) fn n10502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6707);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rbrechen.
pub(crate) fn n7179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5169);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7447);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7448);
					match s {
						[] => true,
						['e', 'i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7731);
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 7449);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 7450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6709);
			match s {
				[] => true,
				['v', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sbrechen.
pub(crate) fn n7180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5170);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 6710);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6711);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -prechen.
pub(crate) fn n3698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2972);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6712);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6713);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6714);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6715);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6716);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6717);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6718);
					match s {
						[] => true,
						['e', 'r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6719);
					match s {
						[] => true,
						['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6720);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6721);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7451);
							match s {
								[] => true,
								['d', 'i', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7452);
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
				['s', s @ ..] => {
					super::push(out, 6722);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6723);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6724);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -techen.
pub(crate) fn n1527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1363);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 2973);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5171);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5172);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5173);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5174);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6726);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6727);
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
				['r', s @ ..] => {
					super::push(out, 5175);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7453);
									s.is_empty()
								}
								['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7454);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 7455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'v', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5176);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5177);
					match s {
						[] => true,
						['o', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zechen.
pub(crate) fn n1528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1364);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2974);
			s.is_empty()
		}
		['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word anschleichen.
pub(crate) fn n14260(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word einschleichen.
pub(crate) fn n14261(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -breichen.
pub(crate) fn n7196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5181);
	match s {
		[] => true,
		['a', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kreichen.
pub(crate) fn n7197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5182);
	match s {
		[] => true,
		['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nreichen.
pub(crate) fn n7198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5183);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6738);
			match s {
				[] => true,
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6739);
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

/// Words ending -rreichen.
pub(crate) fn n7199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5184);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6740);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'ü', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sreichen.
pub(crate) fn n7200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5185);
	match s {
		[] => true,
		['u', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -treichen.
pub(crate) fn n7201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5186);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7461);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7462);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7463);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7464);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7465);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7466);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', 'n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7467);
					match s {
						[] => true,
						['u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word seichen.
pub(crate) fn n3708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2980);
	s.is_empty()
}

/// Words ending -weichen.
pub(crate) fn n3709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2981);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5187);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5188);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5189);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5190);
			match s {
				[] => true,
				['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5191);
			match s {
				[] => true,
				['i', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5192);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5193);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5194);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lichen.
pub(crate) fn n1531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1367);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2982);
			match s {
				[] => true,
				['h', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6742);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2983);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 5195);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pichen.
pub(crate) fn n1532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1368);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2984);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2985);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2986);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hmischen.
pub(crate) fn n7249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5219);
	match s {
		[] => true,
		['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
			super::push(out, 6749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nmischen.
pub(crate) fn n7250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5220);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6750);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6751);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rmischen.
pub(crate) fn n7251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5221);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 7469);
					match s {
						[] => true,
						['n', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 7470);
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

/// Words ending -tmischen.
pub(crate) fn n7252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5222);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6752);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6753);
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

/// Words ending -rischen.
pub(crate) fn n3745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3014);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6754);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6755);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6756);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6757);
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

/// Words ending -wischen.
pub(crate) fn n3747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3016);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5223);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5224);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5225);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5226);
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5227);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5228);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5229);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zischen.
pub(crate) fn n3748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3017);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5230);
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

/// Words ending -lschen.
pub(crate) fn n1547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1382);
	match s {
		[] => true,
		['ä', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5231);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5232);
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
		_ => false,
	}
}

/// The word verramschen.
pub(crate) fn n1548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1383);
	s.is_empty()
}

/// Words ending -nschen.
pub(crate) fn n1549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1384);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3018);
			match s {
				[] => true,
				['m', 'r', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 5233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 3019);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6759);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6760);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7471);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6761);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oschen.
pub(crate) fn n1550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1385);
	s.is_empty()
}

/// Words ending -pschen.
pub(crate) fn n1551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1386);
	match s {
		[] => true,
		['a', 'r', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5234);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rschen.
pub(crate) fn n1552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1387);
	match s {
		[] => true,
		['a', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 3020);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3021);
			match s {
				[] => true,
				['n', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6762);
							match s {
								[] => true,
								['e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3022);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 6763);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6764);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3023);
			match s {
				[] => true,
				['e', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6765);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6766);
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tschen.
pub(crate) fn n1553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1388);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3024);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5235);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7473);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 7737);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7474);
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
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6767);
							match s {
								[] => true,
								['e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6768);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5237);
					match s {
						[] => true,
						['q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7475);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7476);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'l', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 7477);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7478);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7479);
									s.is_empty()
								}
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
			super::push(out, 3025);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5238);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5239);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 7480);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5240);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5241);
					match s {
						[] => true,
						['q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7481);
									match s {
										[] => true,
										['e', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7482);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 7483);
									match s {
										[] => true,
										['o', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3026);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3027);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5242);
					match s {
						[] => true,
						['n', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7484);
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
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5243);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6771);
							match s {
								[] => true,
								['n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5244);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5246);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6772);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t5::n7289(s, out),
				['w', s @ ..] => {
					super::push(out, 5249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 3028);
			match s {
				[] => true,
				['d', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uschen.
pub(crate) fn n1554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1389);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3029);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5251);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6777);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5252);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6778);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6779);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6780);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6781);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6782);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6783);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6784);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6785);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3030);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6786);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3031);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 5255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5256);
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
		['ä', s @ ..] => {
			super::push(out, 3033);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6788);
							match s {
								[] => true,
								['e', 'w', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6789);
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6790);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öschen.
pub(crate) fn n1555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1390);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5257);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5258);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5259);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üschen.
pub(crate) fn n1556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1391);
	s.is_empty()
}

/// The word buchen.
pub(crate) fn n3774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3040);
	s.is_empty()
}

/// Words ending -bbuchen.
pub(crate) fn n3775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3041);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mbuchen.
pub(crate) fn n3776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3042);
	match s {
		[] => true,
		['u', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rbuchen.
pub(crate) fn n3777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3043);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', 'ü', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -euchen.
pub(crate) fn n1559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1394);
	match s {
		[] => true,
		['h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5279);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5280);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3044);
			s.is_empty()
		}
		['l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 3045);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -luchen.
pub(crate) fn n1560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1395);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5281);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ruchen.
pub(crate) fn n1561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1396);
	match s {
		[] => true,
		['p', 's', 'n', 'a', 'e', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -suchen.
pub(crate) fn n1562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1397);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3047);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3048);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 3049);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3050);
			match s {
				[] => true,
				['i', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3051);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 5282);
					s.is_empty()
				}
				['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3052);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3053);
			match s {
				[] => true,
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6806);
							match s {
								[] => true,
								['e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tuchen.
pub(crate) fn n1563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1398);
	s.is_empty()
}

/// The word räuchen.
pub(crate) fn n1564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1399);
	s.is_empty()
}

/// Words ending -erziehen.
pub(crate) fn n7398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5302);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6810);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7492);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', '\u{1}', s @ ..] => {
					super::push(out, 7493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 6811);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 6812);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7494);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6813);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6814);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6815);
			match s {
				[] => true,
				['i', 'e', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7740);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7805);
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6816);
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

/// Words ending -osziehen.
pub(crate) fn n7400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5304);
	match s {
		[] => true,
		['l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usziehen.
pub(crate) fn n7401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5305);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tziehen.
pub(crate) fn n3835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3078);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5306);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5307);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', 'e', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'a', 'l', 'g', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uziehen.
pub(crate) fn n3836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3079);
	match s {
		[] => true,
		['z', s @ ..] => {
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

/// Words ending -erstehen.
pub(crate) fn n7457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5324);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6821);
			match s {
				[] => true,
				['ü', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7741);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7742);
							match s {
								[] => true,
								['e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 6822);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 7497);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7806);
									s.is_empty()
								}
								['r', 'e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6823);
			match s {
				[] => true,
				['ä', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 6824);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 6825);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ß', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tstehen.
pub(crate) fn n3878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3116);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5328);
			match s {
				[] => true,
				['e', 'r', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5329);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5330);
			match s {
				[] => true,
				['e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zustehen.
pub(crate) fn n7464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5331);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word beystehen.
pub(crate) fn n3880(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -hmecken.
pub(crate) fn n4004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3191);
	match s {
		[] => true,
		['c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6839);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6840);
					match s {
						[] => true,
						['u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -necken.
pub(crate) fn n1666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1479);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3192);
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

/// Words ending -pecken.
pub(crate) fn n1667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1480);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3193);
			s.is_empty()
		}
		['s', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -recken.
pub(crate) fn n1668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1481);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3195);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3196);
			match s {
				[] => true,
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3197);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6841);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6842);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 6843);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 6844);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6845);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 7501);
									s.is_empty()
								}
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
			super::push(out, 3198);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3199);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6846);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6847);
							match s {
								[] => true,
								['l', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6848);
							match s {
								[] => true,
								['i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6849);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7502);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 7743);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 7503);
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
						['s', s @ ..] => {
							super::push(out, 6850);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tecken.
pub(crate) fn n1669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1482);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5356);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5357);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5358);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5359);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5360);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6851);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6852);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5361);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5362);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wecken.
pub(crate) fn n1670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1483);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3200);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 3201);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3202);
			match s {
				[] => true,
				['i', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3203);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6853);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nnicken.
pub(crate) fn n4032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3213);
	match s {
		[] => true,
		['i', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unicken.
pub(crate) fn n4033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3214);
	match s {
		[] => true,
		['z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -picken.
pub(crate) fn n1678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1491);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 3215);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'z', '\u{1}', s @ ..] => {
			super::push(out, 3216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ricken.
pub(crate) fn n1679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1492);
	match s {
		[] => true,
		['t', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5386);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5387);
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
		_ => false,
	}
}

/// Words ending -ticken.
pub(crate) fn n1680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1493);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3217);
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5388);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5389);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wicken.
pub(crate) fn n1682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1495);
	match s {
		[] => true,
		['z', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5390);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5391);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5392);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6858);
							s.is_empty()
						}
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5393);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zicken.
pub(crate) fn n1683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1496);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3218);
			match s {
				[] => true,
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word hinunterschlucken.
pub(crate) fn n14274(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word runterschlucken.
pub(crate) fn n14275(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word runterdrücken.
pub(crate) fn n14276(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word herunterdrücken.
pub(crate) fn n14277(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -inrücken.
pub(crate) fn n7666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5431);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6867);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 6868);
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

/// Words ending -rrücken.
pub(crate) fn n4095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3254);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5432);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6869);
					match s {
						[] => true,
						['ä', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5433);
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

/// Words ending -srücken.
pub(crate) fn n4096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3255);
	match s {
		[] => true,
		['u', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -trücken.
pub(crate) fn n4097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3256);
	match s {
		[] => true,
		['n', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tücken.
pub(crate) fn n1707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1520);
	match s {
		[] => true,
		['s', 'h', 'ü', 'r', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5434);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5435);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zücken.
pub(crate) fn n1708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1521);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3257);
			s.is_empty()
		}
		['t', 'n', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -abstellen.
pub(crate) fn n11080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6915);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -estellen.
pub(crate) fn n7877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5528);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6916);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7521);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7522);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7523);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7524);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7525);
					match s {
						[] => true,
						['o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fstellen.
pub(crate) fn n7878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5529);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 6917);
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

/// Words ending -gstellen.
pub(crate) fn n7879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5530);
	match s {
		[] => true,
		['i', 't', 'r', 'e', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hstellen.
pub(crate) fn n7880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5531);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'e', 'l', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -istellen.
pub(crate) fn n7881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5532);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// The word zurückstellen.
pub(crate) fn n7882(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -mstellen.
pub(crate) fn n7883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5533);
	match s {
		[] => true,
		['i', 'e', 'h', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6918);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6919);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7527);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nstellen.
pub(crate) fn n7884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5534);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6920);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'o', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'n', 'i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6921);
			match s {
				[] => true,
				['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6922);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7528);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7529);
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

/// Words ending -rstellen.
pub(crate) fn n7885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5535);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6923);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'k', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7530);
					s.is_empty()
				}
				['b', 'ü', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7749);
							s.is_empty()
						}
						['n', 'e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7531);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7750);
							match s {
								[] => true,
								['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'n', 'u', s @ ..] => {
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
				['v', s @ ..] => {
					super::push(out, 7532);
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
			super::push(out, 6925);
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

/// Words ending -sstellen.
pub(crate) fn n7886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5536);
	match s {
		[] => true,
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tstellen.
pub(crate) fn n7887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5537);
	match s {
		[] => true,
		['i', 'e', 'r', 'e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6926);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'e', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'a', 'l', 'g', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ustellen.
pub(crate) fn n7888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5538);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 6927);
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

/// Words ending -uellen.
pub(crate) fn n1810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1610);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 3427);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5539);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5540);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wellen.
pub(crate) fn n1811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1611);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3428);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6928);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6929);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 6930);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7533);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6931);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6932);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3429);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eikommen.
pub(crate) fn n8014(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6955);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 7540);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mkommen.
pub(crate) fn n4485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3520);
	match s {
		[] => true,
		['i', 'e', 'h', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nkommen.
pub(crate) fn n4486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3521);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5597);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'v', '\u{1}', s @ ..] => {
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
				['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', 'c', 's', 'i', 'w', 'z', 'a', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6956);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'v', 'a', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rkommen.
pub(crate) fn n4487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3522);
	match s {
		[] => true,
		['a', 'l', 'k', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5598);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6957);
					match s {
						[] => true,
						['ü', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7757);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'e', 'i', s @ ..] => {
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ä', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5599);
			match s {
				[] => true,
				['p', 'm', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -skommen.
pub(crate) fn n4488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3523);
	match s {
		[] => true,
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tkommen.
pub(crate) fn n4489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3524);
	match s {
		[] => true,
		['h', 'c', 'e', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5600);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 6959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ukommen.
pub(crate) fn n4490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3525);
	match s {
		[] => true,
		['z', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rommen.
pub(crate) fn n1898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1680);
	s.is_empty()
}

/// The word runterkönnen.
pub(crate) fn n14288(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word drunterkönnen.
pub(crate) fn n14289(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word herunterkönnen.
pub(crate) fn n14290(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -bkommandieren.
pub(crate) fn n14062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7761);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word rekommandieren.
pub(crate) fn n14063(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word herumkommandieren.
pub(crate) fn n14064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7762);
	s.is_empty()
}

/// Words ending -tendieren.
pub(crate) fn n8291(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -fundieren.
pub(crate) fn n8292(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -odieren.
pub(crate) fn n4803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3737);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5711);
			s.is_empty()
		}
		['r', 'r', s @ ..] => {
			super::push(out, 5712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -udieren.
pub(crate) fn n4804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3738);
	match s {
		[] => true,
		['t', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7014);
					match s {
						[] => true,
						['e', 't', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 7554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lädieren.
pub(crate) fn n4805(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ffieren.
pub(crate) fn n2061(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 't', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5713);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'a', 'h', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gieren.
pub(crate) fn n2062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1804);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3739);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3740);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7016);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7555);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3741);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7017);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7556);
									s.is_empty()
								}
								_ => false,
							}
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
		['i', s @ ..] => {
			super::push(out, 3742);
			match s {
				[] => true,
				['r', 'i', s @ ..] => {
					super::push(out, 5714);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7557);
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
		['n', s @ ..] => {
			super::push(out, 3743);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7018);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'l', s @ ..] => {
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
		['r', 'u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hieren.
pub(crate) fn n2063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1805);
	match s {
		[] => true,
		['c', 's', s @ ..] => {
			super::push(out, 3744);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5715);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5716);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 7559);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7560);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 7561);
									match s {
										[] => true,
										['o', 'l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3745);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -liieren.
pub(crate) fn n2064(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kieren.
pub(crate) fn n2065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1806);
	match s {
		[] => true,
		['n', 'a', 'r', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'a', 'm', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -lieren.
pub(crate) fn n2066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1807);
	match s {
		[] => true,
		['a', 'k', 's', s @ ..] => {
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
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'p', 'm', 'o', 'k', s @ ..] => {
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
		['l', s @ ..] => {
			super::push(out, 3746);
			match s {
				[] => true,
				['i', 't', s @ ..] => {
					super::push(out, 5718);
					match s {
						[] => true,
						['s', 'e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7562);
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
			super::push(out, 3747);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 7019);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3748);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5719);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 7020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3749);
			match s {
				[] => true,
				['b', 'a', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5720);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7021);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
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
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 5721);
					match s {
						[] => true,
						['l', 'a', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7564);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 7763);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
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
				['m', s @ ..] => {
					super::push(out, 5722);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7022);
							match s {
								[] => true,
								['o', 'f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 7764);
											match s {
												[] => true,
												['u', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 7765);
											match s {
												[] => true,
												['o', 'v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'i', 'p', 'a', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mieren.
pub(crate) fn n2067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1808);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3750);
			match s {
				[] => true,
				['s', 'l', 'a', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7023);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7565);
									s.is_empty()
								}
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
			super::push(out, 3751);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7024);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7025);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7566);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7026);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7567);
									s.is_empty()
								}
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
			super::push(out, 3752);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', 'm', 'o', 'k', s @ ..] => {
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
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3753);
			match s {
				[] => true,
				['a', 'r', 'g', 'o', 'r', s @ ..] => {
					super::push(out, 5723);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7568);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7569);
									match s {
										[] => true,
										['o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3754);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
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
		['u', 's', 'n', 'o', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nieren.
pub(crate) fn n2068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1809);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3755);
			match s {
				[] => true,
				['l', 'f', s @ ..] => {
					super::push(out, 5724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', 's', s @ ..] => {
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3756);
			match s {
				[] => true,
				['d', 'r', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5725);
					match s {
						[] => true,
						['c', 'a', 'm', s @ ..] => {
							super::push(out, 7027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 't', 'n', 'o', 'k', s @ ..] => {
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
						['r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3757);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5726);
					match s {
						[] => true,
						['e', 'l', 'e', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7570);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 7766);
											match s {
												[] => true,
												['r', 'e', 't', 'n', 'i', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'p', 'e', 'l', 'e', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7028);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5727);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 7029);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7767);
											match s {
												[] => true,
												['e', 'h', 'r', 'e', 't', 'n', 'i', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 7768);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 's', 'i', 'm', s @ ..] => {
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
						['t', s @ ..] => {
							super::push(out, 7030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5728);
					match s {
						[] => true,
						['m', 'o', s @ ..] => {
							super::push(out, 7031);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 7769);
											match s {
												[] => true,
												['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
													super::push(out, 7807);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'u', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ä', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7032);
							match s {
								[] => true,
								['i', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 7770);
											match s {
												[] => true,
												['u', '\u{1}', s @ ..] => {
													super::push(out, 7808);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ä', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5729);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 7771);
											match s {
												[] => true,
												['i', 'e', '\u{1}', s @ ..] => {
													super::push(out, 7809);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 7572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pieren.
pub(crate) fn n2069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1810);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3758);
			s.is_empty()
		}
		['p', 'o', s @ ..] => {
			super::push(out, 3759);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rieren.
pub(crate) fn n2070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1811);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3760);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 't', 'u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5730);
					match s {
						[] => true,
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', 'y', 'h', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 3761);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7034);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'e', 'g', s @ ..] => {
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
						_ => false,
					}
				}
				['p', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3762);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5731);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5732);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5733);
					match s {
						[] => true,
						['i', 'h', 'c', s @ ..] => {
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
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5734);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 7035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5735);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5736);
					match s {
						[] => true,
						['s', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 7036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5737);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 7037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3763);
			match s {
				[] => true,
				['i', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7038);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3764);
			match s {
				[] => true,
				['b', 'a', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3765);
			match s {
				[] => true,
				['l', 'i', 'f', s @ ..] => {
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
				['n', 'e', 'z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'i', 'n', 'i', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 3766);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 5738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7040);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'k', 'u', s @ ..] => {
					super::push(out, 5739);
					match s {
						[] => true,
						['r', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 7575);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3767);
			match s {
				[] => true,
				['ö', 'n', 'a', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7041);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sieren.
pub(crate) fn n2071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1812);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3768);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5740);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['g', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'r', 'e', 't', 'a', 'm', s @ ..] => {
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
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', 'm', 'i', 'r', 'k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', 'n', 'e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'i', 't', s @ ..] => {
											match s {
												[] => true,
												['a', 'n', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n14330(s, out),
														['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => super::t5::n14331(s, out),
														_ => false,
													}
												}
												['u', 't', 'i', 't', 's', 'n', 'i', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n14332(s, out),
														['e', 'r', '\u{1}', s @ ..] => super::t5::n14333(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', 'n', 'i', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'x', 'e', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'b', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 's', s @ ..] => {
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
								['i', 's', 'n', 'e', 's', s @ ..] => {
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
								['o', 'm', s @ ..] => {
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
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 7042);
							match s {
								[] => true,
								['a', 't', 's', 'i', 'r', 'k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 7772);
											match s {
												[] => true,
												['u', '\u{1}', s @ ..] => {
													super::push(out, 7810);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['a', 'k', 'i', 'r', 'e', 'm', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'o', 'k', s @ ..] => super::t5::n13336(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', 't', 'i', 'l', 'i', 'm', s @ ..] => super::t5::n8380(s, out),
				['t', s @ ..] => super::t5::n8381(s, out),
				['v', s @ ..] => super::t5::n8382(s, out),
				_ => false,
			}
		}
		['n', 'e', 'z', s @ ..] => super::t5::n4850(s, out),
		['o', 'd', s @ ..] => super::t5::n4851(s, out),
		['s', 'e', 'r', 'd', s @ ..] => super::t5::n4852(s, out),
		_ => false,
	}
}

/// Words ending -tieren.
pub(crate) fn n2072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1813);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3769);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 7045);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7578);
									s.is_empty()
								}
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
			super::push(out, 3770);
			match s {
				[] => true,
				['r', 'p', 'r', 'e', 't', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7046);
							match s {
								[] => true,
								['h', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7047);
							match s {
								[] => true,
								['i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7580);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ß', 'i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3771);
			match s {
				[] => true,
				['l', 'i', 'b', 'a', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 3772);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3773);
			match s {
				[] => true,
				['a', 'i', 'v', 'o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', 'r', 'f', s @ ..] => {
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
								['i', s @ ..] => {
									super::push(out, 7581);
									match s {
										[] => true,
										['r', 'e', 'p', 'x', 'e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['m', s @ ..] => {
													super::push(out, 7811);
													match s {
														[] => true,
														['u', 'r', s @ ..] => super::t5::n14334(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'ä', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'm', s @ ..] => {
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3774);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 7048);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3775);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7049);
							match s {
								[] => true,
								['i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3776);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5743);
					match s {
						[] => true,
						['u', 'q', s @ ..] => super::t5::n11526(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t5::n8402(s, out),
				['o', s @ ..] => super::t5::n8403(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t5::n4861(s, out),
		['t', s @ ..] => {
			super::push(out, 3778);
			s.is_empty()
		}
		['u', s @ ..] => super::t5::n4863(s, out),
		_ => false,
	}
}

/// Words ending -uieren.
pub(crate) fn n2073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1814);
	match s {
		[] => true,
		['r', 't', 's', 'n', 'o', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vieren.
pub(crate) fn n2074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1815);
	match s {
		[] => true,
		['i', 't', s @ ..] => {
			match s {
				[] => true,
				['k', 'a', s @ ..] => {
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', s @ ..] => {
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
				_ => false,
			}
		}
		['r', 'e', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
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
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'k', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zieren.
pub(crate) fn n2075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1816);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3780);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3781);
			match s {
				[] => true,
				['p', 's', s @ ..] => {
					super::push(out, 5748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3782);
			match s {
				[] => true,
				['d', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', 'r', 'e', 'v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'a', 'n', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
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
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 5749);
					match s {
						[] => true,
						['o', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7590);
									match s {
										[] => true,
										['i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7777);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['b', 'u', 'p', s @ ..] => {
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
						['p', 'm', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3783);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5750);
					match s {
						[] => true,
						['n', 'i', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 7591);
									match s {
										[] => true,
										['c', 'a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 7814);
													match s {
														[] => true,
														['n', 'u', '\u{1}', s @ ..] => super::t5::n14336(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 7778);
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
								['t', s @ ..] => {
									super::push(out, 7592);
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
				['e', s @ ..] => {
					super::push(out, 5751);
					match s {
						[] => true,
						['r', 'e', 'f', 'f', 'i', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7593);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7779);
											s.is_empty()
										}
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
					super::push(out, 5752);
					match s {
						[] => true,
						['r', 'e', 'v', '\u{1}', s @ ..] => {
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
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
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
								['r', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3784);
			match s {
				[] => true,
				['e', 'l', 'o', s @ ..] => {
					super::push(out, 5753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3785);
			match s {
				[] => true,
				['d', 'o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 7053);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -messen.
pub(crate) fn n2211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1910);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3983);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3984);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3985);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3986);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5875);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3987);
			match s {
				[] => true,
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3988);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3989);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3990);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nessen.
pub(crate) fn n2212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1911);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3991);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
			super::push(out, 3992);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3993);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ressen.
pub(crate) fn n2213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1912);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3994);
			match s {
				[] => true,
				['e', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'i', 'e', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3995);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5876);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5877);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5878);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5879);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5880);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3996);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 5881);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5882);
					match s {
						[] => true,
						['e', 'r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5883);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5884);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5885);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itessen.
pub(crate) fn n5145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3998);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uessen.
pub(crate) fn n2215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1914);
	match s {
		[] => true,
		['m', '\u{1}', s @ ..] => {
			super::push(out, 3999);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -enhalten.
pub(crate) fn n8880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5989);
	match s {
		[] => true,
		['b', 'e', 'n', 'a', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', 'f', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inhalten.
pub(crate) fn n8881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5990);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'k', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rhalten.
pub(crate) fn n5334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4126);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5991);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7203);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7204);
					match s {
						[] => true,
						['e', 'i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', 'n', 'i', 'e', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 7659);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7799);
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['a', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7205);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5992);
			match s {
				[] => true,
				['p', 'm', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', 'f', 'a', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -shalten.
pub(crate) fn n5335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4127);
	match s {
		[] => true,
		['s', 'a', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7206);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -thalten.
pub(crate) fn n5336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4128);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5993);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7662);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7663);
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'e', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zuhalten.
pub(crate) fn n8891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5994);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word maßhalten.
pub(crate) fn n5338(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word niederspritzen.
pub(crate) fn n14326(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word festsitzen.
pub(crate) fn n9189(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -witzen.
pub(crate) fn n2515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2146);
	match s {
		[] => true,
		['h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6111);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6112);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6113);
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
		_ => false,
	}
}

/// Words ending -füttern.
pub(crate) fn n6228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4687);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
			super::push(out, 6449);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6450);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6451);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hringen.
pub(crate) fn n3579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2887);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5104);
			match s {
				[] => true,
				['r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mringen.
pub(crate) fn n3580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2888);
	match s {
		[] => true,
		['u', s @ ..] => {
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

/// Words ending -pringen.
pub(crate) fn n3581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2889);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 5106);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6669);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6670);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6671);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6672);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6673);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6674);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6675);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6676);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6677);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6678);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7439);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7440);
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
				['r', s @ ..] => {
					super::push(out, 6679);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7441);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7726);
									match s {
										[] => true,
										['ü', '\u{1}', s @ ..] => {
											super::push(out, 7804);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 7727);
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
							super::push(out, 7442);
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
				['t', s @ ..] => {
					super::push(out, 6680);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6681);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rringen.
pub(crate) fn n3582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2890);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5107);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6682);
					s.is_empty()
				}
				['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6683);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wringen.
pub(crate) fn n3583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2891);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5108);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5109);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5110);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rutschen.
pub(crate) fn n7289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5248);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6773);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6774);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6775);
			match s {
				[] => true,
				['e', 't', 'n', 'u', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6776);
			match s {
				[] => true,
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 7738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word nationalisieren.
pub(crate) fn n14330(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word internationalisieren.
pub(crate) fn n14331(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word institutionalisieren.
pub(crate) fn n14332(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word reinstitutionalisieren.
pub(crate) fn n14333(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -kolonisieren.
pub(crate) fn n13336(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'n', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -militarisieren.
pub(crate) fn n8380(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'n', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tisieren.
pub(crate) fn n8381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5741);
	match s {
		[] => true,
		['a', 'm', 'i', s @ ..] => {
			super::push(out, 7043);
			match s {
				[] => true,
				['l', 'k', 'k', s @ ..] => {
					super::push(out, 7576);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'n', 'g', 'a', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -visieren.
pub(crate) fn n8382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5742);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 7044);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 7577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zensieren.
pub(crate) fn n4850(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dosieren.
pub(crate) fn n4851(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dressieren.
pub(crate) fn n4852(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rumexperimentieren.
pub(crate) fn n14334(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 7816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -quartieren.
pub(crate) fn n11526(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7584);
			match s {
				[] => true,
				['i', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7585);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ertieren.
pub(crate) fn n8402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5744);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 7050);
			s.is_empty()
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ortieren.
pub(crate) fn n8403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5745);
	match s {
		[] => true,
		['p', s @ ..] => {
			match s {
				[] => true,
				['p', 'a', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 7586);
					match s {
						[] => true,
						['n', 'a', 'r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7812);
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
		['s', s @ ..] => {
			super::push(out, 7051);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7587);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stieren.
pub(crate) fn n4861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3777);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5746);
			s.is_empty()
		}
		['e', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
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
				['x', s @ ..] => {
					super::push(out, 7052);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'k', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7774);
									match s {
										[] => true,
										['e', 't', 'i', 'e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 7813);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['g', s @ ..] => {
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
				['j', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -utieren.
pub(crate) fn n4863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3779);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 5747);
			match s {
				[] => true,
				['s', 'i', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7588);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7589);
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word unterfinanzieren.
pub(crate) fn n14336(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

