use flexion::Path;

/// Words ending -tor.
pub(crate) fn n1791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1769);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5328);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5329);
			match s {
				[] => true,
				['d', 'i', 'l', s @ ..] => {
					super::push(out, 9558);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9559);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9560);
					match s {
						[] => true,
						['a', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12584);
							s.is_empty()
						}
						['e', 'r', 'g', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 12585);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 14353);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 15136);
											s.is_empty()
										}
										['o', 'r', 't', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 15137);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 14354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'r', 'i', '\u{1}', s @ ..] => {
							super::push(out, 12586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9561);
					match s {
						[] => true,
						['d', 'e', s @ ..] => {
							super::push(out, 12587);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15138);
											s.is_empty()
										}
										['o', 'r', 'u', 'e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 15139);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 12588);
							match s {
								[] => true,
								['e', 'r', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 14355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', 'w', s @ ..] => {
					super::push(out, 9562);
					match s {
						[] => true,
						['e', 'r', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 12589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 9563);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12590);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 14356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 12591);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['i', 'l', 'a', s @ ..] => {
											super::push(out, 15140);
											match s {
												[] => true,
												['w', 'k', '\u{1}', s @ ..] => {
													super::push(out, 15414);
													s.is_empty()
												}
												_ => false,
											}
										}
										['y', s @ ..] => {
											super::push(out, 15141);
											match s {
												[] => true,
												['t', 's', s @ ..] => {
													super::push(out, 15415);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 14357);
									match s {
										[] => true,
										['u', 'm', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 15142);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12592);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 12593);
							match s {
								[] => true,
								['d', 'n', 'i', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15143);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 15144);
											s.is_empty()
										}
										['w', '\u{1}', s @ ..] => {
											super::push(out, 15145);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t4::n10080(s, out),
				['m', s @ ..] => super::t4::n10081(s, out),
				['n', s @ ..] => super::t4::n10082(s, out),
				['r', s @ ..] => super::t4::n10083(s, out),
				['s', s @ ..] => {
					super::push(out, 9566);
					s.is_empty()
				}
				['t', s @ ..] => super::t4::n10085(s, out),
				['w', s @ ..] => super::t4::n10086(s, out),
				['z', s @ ..] => super::t4::n10087(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5330);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 9570);
					s.is_empty()
				}
				['l', 'e', 'i', 'k', 'z', 's', '\u{1}', s @ ..] => super::t4::n10089(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t4::n5539(s, out),
		['k', s @ ..] => super::t4::n5540(s, out),
		['n', s @ ..] => super::t4::n5541(s, out),
		['o', s @ ..] => super::t4::n5542(s, out),
		['p', s @ ..] => super::t4::n5543(s, out),
		['s', s @ ..] => super::t4::n5544(s, out),
		['u', s @ ..] => super::t4::n5545(s, out),
		['y', s @ ..] => super::t4::n5546(s, out),
		['z', s @ ..] => super::t4::n5547(s, out),
		['ą', s @ ..] => super::t4::n5548(s, out),
		_ => false,
	}
}

/// Words ending -wor.
pub(crate) fn n1793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1771);
	match s {
		[] => true,
		['k', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5340);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5341);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zor.
pub(crate) fn n1794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1772);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5342);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					super::push(out, 9594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5343);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 12640);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5344);
			match s {
				[] => true,
				['e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rus.
pub(crate) fn n1933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1908);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5635);
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 9772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5636);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5637);
			match s {
				[] => true,
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'c', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 9774);
					s.is_empty()
				}
				['n', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 9775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5638);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 9776);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12700);
							s.is_empty()
						}
						['a', 't', s @ ..] => {
							super::push(out, 12701);
							match s {
								[] => true,
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 14413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 12702);
							match s {
								[] => true,
								['r', 't', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 14414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 12703);
							s.is_empty()
						}
						['ś', '\u{1}', s @ ..] => {
							super::push(out, 12704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5639);
			match s {
				[] => true,
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5640);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5641);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5642);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5643);
			match s {
				[] => true,
				['y', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9779);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sus.
pub(crate) fn n1934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1909);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5644);
			s.is_empty()
		}
		['i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 5645);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tus.
pub(crate) fn n1935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1910);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9780);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 9781);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9782);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5646);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5647);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 'd', 'b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5648);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5649);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 5650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ativus.
pub(crate) fn n1936(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5651);
			s.is_empty()
		}
		['s', 'u', 'c', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5652);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wus.
pub(crate) fn n1937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1911);
	match s {
		[] => true,
		['e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5653);
			s.is_empty()
		}
		['i', 't', 'a', s @ ..] => {
			match s {
				[] => true,
				['l', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9786);
					s.is_empty()
				}
				['z', 'u', 'k', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 5654);
			s.is_empty()
		}
		['y', 'r', 'g', 'z', '\u{1}', s @ ..] => {
			super::push(out, 5655);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zus.
pub(crate) fn n1938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1912);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5656);
			match s {
				[] => true,
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łus.
pub(crate) fn n1939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1913);
	match s {
		[] => true,
		['u', '\u{1}', s @ ..] => {
			super::push(out, 5658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rant.
pub(crate) fn n6065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5834);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9932);
			s.is_empty()
		}
		['a', 'm', s @ ..] => {
			super::push(out, 9933);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 12745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9934);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9935);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 9936);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12746);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'p', 's', 'r', s @ ..] => {
			super::push(out, 9937);
			match s {
				[] => true,
				['e', 'p', 'y', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 12748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 9938);
			s.is_empty()
		}
		['o', 'd', 'o', s @ ..] => {
			super::push(out, 9939);
			match s {
				[] => true,
				['z', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 12749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', s @ ..] => {
			super::push(out, 9940);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12750);
					s.is_empty()
				}
				['e', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 12751);
					s.is_empty()
				}
				['s', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 12752);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -esant.
pub(crate) fn n10480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9941);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 12753);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14421);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14422);
					s.is_empty()
				}
				['p', 'e', 'd', 'y', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 14423);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tant.
pub(crate) fn n6067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5836);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 9944);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12754);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12755);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9945);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 12756);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word kontynuant.
pub(crate) fn n10485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9946);
	s.is_empty()
}

/// Words ending -want.
pub(crate) fn n6069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5838);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 9947);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9948);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żant.
pub(crate) fn n6070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5839);
	match s {
		[] => true,
		['a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 9950);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ent.
pub(crate) fn n2009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1981);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9951);
					s.is_empty()
				}
				['u', 'k', 'n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5840);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9953);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9954);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 9955);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 12757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9956);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12758);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 12759);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 14426);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15169);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 14427);
									s.is_empty()
								}
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
			super::push(out, 5841);
			match s {
				[] => true,
				['n', 'e', 'p', s @ ..] => {
					super::push(out, 9957);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'c', 'n', s @ ..] => {
					super::push(out, 9958);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 12761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5842);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9959);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 12762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9960);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9961);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'a', 'r', 't', 'n', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5843);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5844);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9963);
					s.is_empty()
				}
				['d', 'a', s @ ..] => {
					super::push(out, 9964);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 5845);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5846);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5847);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9966);
					match s {
						[] => true,
						['d', 'a', s @ ..] => {
							super::push(out, 12763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9967);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 12764);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 12765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', s @ ..] => {
					super::push(out, 9968);
					match s {
						[] => true,
						['p', 'y', s @ ..] => {
							super::push(out, 12766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9969);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 12767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5848);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9970);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 9971);
					s.is_empty()
				}
				['y', 't', 'n', s @ ..] => {
					super::push(out, 9972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5849);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9973);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9974);
					match s {
						[] => true,
						['f', 'e', s @ ..] => {
							super::push(out, 12768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'k', 'n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 9975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5850);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5851);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9977);
					s.is_empty()
				}
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9978);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', 'e', '\u{1}', s @ ..] => {
							super::push(out, 12769);
							s.is_empty()
						}
						['n', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9979);
					s.is_empty()
				}
				['o', 'p', s @ ..] => {
					match s {
						[] => true,
						['i', 'n', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12771);
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							super::push(out, 12772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'y', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9980);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 9981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['k', 'i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9982);
					s.is_empty()
				}
				['l', 'o', 's', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9984);
					s.is_empty()
				}
				['n', 'e', 'c', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -int.
pub(crate) fn n2010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1982);
	match s {
		[] => true,
		['l', 'p', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 5852);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5853);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5854);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5855);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 9986);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9988);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ont.
pub(crate) fn n2011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1983);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 5857);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5858);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 5859);
			s.is_empty()
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 5860);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5861);
			match s {
				[] => true,
				['c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5862);
			match s {
				[] => true,
				['b', 'm', s @ ..] => {
					super::push(out, 9990);
					match s {
						[] => true,
						['y', 's', '\u{1}', s @ ..] => {
							super::push(out, 12773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 's', 'y', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5863);
			s.is_empty()
		}
		['l', '\u{1}', s @ ..] => {
			super::push(out, 5864);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9991);
					s.is_empty()
				}
				['r', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 9992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5865);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9993);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 9994);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'y', 'r', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 5866);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unt.
pub(crate) fn n2012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1984);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 5867);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ynt.
pub(crate) fn n2013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1985);
	s.is_empty()
}

/// The word donkiszot.
pub(crate) fn n10577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10036);
	s.is_empty()
}

/// Words ending -łot.
pub(crate) fn n2028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2000);
	match s {
		[] => true,
		['k', 's', '\u{1}', s @ ..] => {
			super::push(out, 5918);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5919);
			s.is_empty()
		}
		['p', 'o', 'w', 'y', 'ż', '\u{1}', s @ ..] => {
			super::push(out, 5920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -acz.
pub(crate) fn n2227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2193);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6170);
			match s {
				[] => true,
				['e', 'z', 'r', 'g', 'o', s @ ..] => {
					super::push(out, 10171);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 12820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10172);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 14434);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15172);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 14435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 10173);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 12821);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14436);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 14437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 12822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', 'a', 'r', 'w', 'd', 'o', '\u{1}', s @ ..] => {
			super::push(out, 6171);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 6172);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10174);
					match s {
						[] => true,
						['ł', 'k', 'a', s @ ..] => {
							super::push(out, 12823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10175);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 12824);
							match s {
								[] => true,
								['w', 'o', 'p', s @ ..] => {
									match s {
										[] => true,
										['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 15173);
											s.is_empty()
										}
										['o', '\u{1}', s @ ..] => {
											super::push(out, 15174);
											s.is_empty()
										}
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
					super::push(out, 10176);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 12825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10177);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 12826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6173);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 6174);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12827);
							s.is_empty()
						}
						['ż', 'd', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10178);
					match s {
						[] => true,
						['c', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 12829);
							s.is_empty()
						}
						['m', 'ś', s @ ..] => {
							super::push(out, 12830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10179);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10180);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10181);
					s.is_empty()
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12831);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 12832);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15175);
											s.is_empty()
										}
										['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 15176);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'i', 'l', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 10182);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 10183);
					match s {
						[] => true,
						['i', 'c', 'a', s @ ..] => {
							super::push(out, 12833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 6175);
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 10184);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12834);
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 12835);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12836);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 12837);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 12838);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 14438);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 14439);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15177);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 14440);
									s.is_empty()
								}
								['z', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 14441);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 14442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 12839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6176);
			match s {
				[] => true,
				['b', 'a', 'r', s @ ..] => {
					super::push(out, 10185);
					match s {
						[] => true,
						['z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 12840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 'r', s @ ..] => {
					super::push(out, 10186);
					match s {
						[] => true,
						['k', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10187);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12842);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 14443);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', '\u{1}', s @ ..] => {
													super::push(out, 15424);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 15425);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 15178);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'h', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 14444);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 12843);
							match s {
								[] => true,
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14445);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ź', s @ ..] => {
							super::push(out, 12844);
							match s {
								[] => true,
								['e', 'z', 'r', 'd', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 10188);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 12845);
							match s {
								[] => true,
								['r', 'p', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 14447);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 12846);
							s.is_empty()
						}
						['k', 's', 'u', 'l', 'p', 'd', 'o', '\u{1}', s @ ..] => {
							super::push(out, 12847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => super::t4::n6423(s, out),
		['k', s @ ..] => super::t4::n6424(s, out),
		['l', s @ ..] => super::t4::n6425(s, out),
		['m', s @ ..] => super::t4::n6426(s, out),
		['n', s @ ..] => super::t4::n6427(s, out),
		['p', s @ ..] => super::t4::n6428(s, out),
		['r', s @ ..] => super::t4::n6429(s, out),
		['s', s @ ..] => super::t4::n6430(s, out),
		['t', s @ ..] => super::t4::n6431(s, out),
		['w', s @ ..] => super::t4::n6432(s, out),
		['z', s @ ..] => super::t4::n6433(s, out),
		['ł', s @ ..] => super::t4::n6434(s, out),
		['ż', s @ ..] => super::t4::n6435(s, out),
		_ => false,
	}
}

/// Words ending -ycz.
pub(crate) fn n2237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2203);
	match s {
		[] => true,
		['r', 'o', 'h', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 6220);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6221);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10270);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zcz.
pub(crate) fn n2238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2204);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10272);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 12922);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 12923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10273);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12924);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 12925);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 14479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 12926);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 14480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12927);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 12928);
							match s {
								[] => true,
								['r', 't', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 15188);
											s.is_empty()
										}
										['y', 'w', '\u{1}', s @ ..] => {
											super::push(out, 15189);
											s.is_empty()
										}
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
					super::push(out, 10274);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10275);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 12929);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 12930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10276);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12931);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 12932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10277);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 12933);
							s.is_empty()
						}
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 12934);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 12935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 10278);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 12936);
							s.is_empty()
						}
						['z', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 12937);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 12938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 10279);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 12939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęcz.
pub(crate) fn n2239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2205);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			super::push(out, 6222);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6223);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 10280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -larz.
pub(crate) fn n6498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6247);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10307);
			match s {
				[] => true,
				['k', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 12960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', 'c', 'e', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 10308);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10309);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['i', 'f', s @ ..] => {
					super::push(out, 12961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10310);
			match s {
				[] => true,
				['c', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10311);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10312);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 10313);
			match s {
				[] => true,
				['m', 'e', 'z', 'g', 'e', '\u{1}', s @ ..] => {
					super::push(out, 12964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10314);
			match s {
				[] => true,
				['b', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 12965);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12966);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14492);
							s.is_empty()
						}
						['o', 'n', 'l', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 14493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 12967);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12968);
					match s {
						[] => true,
						['i', 'p', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10315);
			s.is_empty()
		}
		['ś', s @ ..] => {
			super::push(out, 10316);
			match s {
				[] => true,
				['u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 12969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ź', s @ ..] => {
			super::push(out, 10317);
			match s {
				[] => true,
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 12970);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -inarz.
pub(crate) fn n10882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10319);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['r', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 14495);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 14496);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -onarz.
pub(crate) fn n10883(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 12971);
			match s {
				[] => true,
				['y', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 14497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', 'c', 'k', 'y', 'd', '\u{1}', s @ ..] => {
			super::push(out, 12972);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sarz.
pub(crate) fn n6501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6250);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10321);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10322);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10323);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 12973);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tarz.
pub(crate) fn n6502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6251);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10324);
			s.is_empty()
		}
		['h', 'c', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 10325);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10326);
			match s {
				[] => true,
				['m', 'r', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 12974);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12975);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 12976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10327);
			s.is_empty()
		}
		['o', 'k', 's', '\u{1}', s @ ..] => {
			super::push(out, 10328);
			s.is_empty()
		}
		['s', 'e', 'w', 'k', '\u{1}', s @ ..] => {
			super::push(out, 10329);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 10330);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -warz.
pub(crate) fn n6503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6252);
	match s {
		[] => true,
		['a', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 10331);
			s.is_empty()
		}
		['k', 'y', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 10332);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -erz.
pub(crate) fn n2271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2237);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6254);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							super::push(out, 12977);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 12978);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 14498);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 10334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6255);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10335);
					match s {
						[] => true,
						['l', 'a', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 12979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 10336);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 12980);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10337);
					match s {
						[] => true,
						['i', 'l', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 12981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 10338);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10339);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 12983);
							match s {
								[] => true,
								['s', 'k', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 14500);
									s.is_empty()
								}
								['z', 'c', s @ ..] => {
									super::push(out, 14501);
									match s {
										[] => true,
										['z', 's', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 15192);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 12984);
							match s {
								[] => true,
								['e', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 14502);
									s.is_empty()
								}
								['o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 14503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10340);
					match s {
						[] => true,
						['ł', 'o', s @ ..] => {
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 14504);
									s.is_empty()
								}
								['ż', '\u{1}', s @ ..] => {
									super::push(out, 14505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ś', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12985);
							s.is_empty()
						}
						['ź', 'u', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 12987);
							s.is_empty()
						}
						['ą', 'w', '\u{1}', s @ ..] => {
							super::push(out, 12988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 10341);
					match s {
						[] => true,
						['e', 'i', 'z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 12989);
							s.is_empty()
						}
						['l', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12990);
							s.is_empty()
						}
						['r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 12991);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 12992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6256);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10342);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 12993);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14506);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 14507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 10343);
					match s {
						[] => true,
						['n', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 12994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'i', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 10344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6257);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 10345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6258);
			match s {
				[] => true,
				['t', 'a', 'ł', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6259);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10347);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10348);
					match s {
						[] => true,
						['t', 'e', s @ ..] => {
							super::push(out, 12995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6260);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10349);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10350);
					s.is_empty()
				}
				['ł', 'a', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 6261);
			match s {
				[] => true,
				['s', 'ł', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 10352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hrz.
pub(crate) fn n2272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2238);
	match s {
		[] => true,
		['c', 'i', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 6262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orz.
pub(crate) fn n2273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2239);
	match s {
		[] => true,
		['g', 'ę', 'w', '\u{1}', s @ ..] => {
			super::push(out, 6263);
			s.is_empty()
		}
		['h', 'c', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 6264);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 6265);
			s.is_empty()
		}
		['t', 's', 'y', 's', 'n', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 6266);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -prz.
pub(crate) fn n2274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2240);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6267);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 12996);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 12997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -trz.
pub(crate) fn n2275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2241);
	match s {
		[] => true,
		['s', 'i', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10353);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 14509);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 14510);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 12998);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 12999);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 10354);
					s.is_empty()
				}
				['k', 'u', 't', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 10355);
					s.is_empty()
				}
				['l', 'e', 'p', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 10356);
					s.is_empty()
				}
				['o', 'i', 'n', 'g', 'o', '\u{1}', s @ ..] => {
					super::push(out, 10357);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10358);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13001);
							s.is_empty()
						}
						['ó', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 13002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 13003);
							s.is_empty()
						}
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 13004);
							s.is_empty()
						}
						['u', 'h', '\u{1}', s @ ..] => {
							super::push(out, 13005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 10359);
					s.is_empty()
				}
				['z', 'c', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10360);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urz.
pub(crate) fn n2276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2242);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 6268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -órz.
pub(crate) fn n2277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2243);
	s.is_empty()
}

/// Words ending -abosz.
pub(crate) fn n10961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10390);
	s.is_empty()
}

/// The word debosz.
pub(crate) fn n10962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10391);
	s.is_empty()
}

/// Words ending -bibosz.
pub(crate) fn n10963(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13012);
			s.is_empty()
		}
		['y', 'c', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 13013);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mbosz.
pub(crate) fn n10964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10392);
	match s {
		[] => true,
		['a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 13014);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -obosz.
pub(crate) fn n10965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10393);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 13015);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ubosz.
pub(crate) fn n10966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10394);
	s.is_empty()
}

/// Words ending -ębosz.
pub(crate) fn n10967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10395);
	s.is_empty()
}

/// Words ending -dosz.
pub(crate) fn n6549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6297);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10396);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gosz.
pub(crate) fn n6550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6298);
	match s {
		[] => true,
		['u', 'ł', 'd', '\u{1}', s @ ..] => {
			super::push(out, 10397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kosz.
pub(crate) fn n6551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6299);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10398);
			match s {
				[] => true,
				['m', 's', '\u{1}', s @ ..] => {
					super::push(out, 13016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'o', '\u{1}', s @ ..] => {
			super::push(out, 10399);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10400);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10401);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 13017);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14511);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 14512);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10402);
			match s {
				[] => true,
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 13018);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -losz.
pub(crate) fn n6552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6300);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 10403);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 13019);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mosz.
pub(crate) fn n6553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6301);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 10404);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nosz.
pub(crate) fn n6554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6302);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['k', 'c', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 13020);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14514);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 15193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 14515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -posz.
pub(crate) fn n6555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6303);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 10405);
			s.is_empty()
		}
		['u', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 10406);
			s.is_empty()
		}
		['ę', 't', '\u{1}', s @ ..] => {
			super::push(out, 10407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rosz.
pub(crate) fn n6556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6304);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10408);
			match s {
				[] => true,
				['j', '\u{1}', s @ ..] => {
					super::push(out, 13021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10409);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14516);
							s.is_empty()
						}
						['s', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 14517);
							s.is_empty()
						}
						['w', 'r', 'd', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 14518);
							s.is_empty()
						}
						['z', 'd', 'ę', 'w', '\u{1}', s @ ..] => {
							super::push(out, 14519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'z', 'c', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 13022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10410);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 10411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tosz.
pub(crate) fn n6557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6305);
	match s {
		[] => true,
		['s', 'u', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 13023);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 13024);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 13025);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wosz.
pub(crate) fn n6558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6306);
	match s {
		[] => true,
		['a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 10412);
			s.is_empty()
		}
		['i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 10413);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word wypłosz.
pub(crate) fn n6559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6307);
	s.is_empty()
}

/// Words ending -rsz.
pub(crate) fn n2284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2250);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6308);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6309);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6310);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 10414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usz.
pub(crate) fn n2285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2251);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6312);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6313);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10415);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10416);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6314);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6315);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10418);
					match s {
						[] => true,
						['a', 'ł', 's', '\u{1}', s @ ..] => {
							super::push(out, 13026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 10419);
					s.is_empty()
				}
				['d', 'u', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10420);
					s.is_empty()
				}
				['f', 'y', 'r', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 10421);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 10422);
					match s {
						[] => true,
						['i', 'b', 'u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 13027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 10423);
					s.is_empty()
				}
				['t', 'a', '\u{1}', s @ ..] => {
					super::push(out, 10424);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 10425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6316);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 10426);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10427);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							super::push(out, 13028);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10428);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 14521);
									match s {
										[] => true,
										['a', 'l', 'y', 's', '\u{1}', s @ ..] => {
											super::push(out, 15194);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 14522);
									match s {
										[] => true,
										['e', 'c', 'e', 'b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 15195);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 14523);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 15196);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 15427);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 15197);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 14524);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 15198);
											match s {
												[] => true,
												['c', 's', '\u{1}', s @ ..] => {
													super::push(out, 15428);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 15199);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 15429);
													match s {
														[] => true,
														['t', 's', 'e', 'w', 'k', '\u{1}', s @ ..] => super::t4::n16544(s, out),
														_ => false,
													}
												}
												['j', s @ ..] => {
													super::push(out, 15430);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 14525);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 15200);
											match s {
												[] => true,
												['l', 'g', '\u{1}', s @ ..] => {
													super::push(out, 15431);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 14526);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 15201);
											match s {
												[] => true,
												['n', '\u{1}', s @ ..] => {
													super::push(out, 15432);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6317);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10429);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10430);
					match s {
						[] => true,
						['b', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 13030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10431);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 13031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10432);
					match s {
						[] => true,
						['a', 'd', s @ ..] => {
							super::push(out, 13032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 6318);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6319);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10434);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 10435);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 10436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6320);
			s.is_empty()
		}
		['n', 'a', 'j', '\u{1}', s @ ..] => {
			super::push(out, 6321);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 6322);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6323);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 10437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6324);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10438);
					s.is_empty()
				}
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 10439);
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 10440);
					s.is_empty()
				}
				['n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 10441);
					s.is_empty()
				}
				['r', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 10442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 6325);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 6326);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 10443);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ysz.
pub(crate) fn n2286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2252);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6327);
			match s {
				[] => true,
				['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 10444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6328);
			match s {
				[] => true,
				['a', 'ł', 'g', '\u{1}', s @ ..] => {
					super::push(out, 10445);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 10446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6329);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6330);
			match s {
				[] => true,
				['o', 'w', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 10448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6331);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10449);
					s.is_empty()
				}
				['o', 'ł', '\u{1}', s @ ..] => {
					super::push(out, 10450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 6332);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 10451);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10452);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 6333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ósz.
pub(crate) fn n2287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2253);
	s.is_empty()
}

/// Words ending -łsz.
pub(crate) fn n2288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2254);
	s.is_empty()
}

/// Words ending -ższ.
pub(crate) fn n2289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2255);
	s.is_empty()
}

/// Words ending -ość.
pub(crate) fn n2336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2301);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6379);
			s.is_empty()
		}
		['d', 'a', 'r', s @ ..] => {
			super::push(out, 6380);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 6381);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10471);
					s.is_empty()
				}
				['a', 'n', 'ł', s @ ..] => {
					super::push(out, 10472);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10473);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'n', 't', 'e', 'l', 'o', 'ł', s @ ..] => {
			super::push(out, 6382);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 6383);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10475);
					s.is_empty()
				}
				['a', 'p', s @ ..] => {
					super::push(out, 10476);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 10477);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 10478);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 13039);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 13040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10479);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10480);
					s.is_empty()
				}
				['p', 'e', 'l', s @ ..] => {
					super::push(out, 10481);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10482);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 13041);
							s.is_empty()
						}
						['r', 'a', 'm', s @ ..] => {
							super::push(out, 13042);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 13043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 10483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6384);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 6385);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10484);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13044);
							match s {
								[] => true,
								['a', 'i', 'w', 'ś', s @ ..] => {
									super::push(out, 14527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 13045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6386);
			match s {
				[] => true,
				['c', 'e', 'b', 'o', 'e', s @ ..] => {
					super::push(out, 10485);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13046);
							s.is_empty()
						}
						['o', 'g', s @ ..] => {
							super::push(out, 13047);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 14528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 14529);
									s.is_empty()
								}
								['r', 't', s @ ..] => {
									super::push(out, 14530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ę', 'z', 'c', s @ ..] => {
							super::push(out, 13048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'l', s @ ..] => {
					super::push(out, 10486);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 10487);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 13049);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13050);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 14531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'c', 'k', 'a', 'r', s @ ..] => {
							super::push(out, 13051);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									super::push(out, 14532);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 10488);
					s.is_empty()
				}
				['l', s @ ..] => super::t4::n11067(s, out),
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13057);
							match s {
								[] => true,
								['i', 'c', s @ ..] => {
									super::push(out, 14546);
									s.is_empty()
								}
								['j', 'y', s @ ..] => {
									super::push(out, 14547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'ł', 'u', s @ ..] => super::t4::n13905(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t4::n11069(s, out),
				['o', s @ ..] => super::t4::n11070(s, out),
				['p', 'o', 'r', 'k', s @ ..] => super::t4::n11071(s, out),
				['r', s @ ..] => super::t4::n11072(s, out),
				['s', s @ ..] => super::t4::n11073(s, out),
				['t', s @ ..] => super::t4::n11074(s, out),
				['w', s @ ..] => super::t4::n11075(s, out),
				['z', s @ ..] => super::t4::n11076(s, out),
				['ś', s @ ..] => super::t4::n11077(s, out),
				['ż', s @ ..] => super::t4::n11078(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t4::n6640(s, out),
		['t', s @ ..] => super::t4::n6641(s, out),
		['w', s @ ..] => super::t4::n6642(s, out),
		['z', s @ ..] => super::t4::n6643(s, out),
		['ł', s @ ..] => super::t4::n6644(s, out),
		_ => false,
	}
}

/// Words ending -gacja.
pub(crate) fn n7118(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			super::push(out, 10754);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13210);
					s.is_empty()
				}
				['r', 'g', 'e', s @ ..] => {
					super::push(out, 13211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'w', s @ ..] => {
			super::push(out, 10755);
			s.is_empty()
		}
		['o', 'r', 'u', s @ ..] => {
			super::push(out, 10756);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jacja.
pub(crate) fn n7120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6829);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'p', 'a', s @ ..] => {
					super::push(out, 13212);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13213);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14629);
							match s {
								[] => true,
								['t', 's', 'b', 'u', 's', 'n', s @ ..] => {
									super::push(out, 15243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 's', 'i', s @ ..] => {
					super::push(out, 13214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kacja.
pub(crate) fn n7121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6830);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13215);
					match s {
						[] => true,
						['y', s @ ..] => {
							match s {
								[] => true,
								['r', 't', s @ ..] => {
									super::push(out, 15244);
									s.is_empty()
								}
								['t', 'n', s @ ..] => {
									super::push(out, 15245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13216);
					s.is_empty()
				}
				['n', 'u', 'm', 'o', 'k', 'e', s @ ..] => {
					super::push(out, 13217);
					s.is_empty()
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
						['i', s @ ..] => {
							super::push(out, 14630);
							s.is_empty()
						}
						['o', 'i', 'd', 'a', 'r', 'y', s @ ..] => {
							super::push(out, 14631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'd', 'e', 'e', s @ ..] => {
			super::push(out, 10759);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 14632);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'k', 'o', 't', 'e', s @ ..] => {
					super::push(out, 13219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lacja.
pub(crate) fn n7122(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'p', s @ ..] => {
			super::push(out, 10760);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10761);
			s.is_empty()
		}
		['f', 'n', 'i', 't', s @ ..] => {
			super::push(out, 10762);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10763);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['k', 'r', s @ ..] => {
					super::push(out, 13221);
					s.is_empty()
				}
				['p', 'a', s @ ..] => {
					super::push(out, 13222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10764);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13223);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 13224);
					s.is_empty()
				}
				['k', 'y', s @ ..] => {
					super::push(out, 13225);
					s.is_empty()
				}
				['m', 'y', s @ ..] => {
					super::push(out, 13226);
					s.is_empty()
				}
				['p', 'o', 'p', 'e', s @ ..] => {
					super::push(out, 13227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 't', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13228);
					s.is_empty()
				}
				['n', 'e', 'w', 'r', s @ ..] => {
					super::push(out, 13229);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -macja.
pub(crate) fn n7123(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 13230);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10765);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10766);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13232);
					s.is_empty()
				}
				['i', 'f', 'a', 'o', s @ ..] => {
					super::push(out, 13233);
					s.is_empty()
				}
				['o', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 14634);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10767);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nacja.
pub(crate) fn n7124(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 's', s @ ..] => {
			super::push(out, 10768);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 13234);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 13235);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 13236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', 'm', 'o', 'k', 'e', s @ ..] => {
					super::push(out, 13237);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 13238);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15246);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'e', s @ ..] => {
							super::push(out, 14636);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 14637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10769);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'k', 'n', 'i', 'e', s @ ..] => {
					super::push(out, 13239);
					s.is_empty()
				}
				['e', 'b', s @ ..] => {
					super::push(out, 13240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['d', 'r', 'o', 'b', s @ ..] => {
					super::push(out, 13241);
					s.is_empty()
				}
				['r', 't', s @ ..] => {
					super::push(out, 13242);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pacja.
pub(crate) fn n7125(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 10770);
			s.is_empty()
		}
		['y', 'c', 'n', s @ ..] => {
			super::push(out, 10771);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -racja.
pub(crate) fn n7126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6831);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10772);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13244);
					s.is_empty()
				}
				['c', 'a', s @ ..] => {
					super::push(out, 13245);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 13246);
					s.is_empty()
				}
				['m', 'u', s @ ..] => {
					super::push(out, 13247);
					s.is_empty()
				}
				['n', 'e', 'g', 'e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14638);
							s.is_empty()
						}
						['r', 'o', s @ ..] => {
							super::push(out, 14639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', 'o', s @ ..] => {
					super::push(out, 13248);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['e', 't', 'n', 'i', 'z', s @ ..] => {
					super::push(out, 13250);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 13251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'p', 's', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 13252);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10773);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13254);
					s.is_empty()
				}
				['d', 'a', 'ł', s @ ..] => {
					super::push(out, 13255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['l', 'i', 'f', 'o', s @ ..] => {
					super::push(out, 13256);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13257);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14640);
							s.is_empty()
						}
						['i', 'n', 'i', 'm', 'd', 'a', 'ł', s @ ..] => {
							super::push(out, 14641);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14642);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14643);
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
				['g', 'u', s @ ..] => {
					super::push(out, 13258);
					s.is_empty()
				}
				['k', 'e', s @ ..] => {
					super::push(out, 13259);
					s.is_empty()
				}
				['t', 'l', s @ ..] => {
					super::push(out, 13260);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sacja.
pub(crate) fn n7127(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			super::push(out, 10774);
			s.is_empty()
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13261);
					s.is_empty()
				}
				['i', 'f', 'a', s @ ..] => {
					super::push(out, 13262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10775);
			s.is_empty()
		}
		['n', 'e', 'd', 'n', 'o', 'k', 'i', s @ ..] => {
			super::push(out, 10776);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tacja.
pub(crate) fn n7128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6832);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13263);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'g', s @ ..] => {
			super::push(out, 10777);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10778);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13265);
					s.is_empty()
				}
				['l', 'i', s @ ..] => {
					super::push(out, 13266);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13267);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13268);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 13269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13270);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'o', 'o', s @ ..] => {
							super::push(out, 14644);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 15441);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 15442);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'u', s @ ..] => {
									super::push(out, 15248);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15249);
									s.is_empty()
								}
								['u', 'g', s @ ..] => {
									super::push(out, 15250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 14645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'r', 'o', s @ ..] => {
			super::push(out, 10779);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 10780);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10781);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13271);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'g', s @ ..] => {
			super::push(out, 10782);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					super::push(out, 13273);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13274);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uacja.
pub(crate) fn n7129(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13275);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 13276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13277);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13278);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wacja.
pub(crate) fn n7130(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10783);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10784);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10785);
			s.is_empty()
		}
		['y', 't', s @ ..] => {
			match s {
				[] => true,
				['k', 'a', 'e', s @ ..] => {
					super::push(out, 13279);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zacja.
pub(crate) fn n7131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6833);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 13281);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 15251);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 15252);
									s.is_empty()
								}
								['i', 'b', 'a', s @ ..] => {
									super::push(out, 15253);
									s.is_empty()
								}
								['j', 'c', s @ ..] => {
									super::push(out, 15254);
									s.is_empty()
								}
								['m', 'y', s @ ..] => {
									super::push(out, 15255);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 15443);
											s.is_empty()
										}
										['o', 'j', s @ ..] => {
											super::push(out, 15444);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 15256);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 15445);
											s.is_empty()
										}
										['i', s @ ..] => {
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 15474);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 15475);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 15257);
									match s {
										[] => true,
										['t', 'k', 'e', s @ ..] => {
											super::push(out, 15446);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14646);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', s @ ..] => {
					super::push(out, 13282);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 15258);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 15259);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15260);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', s @ ..] => {
							super::push(out, 14648);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14649);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 15261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 14650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 14651);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 14652);
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
				['c', s @ ..] => {
					super::push(out, 13283);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 13284);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 14653);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 15447);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 15448);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'l', s @ ..] => {
							super::push(out, 14654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 15262);
									s.is_empty()
								}
								['o', 'r', s @ ..] => {
									super::push(out, 15263);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14655);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							super::push(out, 14656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -icja.
pub(crate) fn n2747(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'i', s @ ..] => {
			super::push(out, 6834);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6835);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10786);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10787);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kcja.
pub(crate) fn n2748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2691);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', 's', s @ ..] => {
					super::push(out, 10789);
					s.is_empty()
				}
				['r', 'f', 'y', s @ ..] => {
					super::push(out, 10790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['n', 'y', s @ ..] => {
							super::push(out, 13285);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 13286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'o', 'r', 't', s @ ..] => {
					super::push(out, 10791);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 10792);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10793);
					s.is_empty()
				}
				['w', 'n', s @ ..] => {
					super::push(out, 10794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6837);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6838);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13287);
							s.is_empty()
						}
						['o', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14657);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 14658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 't', 's', 'e', s @ ..] => {
					super::push(out, 10795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'r', 'f', s @ ..] => {
			super::push(out, 6839);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ncja.
pub(crate) fn n2749(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6840);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 10796);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 10797);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10798);
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					super::push(out, 10799);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10801);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13288);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 14659);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 15264);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 15449);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 15265);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 14660);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 10802);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 13289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 10803);
					s.is_empty()
				}
				['l', 'a', 'w', 'i', 'w', s @ ..] => {
					super::push(out, 10804);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					super::push(out, 10805);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10806);
					match s {
						[] => true,
						['e', 'h', s @ ..] => {
							super::push(out, 13290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', 't', s @ ..] => {
					super::push(out, 10807);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10808);
					match s {
						[] => true,
						['s', 'y', 'z', 'g', 'e', 'o', s @ ..] => {
							super::push(out, 13291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'l', s @ ..] => {
					super::push(out, 10809);
					s.is_empty()
				}
				['w', 'k', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 14661);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 14662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -komocja.
pub(crate) fn n2750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2692);
	s.is_empty()
}

/// Words ending -pcja.
pub(crate) fn n2751(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'c', s @ ..] => {
			match s {
				[] => true,
				['n', 'o', 'k', 'y', s @ ..] => {
					super::push(out, 10810);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6842);
			s.is_empty()
		}
		['r', 'o', 's', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10812);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', 'o', s @ ..] => {
			super::push(out, 6843);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 6844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcja.
pub(crate) fn n2752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2693);
	match s {
		[] => true,
		['e', 'n', s @ ..] => {
			super::push(out, 6845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucja.
pub(crate) fn n2753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2694);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6846);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6847);
			s.is_empty()
		}
		['t', 'y', 't', 's', 'o', s @ ..] => {
			super::push(out, 6848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ycja.
pub(crate) fn n2754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2695);
	match s {
		[] => true,
		['d', 'n', s @ ..] => {
			super::push(out, 6849);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6850);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['i', 'w', 'k', 'n', s @ ..] => {
					super::push(out, 10814);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10815);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łaszczyzna.
pub(crate) fn n16522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15450);
	s.is_empty()
}

/// Words ending -aniec.
pub(crate) fn n7793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7437);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['e', 'j', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13392);
					s.is_empty()
				}
				['u', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 13393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 11095);
			match s {
				[] => true,
				['e', 'z', 'r', 'p', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 13394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'o', 'c', 'a', 'z', '\u{1}', s @ ..] => {
			super::push(out, 11096);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 11097);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13395);
					match s {
						[] => true,
						['r', 'u', 't', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 14675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11098);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 11099);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13397);
					s.is_empty()
				}
				['s', 'y', 'ł', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13398);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13399);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13400);
					match s {
						[] => true,
						['z', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 14676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'r', 'e', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 13401);
					s.is_empty()
				}
				['z', 's', 'e', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 13402);
					s.is_empty()
				}
				['ą', 'ł', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 13403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					super::push(out, 13404);
					s.is_empty()
				}
				['w', 'o', 'd', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11100);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 11101);
			s.is_empty()
		}
		['p', 'e', 'z', 'r', 't', 'z', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 11102);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13406);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 13407);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 14677);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 13408);
					s.is_empty()
				}
				['e', 'i', 'b', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13409);
					s.is_empty()
				}
				['h', 'c', 'o', 'z', 'c', 'z', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 13410);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 13411);
					s.is_empty()
				}
				['p', 'a', 'p', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13412);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13413);
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 14679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'o', 'n', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 13414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 11103);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 11104);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13415);
					s.is_empty()
				}
				['a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13416);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 14680);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 14681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 11105);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14682);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 14683);
							s.is_empty()
						}
						['k', 'r', 'u', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 14684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 14685);
							s.is_empty()
						}
						['e', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 14686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'o', 'm', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 13417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13418);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13419);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13420);
					match s {
						[] => true,
						['p', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 14687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13421);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14688);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 11106);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 11107);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eniec.
pub(crate) fn n7795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7439);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 11109);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11110);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['e', 'z', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 14690);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14691);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['b', 'o', '\u{1}', s @ ..] => {
											super::push(out, 15451);
											s.is_empty()
										}
										['u', '\u{1}', s @ ..] => {
											super::push(out, 15452);
											s.is_empty()
										}
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
					super::push(out, 13422);
					match s {
						[] => true,
						['d', 'o', '\u{1}', s @ ..] => {
							super::push(out, 14692);
							s.is_empty()
						}
						['e', 'l', 'p', 'ł', 'ó', 'p', 's', 'w', '\u{1}', s @ ..] => {
							super::push(out, 14693);
							s.is_empty()
						}
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13423);
					match s {
						[] => true,
						['a', 'r', 't', 'u', '\u{1}', s @ ..] => {
							super::push(out, 14695);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'z', '\u{1}', s @ ..] => {
									super::push(out, 15271);
									s.is_empty()
								}
								['z', 'c', 'z', 's', 'd', 'o', '\u{1}', s @ ..] => {
									super::push(out, 15272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13424);
					match s {
						[] => true,
						['o', 'r', 'd', 'z', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 15273);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 15274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'd', s @ ..] => {
					match s {
						[] => true,
						['o', 'ł', 'm', '\u{1}', s @ ..] => {
							super::push(out, 14696);
							s.is_empty()
						}
						['ą', s @ ..] => {
							super::push(out, 14697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 11111);
			match s {
				[] => true,
				['e', 'l', 'o', 'k', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 13425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11112);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13426);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 13427);
					s.is_empty()
				}
				['d', 'y', s @ ..] => {
					super::push(out, 13428);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13429);
					s.is_empty()
				}
				['y', 'p', s @ ..] => {
					super::push(out, 13430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 11113);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13431);
					match s {
						[] => true,
						['e', 'z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 14698);
							s.is_empty()
						}
						['o', 'b', 'z', '\u{1}', s @ ..] => {
							super::push(out, 14699);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 14700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 14701);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15275);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 15276);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 14702);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14703);
							s.is_empty()
						}
						['u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 11114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13433);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', 'ł', 'm', '\u{1}', s @ ..] => {
							super::push(out, 14705);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 14706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 13434);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 14707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ziniec.
pub(crate) fn n11786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11119);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13437);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oniec.
pub(crate) fn n7797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7441);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 11120);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13439);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 11121);
			s.is_empty()
		}
		['p', 'a', 'j', '\u{1}', s @ ..] => {
			super::push(out, 11122);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11123);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 13441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 11124);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 13442);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rniec.
pub(crate) fn n7798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7442);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11125);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 13443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'i', s @ ..] => {
			super::push(out, 11126);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word kuniec.
pub(crate) fn n7799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7443);
	s.is_empty()
}

/// Words ending -yniec.
pub(crate) fn n7800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7444);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 11127);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['c', 'u', s @ ..] => {
					super::push(out, 13444);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13445);
					match s {
						[] => true,
						['a', 'b', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -źniec.
pub(crate) fn n7801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7445);
	match s {
		[] => true,
		['i', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 11128);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -piec.
pub(crate) fn n3416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3308);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7446);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7447);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 13446);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 13447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14710);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7448);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7449);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 11130);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 13448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 11131);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 11132);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 13449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7450);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7451);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 11133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 7452);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 7453);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -siec.
pub(crate) fn n3417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3309);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7454);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7455);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7456);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7457);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 11134);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wiec.
pub(crate) fn n3418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3310);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7458);
			s.is_empty()
		}
		['-', 'o', 'm', 'o', 'z', '\u{1}', s @ ..] => {
			super::push(out, 7459);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7460);
			match s {
				[] => true,
				['d', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 11135);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11136);
					match s {
						[] => true,
						['u', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 13450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'u', 'm', 'd', '\u{1}', s @ ..] => {
					super::push(out, 11137);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11138);
					match s {
						[] => true,
						['b', 'u', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 13451);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 13452);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 13453);
							match s {
								[] => true,
								['e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 14711);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 13454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 11139);
					s.is_empty()
				}
				['w', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 11140);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 11141);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 11142);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 13455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'z', 'r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 7461);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7462);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11143);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['d', 'r', 'e', 'i', 'p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 13456);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13457);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 13458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14712);
									s.is_empty()
								}
								['y', 'm', '\u{1}', s @ ..] => {
									super::push(out, 14713);
									s.is_empty()
								}
								['ę', s @ ..] => {
									super::push(out, 14714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11144);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 11145);
					s.is_empty()
				}
				['z', 'd', 'o', 'g', 'e', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 11146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n7819(s, out),
		['r', 'e', 'z', 'c', '\u{1}', s @ ..] => {
			super::push(out, 7464);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7465);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['z', s @ ..] => {
					match s {
						[] => true,
						['r', 'k', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 13560);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14810);
									s.is_empty()
								}
								['ł', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 14811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 11169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ź', s @ ..] => {
			super::push(out, 7466);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ziec.
pub(crate) fn n3419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3311);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7467);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 7468);
			match s {
				[] => true,
				['o', 'r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13561);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 13562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11170);
					s.is_empty()
				}
				['ź', s @ ..] => {
					super::push(out, 11171);
					match s {
						[] => true,
						['e', 'j', '\u{1}', s @ ..] => {
							super::push(out, 13563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iechanie.
pub(crate) fn n14512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13595);
	s.is_empty()
}

/// Words ending -jechanie.
pub(crate) fn n14513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13596);
	s.is_empty()
}

/// Words ending -esłuchanie.
pub(crate) fn n11917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11240);
	s.is_empty()
}

/// Words ending -ychanie.
pub(crate) fn n11918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11241);
	s.is_empty()
}

/// Words ending -ianie.
pub(crate) fn n7947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7579);
	s.is_empty()
}

/// Words ending -janie.
pub(crate) fn n7948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7580);
	s.is_empty()
}

/// Words ending -kanie.
pub(crate) fn n7949(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'd', s @ ..] => {
			super::push(out, 11242);
			s.is_empty()
		}
		['e', 'z', 'r', 'a', s @ ..] => {
			super::push(out, 11243);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11244);
			s.is_empty()
		}
		['t', 'o', s @ ..] => {
			super::push(out, 11245);
			s.is_empty()
		}
		['u', 'ł', s @ ..] => {
			super::push(out, 11246);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 11247);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 11248);
			s.is_empty()
		}
		['ą', 'ł', 'b', 'o', s @ ..] => {
			super::push(out, 11249);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lanie.
pub(crate) fn n7950(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11250);
			s.is_empty()
		}
		['e', 'z', s @ ..] => {
			super::push(out, 11251);
			s.is_empty()
		}
		['g', 'ą', s @ ..] => {
			super::push(out, 11252);
			s.is_empty()
		}
		['t', 'e', 'i', 'w', 'ś', 'a', s @ ..] => {
			super::push(out, 11253);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -manie.
pub(crate) fn n7951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7581);
	s.is_empty()
}

/// Words ending -nanie.
pub(crate) fn n7952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7582);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 11254);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 11255);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 11256);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -panie.
pub(crate) fn n7953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7583);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 11257);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ranie.
pub(crate) fn n7954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7584);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11258);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 13597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 11259);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14831);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11260);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 14833);
							s.is_empty()
						}
						['w', 'a', s @ ..] => {
							super::push(out, 14834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'a', s @ ..] => {
			super::push(out, 11261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sanie.
pub(crate) fn n7955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7585);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11262);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11263);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 13598);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tanie.
pub(crate) fn n7956(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'ł', 'o', s @ ..] => {
			super::push(out, 11264);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 11265);
			match s {
				[] => true,
				['w', 'k', s @ ..] => {
					super::push(out, 13599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'k', 'e', s @ ..] => {
			super::push(out, 11266);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 13600);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 11267);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 11268);
			s.is_empty()
		}
		['ę', 'p', 'o', s @ ..] => {
			super::push(out, 11269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wanie.
pub(crate) fn n7957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7586);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11270);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 13602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 11271);
			s.is_empty()
		}
		['i', 'k', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13603);
					s.is_empty()
				}
				['u', 'z', 's', 'o', 'p', s @ ..] => {
					super::push(out, 13604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11272);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['e', 'z', 'r', 't', 'o', 'p', 'a', s @ ..] => {
							super::push(out, 14835);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 13605);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14837);
							match s {
								[] => true,
								['k', 'r', s @ ..] => {
									super::push(out, 15351);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'k', 'z', s @ ..] => {
							super::push(out, 14838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 13606);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 13607);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							super::push(out, 14839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 'z', 'o', s @ ..] => {
									super::push(out, 15353);
									s.is_empty()
								}
								['p', 'o', s @ ..] => {
									super::push(out, 15354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14840);
							s.is_empty()
						}
						['n', 'a', 'l', s @ ..] => {
							super::push(out, 14841);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 14842);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14843);
							s.is_empty()
						}
						['z', 'c', 'i', 'n', s @ ..] => {
							super::push(out, 14844);
							s.is_empty()
						}
						['ę', 'i', 'z', 'd', 'o', s @ ..] => {
							super::push(out, 14845);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 14846);
							match s {
								[] => true,
								['a', 'c', s @ ..] => {
									super::push(out, 15355);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 15356);
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
						['a', 'p', s @ ..] => {
							super::push(out, 14847);
							s.is_empty()
						}
						['g', 'a', s @ ..] => {
							super::push(out, 14848);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 14849);
							s.is_empty()
						}
						['u', 'g', 'e', 'r', 'u', s @ ..] => {
							super::push(out, 14850);
							s.is_empty()
						}
						['y', 'd', s @ ..] => {
							super::push(out, 14851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 14852);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['s', 'd', s @ ..] => {
									super::push(out, 15357);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 15358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'r', s @ ..] => {
							super::push(out, 14853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13608);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 14854);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 14855);
							s.is_empty()
						}
						['o', 'z', s @ ..] => {
							super::push(out, 14856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13609);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 14857);
							s.is_empty()
						}
						['u', 'r', s @ ..] => {
							super::push(out, 14858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', 'c', 'z', s @ ..] => {
							super::push(out, 14859);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'k', 's', s @ ..] => {
									super::push(out, 15359);
									s.is_empty()
								}
								['m', 'u', s @ ..] => {
									super::push(out, 15360);
									s.is_empty()
								}
								['t', 'a', s @ ..] => {
									super::push(out, 15361);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 15362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14860);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14862);
							s.is_empty()
						}
						['e', 'r', 'e', 't', 'n', 'i', 'a', s @ ..] => {
							super::push(out, 14863);
							s.is_empty()
						}
						['n', 'a', 'n', s @ ..] => {
							super::push(out, 14864);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'w', s @ ..] => {
							super::push(out, 14866);
							s.is_empty()
						}
						['n', 'e', 'c', 'o', s @ ..] => {
							super::push(out, 14867);
							s.is_empty()
						}
						['o', 'g', s @ ..] => {
							super::push(out, 14868);
							match s {
								[] => true,
								['y', 'z', 'r', 'p', 'e', s @ ..] => {
									super::push(out, 15363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'o', 'r', 'p', 's', s @ ..] => {
							super::push(out, 14869);
							s.is_empty()
						}
						['u', 'z', s @ ..] => {
							super::push(out, 14870);
							s.is_empty()
						}
						['y', 'z', 'c', s @ ..] => {
							super::push(out, 14871);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 13610);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14873);
							match s {
								[] => true,
								['i', 'b', 'o', s @ ..] => {
									super::push(out, 15364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14874);
							s.is_empty()
						}
						['o', 'n', s @ ..] => {
							super::push(out, 14875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', 'u', s @ ..] => {
							super::push(out, 14876);
							s.is_empty()
						}
						['i', 'm', 'a', s @ ..] => {
							super::push(out, 14877);
							s.is_empty()
						}
						['u', 'm', 'r', 'o', 'f', 's', s @ ..] => {
							super::push(out, 14878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', 'y', 'z', 'r', 'k', 's', s @ ..] => {
					super::push(out, 13611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', s @ ..] => {
			super::push(out, 11273);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 11274);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 11275);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					super::push(out, 13612);
					s.is_empty()
				}
				['ł', 'a', s @ ..] => {
					super::push(out, 13613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 11276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zanie.
pub(crate) fn n7958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7587);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11277);
			match s {
				[] => true,
				['k', 's', 'w', s @ ..] => {
					super::push(out, 13614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 11278);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 11279);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 11280);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11281);
			match s {
				[] => true,
				['a', 'd', s @ ..] => {
					super::push(out, 13615);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 14879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'z', s @ ..] => {
					super::push(out, 13616);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'e', 'i', 'm', 'a', s @ ..] => {
			super::push(out, 11282);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 11283);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łanie.
pub(crate) fn n7959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7588);
	s.is_empty()
}

/// Words ending -ważanie.
pub(crate) fn n7960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7589);
	s.is_empty()
}

/// Words ending -dnie.
pub(crate) fn n3558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3443);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 7590);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7591);
			match s {
				[] => true,
				['ł', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11284);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -enie.
pub(crate) fn n3559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3444);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7592);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11285);
					match s {
						[] => true,
						['i', 'n', 'g', s @ ..] => {
							super::push(out, 13617);
							s.is_empty()
						}
						['l', 'z', s @ ..] => {
							super::push(out, 13618);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 14881);
									match s {
										[] => true,
										['a', 'w', 'o', 'm', 'u', '\u{1}', s @ ..] => {
											super::push(out, 15365);
											s.is_empty()
										}
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
					super::push(out, 11286);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 11287);
					s.is_empty()
				}
				['ł', 'a', 't', 'z', 's', 'k', 'o', s @ ..] => {
					super::push(out, 11288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7593);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11289);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13619);
							s.is_empty()
						}
						['e', 'i', s @ ..] => {
							super::push(out, 13620);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 14882);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 14883);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 14884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 13621);
							match s {
								[] => true,
								['z', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 14885);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ę', s @ ..] => {
							super::push(out, 13622);
							match s {
								[] => true,
								['z', 'u', s @ ..] => {
									super::push(out, 14886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 11290);
					s.is_empty()
				}
				['m', 'u', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 13623);
							s.is_empty()
						}
						['z', 'o', 'r', 'o', s @ ..] => {
							super::push(out, 13624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 11291);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13625);
							s.is_empty()
						}
						['d', 'u', 'l', s @ ..] => {
							super::push(out, 13626);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13627);
							match s {
								[] => true,
								['i', 'w', 'r', 'e', 'z', 'c', 'a', s @ ..] => {
									super::push(out, 14887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 13628);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 14888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 13629);
							match s {
								[] => true,
								['c', 't', s @ ..] => {
									super::push(out, 14889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'k', s @ ..] => {
							super::push(out, 13630);
							s.is_empty()
						}
						['k', 'y', s @ ..] => {
							super::push(out, 13631);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13632);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13633);
							s.is_empty()
						}
						['t', 'o', s @ ..] => {
							super::push(out, 13634);
							s.is_empty()
						}
						['z', 'c', 'ę', s @ ..] => {
							super::push(out, 13635);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 13636);
							match s {
								[] => true,
								['e', 'p', 's', s @ ..] => {
									super::push(out, 14890);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ś', 'i', s @ ..] => {
							super::push(out, 13637);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 14891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 11292);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 13638);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 14892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ę', 't', 'o', s @ ..] => {
							super::push(out, 13639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 11293);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 14893);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 14894);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 14895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'ł', s @ ..] => {
							super::push(out, 13640);
							s.is_empty()
						}
						['r', 'k', 'o', s @ ..] => {
							super::push(out, 13641);
							s.is_empty()
						}
						['ó', s @ ..] => {
							super::push(out, 13642);
							match s {
								[] => true,
								['m', 'o', 'd', s @ ..] => {
									super::push(out, 14896);
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
						['d', 'a', s @ ..] => {
							super::push(out, 13643);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13644);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 13645);
							match s {
								[] => true,
								['r', 'g', 'u', s @ ..] => {
									super::push(out, 14897);
									s.is_empty()
								}
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
			super::push(out, 7594);
			match s {
				[] => true,
				['a', 'z', 'c', 'y', s @ ..] => {
					super::push(out, 11294);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 13646);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 14898);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 14899);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 13647);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['o', 'ł', s @ ..] => {
									super::push(out, 14900);
									s.is_empty()
								}
								['s', 'o', s @ ..] => {
									super::push(out, 14901);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 13648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 11295);
					match s {
						[] => true,
						['i', 'c', 'w', s @ ..] => {
							super::push(out, 13649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13650);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 13651);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['o', 'd', 'a', 'z', 'e', s @ ..] => {
									super::push(out, 14902);
									s.is_empty()
								}
								['z', s @ ..] => super::t4::n15950(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t4::n11980(s, out),
				['u', 't', s @ ..] => super::t4::n11981(s, out),
				['ś', 'e', s @ ..] => super::t4::n11982(s, out),
				_ => false,
			}
		}
		['z', s @ ..] => super::t4::n7967(s, out),
		['ż', s @ ..] => super::t4::n7968(s, out),
		_ => false,
	}
}

/// Words ending -jnie.
pub(crate) fn n3560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3445);
	match s {
		[] => true,
		['o', 'w', 'y', s @ ..] => {
			super::push(out, 7596);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onie.
pub(crate) fn n3561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3446);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 7597);
			s.is_empty()
		}
		['r', 't', s @ ..] => {
			super::push(out, 7598);
			s.is_empty()
		}
		['ł', 'b', '\u{1}', s @ ..] => {
			super::push(out, 7599);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ynie.
pub(crate) fn n3562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3447);
	s.is_empty()
}

/// Words ending -śnie.
pub(crate) fn n3563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3448);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 7600);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -aniak.
pub(crate) fn n8247(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 11419);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13719);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14926);
							s.is_empty()
						}
						['t', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 14927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'z', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 11420);
			s.is_empty()
		}
		['w', 'c', '\u{1}', s @ ..] => {
			super::push(out, 11421);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bniak.
pub(crate) fn n8248(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11422);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11423);
			s.is_empty()
		}
		['u', 'l', 'ś', '\u{1}', s @ ..] => {
			super::push(out, 11424);
			s.is_empty()
		}
		['ę', 'd', '\u{1}', s @ ..] => {
			super::push(out, 11425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dniak.
pub(crate) fn n8249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7852);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11426);
			match s {
				[] => true,
				['r', 'ś', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'z', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 13722);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 13723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', 'm', '\u{1}', s @ ..] => {
					super::push(out, 13724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 11427);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eniak.
pub(crate) fn n8250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7853);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 11428);
			s.is_empty()
		}
		['l', 'e', 'i', 'z', '\u{1}', s @ ..] => {
			super::push(out, 11429);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11430);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 11431);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gniak.
pub(crate) fn n8251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7854);
	s.is_empty()
}

/// Words ending -iniak.
pub(crate) fn n8252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7855);
	match s {
		[] => true,
		['f', 'l', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 11432);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11433);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'r', 'a', 'i', 'z', '\u{1}', s @ ..] => {
			super::push(out, 11434);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11435);
			s.is_empty()
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13726);
					s.is_empty()
				}
				['ś', '\u{1}', s @ ..] => {
					super::push(out, 13727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jniak.
pub(crate) fn n8253(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 't', '\u{1}', s @ ..] => {
			super::push(out, 11436);
			s.is_empty()
		}
		['o', 't', 's', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 11437);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13728);
					match s {
						[] => true,
						['k', 'u', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 11438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kniak.
pub(crate) fn n8254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7856);
	s.is_empty()
}

/// Words ending -lniak.
pub(crate) fn n8255(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 13730);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 13731);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 14930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 13732);
					s.is_empty()
				}
				['t', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 13733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11439);
			match s {
				[] => true,
				['w', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'k', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 11440);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 11441);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mniak.
pub(crate) fn n8256(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11442);
			match s {
				[] => true,
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 13735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 11443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oniak.
pub(crate) fn n8257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7857);
	match s {
		[] => true,
		['g', 'o', '\u{1}', s @ ..] => {
			super::push(out, 11444);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 11445);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 11446);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13736);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 13737);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 13738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', '\u{1}', s @ ..] => {
			super::push(out, 11447);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 11448);
			s.is_empty()
		}
		['ł', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13739);
					s.is_empty()
				}
				['b', 'y', 'z', 'd', 'ę', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 13740);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13741);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pniak.
pub(crate) fn n8258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7858);
	match s {
		[] => true,
		['a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 11449);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 11450);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 13742);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 13743);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rniak.
pub(crate) fn n8259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7859);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11451);
			match s {
				[] => true,
				['i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 13744);
					s.is_empty()
				}
				['l', 'u', 'p', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13745);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 13746);
					s.is_empty()
				}
				['z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 13747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13748);
					s.is_empty()
				}
				['w', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 13749);
					s.is_empty()
				}
				['z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 13750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11452);
			match s {
				[] => true,
				['i', 'p', 'y', 'z', 'c', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 13751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 13752);
					s.is_empty()
				}
				['k', 's', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 13753);
					s.is_empty()
				}
				['w', 'z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 13754);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tniak.
pub(crate) fn n8260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7860);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11453);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 13755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 11454);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 13756);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13757);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 13758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 11455);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 11456);
			match s {
				[] => true,
				['i', 'c', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13759);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wniak.
pub(crate) fn n8261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7861);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 13760);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 14931);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 14932);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 11457);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['t', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 13761);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 11458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yniak.
pub(crate) fn n8262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7862);
	match s {
		[] => true,
		['z', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 13763);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', 'o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 11459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zniak.
pub(crate) fn n8263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7863);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13765);
					match s {
						[] => true,
						['i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 14933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', 'a', 'r', 'g', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 13766);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13767);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 13768);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word wełniak.
pub(crate) fn n8264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7864);
	s.is_empty()
}

/// Words ending -śniak.
pub(crate) fn n8265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7865);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 11460);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13769);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 11461);
			s.is_empty()
		}
		['ę', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 11462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -źniak.
pub(crate) fn n8266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7866);
	match s {
		[] => true,
		['i', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 11463);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żniak.
pub(crate) fn n8267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7867);
	match s {
		[] => true,
		['a', 'w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13771);
					s.is_empty()
				}
				['d', 'o', '\u{1}', s @ ..] => {
					super::push(out, 13772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 11464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -piak.
pub(crate) fn n3926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3794);
	s.is_empty()
}

/// Words ending -riak.
pub(crate) fn n3927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3795);
	match s {
		[] => true,
		['e', 't', '\u{1}', s @ ..] => {
			super::push(out, 7868);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -siak.
pub(crate) fn n3928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3796);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 7870);
			s.is_empty()
		}
		['n', 'o', 'f', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 7871);
			s.is_empty()
		}
		['o', 'r', s @ ..] => {
			super::push(out, 7872);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7873);
			match s {
				[] => true,
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 11465);
					s.is_empty()
				}
				['r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 11466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 7874);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wiak.
pub(crate) fn n3929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3797);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7875);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 11467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 's', 'u', 'ł', s @ ..] => {
			super::push(out, 7876);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['k', 's', s @ ..] => {
					super::push(out, 11468);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11469);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 13773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 11470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7877);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ziak.
pub(crate) fn n3930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3798);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7878);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11471);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11472);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 11473);
					match s {
						[] => true,
						['ł', s @ ..] => {
							super::push(out, 13774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11474);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 11475);
					s.is_empty()
				}
				['ź', s @ ..] => {
					super::push(out, 11476);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 13775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7879);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7880);
			match s {
				[] => true,
				['b', 'o', s @ ..] => {
					super::push(out, 11477);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ynek.
pub(crate) fn n4110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3968);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8150);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 8151);
			match s {
				[] => true,
				['e', 'j', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11637);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11638);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11639);
					match s {
						[] => true,
						['o', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 13822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 11640);
					s.is_empty()
				}
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 11641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8152);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11642);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11643);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 11644);
					match s {
						[] => true,
						['n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 13823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11646);
					s.is_empty()
				}
				['n', 'i', 'm', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11647);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 11648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8154);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 8155);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11649);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11650);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 13824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 11651);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 13825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 8156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urek.
pub(crate) fn n4128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3986);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 8207);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 8208);
			s.is_empty()
		}
		['d', 'n', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 8209);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 8210);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8211);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 8212);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11694);
					s.is_empty()
				}
				['d', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11696);
					s.is_empty()
				}
				['y', 'z', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 13837);
							match s {
								[] => true,
								['z', 's', 'b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 14942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8214);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11697);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 11698);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 11699);
					match s {
						[] => true,
						['b', 'm', 'a', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 13839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 11700);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 11701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8215);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 8216);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					super::push(out, 11703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 8217);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11704);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11705);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hórek.
pub(crate) fn n8640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8219);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 11706);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kórek.
pub(crate) fn n8642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8221);
	match s {
		[] => true,
		['s', 'a', s @ ..] => {
			super::push(out, 11707);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kutek.
pub(crate) fn n8699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8275);
	match s {
		[] => true,
		['s', '\u{1}', s @ ..] => {
			super::push(out, 11750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lutek.
pub(crate) fn n8700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8276);
	s.is_empty()
}

/// Words ending -mutek.
pub(crate) fn n8701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8277);
	match s {
		[] => true,
		['s', '\u{1}', s @ ..] => {
			super::push(out, 11751);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sutek.
pub(crate) fn n8702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8278);
	s.is_empty()
}

/// Words ending -zutek.
pub(crate) fn n8703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8279);
	match s {
		[] => true,
		['c', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 11752);
			s.is_empty()
		}
		['r', '\u{1}', s @ ..] => {
			super::push(out, 11753);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ytek.
pub(crate) fn n4148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4006);
	match s {
		[] => true,
		['b', 'z', s @ ..] => {
			super::push(out, 8280);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 8281);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 8282);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ztek.
pub(crate) fn n4149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4007);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11755);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ątek.
pub(crate) fn n4150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4008);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 8284);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 11756);
					match s {
						[] => true,
						['i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 13853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 11757);
					match s {
						[] => true,
						['ś', '\u{1}', s @ ..] => {
							super::push(out, 13854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', 'a', s @ ..] => {
			super::push(out, 8285);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 8286);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 8287);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 8288);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 13855);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 13856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 8289);
			match s {
				[] => true,
				['r', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 11759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', 'u', '\u{1}', s @ ..] => {
			super::push(out, 8290);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ętek.
pub(crate) fn n4151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4009);
	match s {
		[] => true,
		['r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 8291);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word żółtek.
pub(crate) fn n4152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4010);
	s.is_empty()
}

/// Words ending -aczek.
pub(crate) fn n8727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8303);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11764);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 13857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 11765);
			match s {
				[] => true,
				['e', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13858);
					s.is_empty()
				}
				['o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 13859);
					s.is_empty()
				}
				['r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 13860);
					s.is_empty()
				}
				['u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 13861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', '\u{1}', s @ ..] => {
			super::push(out, 11766);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13862);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13863);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13864);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14949);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 15369);
									s.is_empty()
								}
								['w', 'c', '\u{1}', s @ ..] => {
									super::push(out, 15370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 14950);
							s.is_empty()
						}
						['d', 'o', 'h', 'c', 'z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14951);
							s.is_empty()
						}
						['e', 'z', s @ ..] => {
							super::push(out, 14952);
							s.is_empty()
						}
						['m', 'e', 'j', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14953);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 14954);
							match s {
								[] => true,
								['u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 15371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ź', s @ ..] => {
							super::push(out, 14955);
							match s {
								[] => true,
								['i', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 15372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13865);
					s.is_empty()
				}
				['w', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 13866);
					s.is_empty()
				}
				['z', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 11767);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 11768);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13869);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 13870);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13871);
					s.is_empty()
				}
				['r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13872);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11769);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13874);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 13875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11770);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13876);
					match s {
						[] => true,
						['p', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 14956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13877);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13878);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 14957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 11771);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13879);
					match s {
						[] => true,
						['ł', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 14958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 11772);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 14959);
							s.is_empty()
						}
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 14960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 13880);
					s.is_empty()
				}
				['d', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 13881);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 13882);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13883);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14961);
							match s {
								[] => true,
								['e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 15373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'a', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 13884);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13885);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 14962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 11773);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 11774);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 11775);
			match s {
				[] => true,
				['i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 13887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 11776);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13888);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 13889);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 14963);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 14964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 13890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 11777);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 13891);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13892);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -deczek.
pub(crate) fn n12505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11779);
	s.is_empty()
}

/// Words ending -ieczek.
pub(crate) fn n12506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11780);
	match s {
		[] => true,
		['w', 'o', 'ł', 'z', s @ ..] => {
			super::push(out, 13894);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -meczek.
pub(crate) fn n12507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11781);
	match s {
		[] => true,
		['a', 'z', '\u{1}', s @ ..] => {
			super::push(out, 13895);
			s.is_empty()
		}
		['o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 13896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -neczek.
pub(crate) fn n12508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11782);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 13897);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13898);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14967);
					match s {
						[] => true,
						['w', 'o', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 15374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 13899);
			match s {
				[] => true,
				['d', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14968);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 14969);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reczek.
pub(crate) fn n12509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11783);
	match s {
		[] => true,
		['a', 'n', s @ ..] => {
			super::push(out, 13900);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 14971);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -seczek.
pub(crate) fn n12510(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13901);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14972);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 15376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13902);
			match s {
				[] => true,
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 14973);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -teczek.
pub(crate) fn n12511(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 'k', s @ ..] => {
			super::push(out, 13903);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 13904);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zeczek.
pub(crate) fn n12512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11784);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 13905);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łeczek.
pub(crate) fn n12513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11785);
	match s {
		[] => true,
		['o', 'i', s @ ..] => {
			super::push(out, 13906);
			match s {
				[] => true,
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 14975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iczek.
pub(crate) fn n8729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8305);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 11786);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 11787);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14976);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 14977);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 15378);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 13907);
					match s {
						[] => true,
						['r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11788);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['d', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 13908);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 13909);
					s.is_empty()
				}
				['j', 'ó', 'b', 'z', '\u{1}', s @ ..] => {
					super::push(out, 13910);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13911);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					super::push(out, 13912);
					s.is_empty()
				}
				['n', 'i', s @ ..] => {
					super::push(out, 13913);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13914);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 14979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13915);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'c', s @ ..] => {
									super::push(out, 15379);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 15380);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15463);
											s.is_empty()
										}
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
					super::push(out, 13916);
					match s {
						[] => true,
						['o', 'h', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 14980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 13917);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 13918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 11789);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 11790);
			match s {
				[] => true,
				['u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 13920);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lczek.
pub(crate) fn n8730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8306);
	match s {
		[] => true,
		['i', 'm', s @ ..] => {
			super::push(out, 11791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oczek.
pub(crate) fn n8731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8307);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11792);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 11793);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13922);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13923);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14981);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 14982);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14983);
							match s {
								[] => true,
								['z', 's', 'y', 'm', '\u{1}', s @ ..] => {
									super::push(out, 15381);
									s.is_empty()
								}
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
			super::push(out, 11794);
			s.is_empty()
		}
		['m', 'y', 'w', '\u{1}', s @ ..] => {
			super::push(out, 11795);
			s.is_empty()
		}
		['n', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 11796);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 11797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13924);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 11798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rczek.
pub(crate) fn n8732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8308);
	match s {
		[] => true,
		['a', 'i', s @ ..] => {
			super::push(out, 11799);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 13926);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uczek.
pub(crate) fn n8733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8309);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11800);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 11801);
			s.is_empty()
		}
		['o', 'm', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 11802);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 13927);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', 't', '\u{1}', s @ ..] => {
			super::push(out, 11803);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 11804);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yczek.
pub(crate) fn n8734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8310);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 11805);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 11806);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 13929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 11807);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 11808);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'z', 's', s @ ..] => {
			super::push(out, 11809);
			match s {
				[] => true,
				['f', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zczek.
pub(crate) fn n8735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8311);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13932);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13933);
					match s {
						[] => true,
						['i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 14986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 13934);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ączek.
pub(crate) fn n8736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8312);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 11810);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 11811);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 14987);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 11812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ozek.
pub(crate) fn n4161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4017);
	match s {
		[] => true,
		['r', 'm', '\u{1}', s @ ..] => {
			super::push(out, 8313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rzek.
pub(crate) fn n4162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4018);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 8314);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 11813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8315);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -szek.
pub(crate) fn n4163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4019);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8316);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11814);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 11815);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'ę', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 13935);
							s.is_empty()
						}
						['m', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 13936);
							s.is_empty()
						}
						['n', 'e', 'i', 'z', 'd', 'o', 'ł', 'm', '\u{1}', s @ ..] => {
							super::push(out, 13937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 11816);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11817);
					match s {
						[] => true,
						['a', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13938);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 13939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11818);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11819);
					match s {
						[] => true,
						['s', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 13940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 11820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8317);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', 'z', 'e', 'z', 'r', '\u{1}', s @ ..] => {
									super::push(out, 14989);
									s.is_empty()
								}
								['ś', '\u{1}', s @ ..] => {
									super::push(out, 14990);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 13941);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14991);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11821);
					s.is_empty()
				}
				['z', 'r', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 13942);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8318);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					super::push(out, 11822);
					match s {
						[] => true,
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 13944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11823);
					match s {
						[] => true,
						['e', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 13945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'm', '\u{1}', s @ ..] => {
					super::push(out, 11824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8319);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11825);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'ś', s @ ..] => {
							match s {
								[] => true,
								['u', 'ł', 't', '\u{1}', s @ ..] => {
									super::push(out, 14992);
									s.is_empty()
								}
								['y', 'z', 'c', '\u{1}', s @ ..] => {
									super::push(out, 14993);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 13946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11826);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 13947);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14994);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 14995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11827);
					match s {
						[] => true,
						['ę', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 13948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 11828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8320);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11829);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 11830);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13949);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13950);
							match s {
								[] => true,
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 14996);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 13951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 11831);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 11832);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11833);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14997);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 14998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 14999);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15000);
									s.is_empty()
								}
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 15001);
									s.is_empty()
								}
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 15002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11834);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 13952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 11835);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11836);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 11837);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 11838);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11839);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11840);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 13955);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 11841);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 11842);
					match s {
						[] => true,
						['o', 'm', 'c', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 13956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 8321);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 11843);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 11844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ązek.
pub(crate) fn n4164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4020);
	match s {
		[] => true,
		['i', 'w', s @ ..] => {
			match s {
				[] => true,
				['o', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 11845);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 11846);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ogrodnik.
pub(crate) fn n16060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15012);
	s.is_empty()
}

/// The word winogrodnik.
pub(crate) fn n16061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15013);
	s.is_empty()
}

/// The word przyrodnik.
pub(crate) fn n14953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13995);
	s.is_empty()
}

/// Words ending -wodnik.
pub(crate) fn n12684(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13996);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 13997);
			s.is_empty()
		}
		['z', 'o', s @ ..] => {
			super::push(out, 13998);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -płodnik.
pub(crate) fn n12685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11942);
	s.is_empty()
}

/// The word akordnik.
pub(crate) fn n8890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8456);
	s.is_empty()
}

/// Words ending -udnik.
pub(crate) fn n8891(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', '\u{1}', s @ ..] => {
			super::push(out, 11943);
			s.is_empty()
		}
		['ł', 'b', s @ ..] => {
			super::push(out, 11944);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word bezwstydnik.
pub(crate) fn n8892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8457);
	s.is_empty()
}

/// Words ending -ądnik.
pub(crate) fn n8893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8458);
	s.is_empty()
}

/// Words ending -ędnik.
pub(crate) fn n8894(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 11945);
			s.is_empty()
		}
		['z', 'r', 'u', s @ ..] => {
			super::push(out, 11946);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -enik.
pub(crate) fn n4238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4089);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 8459);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8460);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 8461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gnik.
pub(crate) fn n4239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4090);
	s.is_empty()
}

/// Words ending -hnik.
pub(crate) fn n4240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4091);
	match s {
		[] => true,
		['c', 'e', 'z', s @ ..] => {
			super::push(out, 8462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inik.
pub(crate) fn n4241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4092);
	s.is_empty()
}

/// Words ending -jnik.
pub(crate) fn n4242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4093);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8463);
			s.is_empty()
		}
		['i', 'p', s @ ..] => {
			super::push(out, 8464);
			match s {
				[] => true,
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 11947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 11948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'o', s @ ..] => {
					super::push(out, 11949);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 14000);
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
				['t', 's', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 11950);
					s.is_empty()
				}
				['z', 's', 'o', s @ ..] => {
					super::push(out, 11951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 8465);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11952);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 11953);
					s.is_empty()
				}
				['r', 't', '\u{1}', s @ ..] => {
					super::push(out, 11954);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pyknik.
pub(crate) fn n4243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4094);
	s.is_empty()
}

/// Words ending -lnik.
pub(crate) fn n4244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4095);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8466);
			match s {
				[] => true,
				['g', 'a', 'ł', 'b', '\u{1}', s @ ..] => {
					super::push(out, 11955);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 11956);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 14001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11957);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'r', 'd', s @ ..] => {
					super::push(out, 11958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 11959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8467);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 15014);
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									super::push(out, 15015);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14004);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15016);
									s.is_empty()
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15384);
											s.is_empty()
										}
										['z', 'o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 15385);
											s.is_empty()
										}
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
					super::push(out, 11960);
					match s {
						[] => true,
						['r', 'e', 'i', 'm', 'ś', 'e', s @ ..] => {
							super::push(out, 14005);
							match s {
								[] => true,
								['i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 15017);
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
						['c', 'z', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14006);
							s.is_empty()
						}
						['s', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 14007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8468);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8469);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11961);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 8470);
			s.is_empty()
		}
		['y', 'b', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 8471);
			s.is_empty()
		}
		['ó', 'p', 's', 'w', '\u{1}', s @ ..] => {
			super::push(out, 8472);
			s.is_empty()
		}
		['ś', 'e', s @ ..] => {
			match s {
				[] => true,
				['i', 'm', 'e', 'z', 'r', '\u{1}', s @ ..] => {
					super::push(out, 11963);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11964);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mnik.
pub(crate) fn n4245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4096);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8473);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8474);
			match s {
				[] => true,
				['j', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14008);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 11965);
					s.is_empty()
				}
				['z', 'c', 'k', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 11966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8475);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8476);
			s.is_empty()
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 8477);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nnik.
pub(crate) fn n4246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4097);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 11967);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 11968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8478);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', 'ó', 'ł', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14010);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 14011);
							s.is_empty()
						}
						['k', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 14012);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 15018);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 15019);
									s.is_empty()
								}
								['ś', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 14013);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 15021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14014);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 15022);
									s.is_empty()
								}
								['w', 'z', '\u{1}', s @ ..] => {
									super::push(out, 15023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'c', s @ ..] => {
					super::push(out, 11970);
					match s {
						[] => true,
						['ę', 'm', '\u{1}', s @ ..] => {
							super::push(out, 14015);
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
				['c', s @ ..] => {
					super::push(out, 11971);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 11972);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14016);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14017);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8479);
			match s {
				[] => true,
				['g', 'z', '\u{1}', s @ ..] => {
					super::push(out, 11973);
					s.is_empty()
				}
				['k', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 11974);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11975);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 14019);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 14020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 11976);
					match s {
						[] => true,
						['z', 'd', '\u{1}', s @ ..] => {
							super::push(out, 14021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 11977);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14022);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'p', s @ ..] => {
			super::push(out, 8480);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 8481);
			match s {
				[] => true,
				['t', 'ą', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 11978);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -onik.
pub(crate) fn n4247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4098);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 8482);
			match s {
				[] => true,
				['r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 11979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 8483);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 8484);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 11980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 11982);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 14025);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 15024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11983);
					match s {
						[] => true,
						['s', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8485);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11984);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 11985);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11986);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 11987);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 14027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 11988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8486);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					super::push(out, 11989);
					s.is_empty()
				}
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 11990);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 11991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8487);
			match s {
				[] => true,
				['r', 'a', 'h', 'l', 'i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 11992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8488);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 11993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8489);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8490);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11994);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 11995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8491);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11996);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14028);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 8492);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pnik.
pub(crate) fn n4248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4099);
	match s {
		[] => true,
		['a', 'z', 'c', '\u{1}', s @ ..] => {
			super::push(out, 8493);
			s.is_empty()
		}
		['e', 'z', 'c', 'z', 's', 'o', '\u{1}', s @ ..] => {
			super::push(out, 8494);
			s.is_empty()
		}
		['o', 'l', 'r', 'u', '\u{1}', s @ ..] => {
			super::push(out, 8495);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['k', 'o', '\u{1}', s @ ..] => {
					super::push(out, 11997);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 11998);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 14030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 11999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rnik.
pub(crate) fn n4249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4100);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12000);
					match s {
						[] => true,
						['u', 'k', 'o', '\u{1}', s @ ..] => {
							super::push(out, 14031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', s @ ..] => {
					super::push(out, 12001);
					s.is_empty()
				}
				['w', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 12002);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 12003);
					s.is_empty()
				}
				['ż', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8496);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12005);
					match s {
						[] => true,
						['n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12006);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14033);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 14034);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15025);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 15026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 14035);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14036);
							match s {
								[] => true,
								['-', 'a', 'i', 'z', 'd', 'i', 'z', 'd', '\u{1}', s @ ..] => {
									super::push(out, 15027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'o', s @ ..] => {
							super::push(out, 14037);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 15028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 14038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12007);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12008);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12009);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12010);
					match s {
						[] => true,
						['s', 'o', s @ ..] => {
							super::push(out, 14040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'l', s @ ..] => {
			super::push(out, 8497);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8498);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12011);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							super::push(out, 14041);
							match s {
								[] => true,
								['i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 15029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 12012);
					match s {
						[] => true,
						['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12013);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12014);
					s.is_empty()
				}
				['p', 'o', s @ ..] => {
					super::push(out, 12015);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 14044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 12016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8499);
			match s {
				[] => true,
				['f', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 12017);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 12018);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12019);
					match s {
						[] => true,
						['n', 'a', 'w', 'a', '\u{1}', s @ ..] => {
							super::push(out, 14045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 12020);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							super::push(out, 14046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 12021);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14047);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 's', 'o', s @ ..] => {
					super::push(out, 12022);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tnik.
pub(crate) fn n4250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4101);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8500);
			match s {
				[] => true,
				['b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 12023);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 12024);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 12025);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12026);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12027);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12028);
					s.is_empty()
				}
				['ł', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12030);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12031);
					s.is_empty()
				}
				['r', 'k', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 12032);
					s.is_empty()
				}
				['w', 'r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 12033);
					s.is_empty()
				}
				['z', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 12034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12035);
					s.is_empty()
				}
				['k', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 12036);
					s.is_empty()
				}
				['m', 'a', 's', 'k', 'a', '\u{1}', s @ ..] => {
					super::push(out, 12037);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 12038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', 'ż', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 8501);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8502);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12039);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14049);
							s.is_empty()
						}
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 12040);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15030);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 15031);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 15032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'm', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12041);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 12042);
					match s {
						[] => true,
						['e', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14051);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14052);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 14053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12043);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12044);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14054);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 15033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12045);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12046);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12047);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 14056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'y', 'ż', '\u{1}', s @ ..] => {
					super::push(out, 12048);
					s.is_empty()
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 14057);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8503);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 14059);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14060);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 14061);
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
				['a', 'p', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 12049);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12050);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14062);
							s.is_empty()
						}
						['w', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 15034);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 15035);
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
						['\u{1}', s @ ..] => {
							super::push(out, 14063);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15036);
									match s {
										[] => true,
										['k', '\u{1}', s @ ..] => {
											super::push(out, 15386);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 15037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ę', 'g', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 12051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8504);
			match s {
				[] => true,
				['n', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 12052);
					s.is_empty()
				}
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 12053);
					s.is_empty()
				}
				['z', 'r', '\u{1}', s @ ..] => {
					super::push(out, 12054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 8505);
			match s {
				[] => true,
				['m', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12055);
					s.is_empty()
				}
				['r', 'o', 'w', 'e', 'z', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 12056);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 12057);
					s.is_empty()
				}
				['ż', 'o', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', 'ł', 'k', '\u{1}', s @ ..] => {
			super::push(out, 8506);
			s.is_empty()
		}
		['ą', s @ ..] => {
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 12059);
					s.is_empty()
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 8507);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14064);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14065);
							match s {
								[] => true,
								['r', 'e', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 15038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 14066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 12061);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 14067);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -unik.
pub(crate) fn n4251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4102);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8508);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wnik.
pub(crate) fn n4252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4103);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8509);
			match s {
				[] => true,
				['r', 'p', s @ ..] => {
					super::push(out, 12062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14069);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 14070);
							s.is_empty()
						}
						['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 12063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8510);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12064);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12065);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 14072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8511);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12066);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14073);
							s.is_empty()
						}
						['o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 14074);
							s.is_empty()
						}
						['r', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 14075);
							s.is_empty()
						}
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 14076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14077);
							s.is_empty()
						}
						['i', 'k', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 14078);
							s.is_empty()
						}
						['s', 'j', 'e', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 14079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14080);
							s.is_empty()
						}
						['o', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 14081);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14082);
							match s {
								[] => true,
								['a', 'i', 'n', 'g', '\u{1}', s @ ..] => {
									super::push(out, 15039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ą', 'l', '\u{1}', s @ ..] => {
							super::push(out, 14083);
							s.is_empty()
						}
						['ę', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 14084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 'u', 'w', 'd', '\u{1}', s @ ..] => {
					super::push(out, 12067);
					s.is_empty()
				}
				['j', 'o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14085);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 14086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 14087);
							s.is_empty()
						}
						['ł', s @ ..] => {
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 15040);
									s.is_empty()
								}
								['ó', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12068);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14089);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 14090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 12069);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14091);
							match s {
								[] => true,
								['z', 'c', '\u{1}', s @ ..] => {
									super::push(out, 15042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 14092);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14093);
							match s {
								[] => true,
								['i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 15043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 's', 'i', '\u{1}', s @ ..] => {
							super::push(out, 14094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', s @ ..] => {
					super::push(out, 12070);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12071);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14095);
							s.is_empty()
						}
						['n', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14096);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 15044);
									s.is_empty()
								}
								['u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 15045);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14097);
							s.is_empty()
						}
						['y', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14098);
							s.is_empty()
						}
						['ł', 'a', 'w', 'g', '\u{1}', s @ ..] => {
							super::push(out, 14099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'e', 'ż', '\u{1}', s @ ..] => {
							super::push(out, 14100);
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', 'z', 'r', '\u{1}', s @ ..] => {
									super::push(out, 15046);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15047);
									match s {
										[] => true,
										['a', 'k', s @ ..] => {
											super::push(out, 15387);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 14101);
							s.is_empty()
						}
						['s', 'n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14103);
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 15048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', 'ą', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 12072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8512);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 12073);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['r', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12075);
					s.is_empty()
				}
				['z', 'r', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14105);
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 14106);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 14107);
							s.is_empty()
						}
						['z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12077);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14109);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 14110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ynik.
pub(crate) fn n4253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4104);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8513);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 8514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12078);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -znik.
pub(crate) fn n4254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4105);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12079);
					match s {
						[] => true,
						['d', 'u', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 14111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12080);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 15049);
									s.is_empty()
								}
								['s', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'p', s @ ..] => {
							super::push(out, 14112);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14113);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14114);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14115);
							s.is_empty()
						}
						['s', 'a', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14116);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'z', 's', 'w', '\u{1}', s @ ..] => {
									super::push(out, 15051);
									s.is_empty()
								}
								['s', 'w', '\u{1}', s @ ..] => {
									super::push(out, 15052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 14117);
							match s {
								[] => true,
								['r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 15053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', 'o', s @ ..] => {
							super::push(out, 14118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14119);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 15054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'g', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15055);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15056);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['m', s @ ..] => {
													super::push(out, 15464);
													match s {
														[] => true,
														['a', 'k', '\u{1}', s @ ..] => super::t4::n16548(s, out),
														_ => false,
													}
												}
												['s', 'o', s @ ..] => {
													super::push(out, 15465);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', 't', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12081);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14121);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 14122);
							s.is_empty()
						}
						['r', 'm', s @ ..] => {
							super::push(out, 14123);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 14124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12082);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 14125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 12083);
					s.is_empty()
				}
				['ó', 'ł', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12084);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 12085);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 12086);
					match s {
						[] => true,
						['i', 'z', s @ ..] => {
							super::push(out, 14126);
							match s {
								[] => true,
								['d', 'w', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 15057);
									s.is_empty()
								}
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
			super::push(out, 8515);
			match s {
				[] => true,
				['ę', 'n', '\u{1}', s @ ..] => {
					super::push(out, 12087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8516);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8517);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14127);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 14128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'z', 'r', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 14129);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 14130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'e', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 12088);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12089);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 15058);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 15059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 14131);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 14132);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14133);
							s.is_empty()
						}
						['p', 'o', 'r', 'ó', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14134);
							s.is_empty()
						}
						['r', 'z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14135);
							s.is_empty()
						}
						['ż', s @ ..] => {
							super::push(out, 14136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łnik.
pub(crate) fn n4255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4106);
	s.is_empty()
}

/// Words ending -śnik.
pub(crate) fn n4256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4107);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 12090);
					s.is_empty()
				}
				['p', 'a', s @ ..] => {
					super::push(out, 12091);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12093);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14137);
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8518);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 12094);
					s.is_empty()
				}
				['n', 'z', 'e', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 12095);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12096);
					s.is_empty()
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14139);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8519);
			s.is_empty()
		}
		['u', 'ł', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 8520);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 8521);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 8522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -źnik.
pub(crate) fn n4257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8523);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 12098);
					s.is_empty()
				}
				['r', 'e', 'b', 'z', '\u{1}', s @ ..] => {
					super::push(out, 12099);
					s.is_empty()
				}
				['z', 'r', '\u{1}', s @ ..] => {
					super::push(out, 12100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8524);
			s.is_empty()
		}
		['y', 'c', 'k', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żnik.
pub(crate) fn n4258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8526);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12101);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 14141);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 14142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 12102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8527);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15060);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 15061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 14143);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14144);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'o', 'n', 'w', 'ó', 'r', '\u{1}', s @ ..] => {
					super::push(out, 12103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', 'z', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 12104);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 12105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8528);
			s.is_empty()
		}
		['ó', 'r', 'd', s @ ..] => {
			super::push(out, 8529);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 8530);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					super::push(out, 12106);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 12107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łopczyk.
pub(crate) fn n15152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14179);
	s.is_empty()
}

/// Words ending -rczyk.
pub(crate) fn n9146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8680);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12183);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14180);
					match s {
						[] => true,
						['s', 'p', '\u{1}', s @ ..] => {
							super::push(out, 15070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 14181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'i', 'b', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 12184);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 12185);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14182);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 14183);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 14184);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 14185);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 14186);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14187);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 15071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 12186);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uczyk.
pub(crate) fn n9147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8681);
	match s {
		[] => true,
		['l', 'k', '\u{1}', s @ ..] => {
			super::push(out, 12187);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wczyk.
pub(crate) fn n9148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8682);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12188);
			match s {
				[] => true,
				['d', 'e', 'z', 'r', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 14188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 12189);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', 'e', 'l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 14189);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12190);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14191);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zczyk.
pub(crate) fn n9149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8683);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14192);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 14193);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14194);
					match s {
						[] => true,
						['b', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 15074);
							s.is_empty()
						}
						['w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 15075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14195);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15076);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15077);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15078);
							match s {
								[] => true,
								['t', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 15389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 14196);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 15079);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'k', s @ ..] => {
							super::push(out, 15080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 14197);
					match s {
						[] => true,
						['ł', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15391);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 15392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęczyk.
pub(crate) fn n9150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8684);
	match s {
		[] => true,
		['z', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 12191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ńczyk.
pub(crate) fn n9151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8685);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12192);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14198);
					s.is_empty()
				}
				['t', 'l', s @ ..] => {
					super::push(out, 14199);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 15081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'h', s @ ..] => {
			super::push(out, 12193);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					super::push(out, 14200);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 15082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14201);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 15083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 12194);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 14202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 't', 'a', s @ ..] => {
			super::push(out, 12195);
			match s {
				[] => true,
				['m', 'l', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 14203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dzyk.
pub(crate) fn n4425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4271);
	s.is_empty()
}

/// Words ending -izyk.
pub(crate) fn n4426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4272);
	s.is_empty()
}

/// Words ending -rzyk.
pub(crate) fn n4427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4273);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8686);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12196);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 12197);
					match s {
						[] => true,
						['ł', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 'ł', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 12198);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12199);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'm', 'c', '\u{1}', s @ ..] => {
							super::push(out, 14205);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14206);
							s.is_empty()
						}
						['ł', 'o', '\u{1}', s @ ..] => {
							super::push(out, 14207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 8687);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 8688);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8689);
			match s {
				[] => true,
				['c', 'y', s @ ..] => {
					super::push(out, 12200);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 14208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12201);
					match s {
						[] => true,
						['n', 'ł', 'o', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 15085);
									s.is_empty()
								}
								['ż', '\u{1}', s @ ..] => {
									super::push(out, 15086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 12202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8690);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 12203);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12204);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 14209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'u', '\u{1}', s @ ..] => {
			super::push(out, 8691);
			s.is_empty()
		}
		['o', 't', 'k', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8692);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 8693);
			match s {
				[] => true,
				['e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8694);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 12206);
					match s {
						[] => true,
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 14210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14211);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 14212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8695);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -szyk.
pub(crate) fn n4428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4274);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8696);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8697);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 8698);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8699);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8700);
			match s {
				[] => true,
				['e', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12207);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12208);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 14213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ęzyk.
pub(crate) fn n4429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4275);
	match s {
		[] => true,
		['j', s @ ..] => {
			super::push(out, 8701);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bieństwo.
pub(crate) fn n16187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15130);
	s.is_empty()
}

/// Words ending -nieństwo.
pub(crate) fn n16188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15131);
	s.is_empty()
}

/// Words ending -leństwo.
pub(crate) fn n15302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14311);
	s.is_empty()
}

/// Words ending -szeństwo.
pub(crate) fn n15303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14312);
	s.is_empty()
}

/// Words ending -eżeństwo.
pub(crate) fn n15304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14313);
	s.is_empty()
}

/// The word blister.
pub(crate) fn n15334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14343);
	s.is_empty()
}

/// Words ending -ilister.
pub(crate) fn n15335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14344);
	match s {
		[] => true,
		['f', '\u{1}', s @ ..] => {
			super::push(out, 15135);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mister.
pub(crate) fn n13347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12543);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nister.
pub(crate) fn n13348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12544);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 14346);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15412);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jster.
pub(crate) fn n9993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9484);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 14347);
					s.is_empty()
				}
				['l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 14348);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kster.
pub(crate) fn n9994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9485);
	s.is_empty()
}

/// Words ending -oster.
pub(crate) fn n9995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9486);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 12545);
			s.is_empty()
		}
		['p', 'm', 'i', '\u{1}', s @ ..] => {
			super::push(out, 12546);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 12547);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pster.
pub(crate) fn n9996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9487);
	s.is_empty()
}

/// Words ending -uster.
pub(crate) fn n9997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9488);
	s.is_empty()
}

/// Words ending -tter.
pub(crate) fn n5455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5254);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12548);
					s.is_empty()
				}
				['s', 'd', 'n', 'e', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 12549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 's', 't', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uter.
pub(crate) fn n5456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5255);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9490);
			match s {
				[] => true,
				['k', 'o', 'r', 't', 'k', 'e', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 12550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9491);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 12551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9492);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12552);
					s.is_empty()
				}
				['n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 12553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9493);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9494);
			match s {
				[] => true,
				['k', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 12554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 9495);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 12555);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lator.
pub(crate) fn n10080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9564);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 14358);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 15146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 14359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['m', 'y', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 14360);
					s.is_empty()
				}
				['p', 'm', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 14361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12594);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 14362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14363);
					s.is_empty()
				}
				['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 14364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14365);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 15147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14366);
					match s {
						[] => true,
						['y', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 15148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 14367);
					match s {
						[] => true,
						['i', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 15149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 12595);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mator.
pub(crate) fn n10081(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12596);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 12597);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 14368);
					s.is_empty()
				}
				['o', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 15150);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 15151);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 12598);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 14369);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nator.
pub(crate) fn n10082(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'm', 'r', s @ ..] => {
			super::push(out, 12599);
			match s {
				[] => true,
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 14370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12600);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14372);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 14373);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 15154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', 't', s @ ..] => {
			super::push(out, 12601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rator.
pub(crate) fn n10083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9565);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12602);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 12603);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12604);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 15155);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 15416);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15417);
									s.is_empty()
								}
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
			super::push(out, 12605);
			match s {
				[] => true,
				['y', 'z', 's', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 14374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'p', 's', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14375);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14376);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 15157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 12606);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 12607);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 12608);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14379);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 15159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tator.
pub(crate) fn n10085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9567);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12609);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['g', 'a', s @ ..] => {
					super::push(out, 14381);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14382);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 15160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'e', 'm', 'i', s @ ..] => {
			super::push(out, 12610);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12611);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wator.
pub(crate) fn n10086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9568);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 12612);
			s.is_empty()
		}
		['k', 'e', '\u{1}', s @ ..] => {
			super::push(out, 12613);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 12614);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 12615);
			s.is_empty()
		}
		['y', 't', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14383);
					s.is_empty()
				}
				['l', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 14384);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zator.
pub(crate) fn n10087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9569);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12616);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12617);
			match s {
				[] => true,
				['t', 'n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 14385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14386);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 15418);
									s.is_empty()
								}
								['k', 'o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 15419);
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 15420);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 15468);
											s.is_empty()
										}
										['o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 15469);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 15421);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 15470);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 15471);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 15161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14387);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14388);
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 15162);
							match s {
								[] => true,
								['h', 'c', 'n', 'y', 's', '\u{1}', s @ ..] => {
									super::push(out, 15422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 14389);
					match s {
						[] => true,
						['o', 'r', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 15163);
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
				['r', s @ ..] => {
					super::push(out, 14390);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word szkieletor.
pub(crate) fn n10089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9571);
	s.is_empty()
}

/// Words ending -itor.
pub(crate) fn n5539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5331);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 9572);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 12618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ktor.
pub(crate) fn n5540(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9573);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12619);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 12620);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12621);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 12622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14392);
							s.is_empty()
						}
						['e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 14393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12624);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 14394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14395);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15164);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 15165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 12626);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12627);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12628);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14398);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 12629);
					s.is_empty()
				}
				['ż', 'd', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 12630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9574);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 12631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9575);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9576);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 12632);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9577);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14400);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 15166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14401);
							s.is_empty()
						}
						['n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 14402);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12633);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntor.
pub(crate) fn n5541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5332);
	match s {
		[] => true,
		['a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 9578);
			s.is_empty()
		}
		['e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 9579);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -otor.
pub(crate) fn n5542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5333);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 9580);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptor.
pub(crate) fn n5543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5334);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9581);
			s.is_empty()
		}
		['e', 'c', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14404);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14405);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 15167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 14406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 12634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', 'k', 's', '\u{1}', s @ ..] => {
			super::push(out, 9582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stor.
pub(crate) fn n5544(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 9583);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9584);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9585);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 12635);
					s.is_empty()
				}
				['h', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'p', 'm', 'i', '\u{1}', s @ ..] => {
			super::push(out, 9586);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 9587);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utor.
pub(crate) fn n5545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5335);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9588);
			s.is_empty()
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 9589);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 9590);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 12637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 9591);
			s.is_empty()
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['e', 'z', 'g', 'e', '\u{1}', s @ ..] => {
					super::push(out, 12638);
					s.is_empty()
				}
				['ą', 'k', '\u{1}', s @ ..] => {
					super::push(out, 12639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ytor.
pub(crate) fn n5546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5336);
	s.is_empty()
}

/// Words ending -ztor.
pub(crate) fn n5547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5337);
	match s {
		[] => true,
		['s', 'a', 'l', 'k', '\u{1}', s @ ..] => {
			super::push(out, 9592);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ątor.
pub(crate) fn n5548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5338);
	match s {
		[] => true,
		['w', '\u{1}', s @ ..] => {
			super::push(out, 9593);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jacz.
pub(crate) fn n6423(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 12848);
					s.is_empty()
				}
				['k', 'o', 'p', 's', 'u', '\u{1}', s @ ..] => {
					super::push(out, 12849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', '\u{1}', s @ ..] => {
							super::push(out, 14448);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 14449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12850);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 12851);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kacz.
pub(crate) fn n6424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6177);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10189);
			s.is_empty()
		}
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 10190);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10191);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10192);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12852);
					match s {
						[] => true,
						['m', 's', '\u{1}', s @ ..] => {
							super::push(out, 14450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12853);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 14451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 12854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12855);
					s.is_empty()
				}
				['i', 'c', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12856);
					s.is_empty()
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 14452);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 14453);
							s.is_empty()
						}
						['ł', '\u{1}', s @ ..] => {
							super::push(out, 14454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10193);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10194);
			match s {
				[] => true,
				['t', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12858);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 12859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', 'ł', 'b', '\u{1}', s @ ..] => {
			super::push(out, 10195);
			s.is_empty()
		}
		['ę', 's', '\u{1}', s @ ..] => {
			super::push(out, 10196);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lacz.
pub(crate) fn n6425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6178);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10197);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14455);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 14456);
							match s {
								[] => true,
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15179);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'z', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10198);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 10199);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10200);
			match s {
				[] => true,
				['p', 'a', s @ ..] => {
					super::push(out, 12862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ś', s @ ..] => {
			super::push(out, 10201);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12863);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 14458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -macz.
pub(crate) fn n6426(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'ł', 'o', 'd', 'o', 'l', '\u{1}', s @ ..] => {
			super::push(out, 10202);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nacz.
pub(crate) fn n6427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6179);
	match s {
		[] => true,
		['d', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 10204);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', 'k', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 12864);
					s.is_empty()
				}
				['p', 's', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12865);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pacz.
pub(crate) fn n6428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6180);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10205);
			match s {
				[] => true,
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 12866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12867);
					s.is_empty()
				}
				['l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 12868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10206);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 12869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10207);
			match s {
				[] => true,
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 12870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10208);
			match s {
				[] => true,
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 12871);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -racz.
pub(crate) fn n6429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6181);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10209);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 12872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'z', 'u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 10210);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10211);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15180);
									s.is_empty()
								}
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 15181);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 15182);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 15426);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 14459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 12873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12874);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12875);
					s.is_empty()
				}
				['z', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 12876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10212);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 10213);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10214);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10215);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12877);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sacz.
pub(crate) fn n6430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6182);
	match s {
		[] => true,
		['o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 10216);
			s.is_empty()
		}
		['ą', s @ ..] => {
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 12878);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 12879);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tacz.
pub(crate) fn n6431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6183);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['i', 'n', 'g', 'z', '\u{1}', s @ ..] => {
					super::push(out, 12880);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12881);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 12882);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 12883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 10217);
			s.is_empty()
		}
		['o', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 10218);
			s.is_empty()
		}
		['r', 'a', s @ ..] => {
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 12884);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 12885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', '\u{1}', s @ ..] => {
			super::push(out, 10219);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 10220);
			s.is_empty()
		}
		['ą', 'z', 'r', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 10221);
			s.is_empty()
		}
		['ę', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 10222);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wacz.
pub(crate) fn n6432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6184);
	match s {
		[] => true,
		['a', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 10223);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10224);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', 'z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 14460);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 14461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 12886);
					match s {
						[] => true,
						['r', 'g', s @ ..] => {
							match s {
								[] => true,
								['d', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15183);
									s.is_empty()
								}
								['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15184);
									s.is_empty()
								}
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
			super::push(out, 10225);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['s', 'y', 'r', s @ ..] => {
							super::push(out, 14462);
							s.is_empty()
						}
						['u', 'z', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10226);
			match s {
				[] => true,
				['d', 'a', 'ł', '\u{1}', s @ ..] => {
					super::push(out, 12887);
					s.is_empty()
				}
				['k', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12888);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12889);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 14464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 12890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10227);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 12891);
					s.is_empty()
				}
				['r', 'p', 'z', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 12892);
					s.is_empty()
				}
				['ł', 'k', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12893);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 12894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10228);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', 'ł', 'w', '\u{1}', s @ ..] => {
							super::push(out, 14465);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14466);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 14467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14468);
							s.is_empty()
						}
						['k', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 14469);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'p', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12895);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', 'd', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14471);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14472);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14473);
							s.is_empty()
						}
						['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['o', 'w', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14475);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 14476);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', 's', '\u{1}', s @ ..] => {
			super::push(out, 10229);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 10230);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zacz.
pub(crate) fn n6433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6185);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 10231);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 10232);
			match s {
				[] => true,
				['e', 'z', 'r', 'p', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 12896);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10233);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 10234);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 12898);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łacz.
pub(crate) fn n6434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6186);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10235);
			match s {
				[] => true,
				['i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 12899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10236);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10237);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 12900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10238);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10239);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 10240);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10241);
			match s {
				[] => true,
				['a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 12902);
					s.is_empty()
				}
				['i', 'm', 'ż', '\u{1}', s @ ..] => {
					super::push(out, 12903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10242);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 10243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żacz.
pub(crate) fn n6435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6187);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 10244);
			match s {
				[] => true,
				['ż', 'e', 'j', 'u', '\u{1}', s @ ..] => {
					super::push(out, 12904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10245);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word kwestionariusz.
pub(crate) fn n16544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15472);
	s.is_empty()
}

/// Words ending -lność.
pub(crate) fn n11067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10489);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13052);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14533);
					match s {
						[] => true,
						['a', 'r', 'g', 'e', 'd', 'o', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15202);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['w', 'y', s @ ..] => {
							super::push(out, 15204);
							s.is_empty()
						}
						['z', 'd', 'e', s @ ..] => {
							super::push(out, 15205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', 'c', s @ ..] => {
					super::push(out, 14534);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14535);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15207);
							s.is_empty()
						}
						['r', 'o', 'f', s @ ..] => {
							super::push(out, 15208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15209);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14536);
					s.is_empty()
				}
				['t', 'n', s @ ..] => {
					super::push(out, 14537);
					s.is_empty()
				}
				['u', 't', 'k', 'a', s @ ..] => {
					super::push(out, 14538);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 14539);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15433);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 15473);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 15434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'c', 'a', 'r', s @ ..] => {
					super::push(out, 14540);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 14541);
					match s {
						[] => true,
						['ą', s @ ..] => {
							super::push(out, 15211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 't', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14542);
					s.is_empty()
				}
				['y', 'z', 'r', s @ ..] => {
					super::push(out, 14543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'b', 'a', 't', 's', 'e', s @ ..] => {
			super::push(out, 13053);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13054);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14544);
					s.is_empty()
				}
				['w', 'o', s @ ..] => {
					super::push(out, 14545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 13055);
			s.is_empty()
		}
		['ś', s @ ..] => {
			super::push(out, 13056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ułomność.
pub(crate) fn n13905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13058);
	s.is_empty()
}

/// Words ending -nność.
pub(crate) fn n11069(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 13059);
			s.is_empty()
		}
		['i', 'w', 'o', s @ ..] => {
			super::push(out, 13060);
			s.is_empty()
		}
		['o', 'ł', 'k', s @ ..] => {
			super::push(out, 13061);
			s.is_empty()
		}
		['y', 'z', 'c', 'o', 'r', s @ ..] => {
			super::push(out, 13062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oność.
pub(crate) fn n11070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10490);
	s.is_empty()
}

/// Words ending -kropność.
pub(crate) fn n11071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10491);
	s.is_empty()
}

/// Words ending -rność.
pub(crate) fn n11072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10492);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14548);
					s.is_empty()
				}
				['l', 'u', s @ ..] => {
					match s {
						[] => true,
						['g', 'e', 'r', 'e', s @ ..] => {
							super::push(out, 15212);
							s.is_empty()
						}
						['k', 's', s @ ..] => {
							super::push(out, 15213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14549);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14551);
					s.is_empty()
				}
				['p', 'd', s @ ..] => {
					super::push(out, 14552);
					s.is_empty()
				}
				['w', 't', 'o', s @ ..] => {
					super::push(out, 14553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'm', 'h', 'c', 'o', s @ ..] => {
			super::push(out, 13063);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sność.
pub(crate) fn n11073(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'ł', s @ ..] => {
			super::push(out, 13064);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13065);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tność.
pub(crate) fn n11074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10493);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['w', 'y', s @ ..] => {
					super::push(out, 14554);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 14555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13066);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 13067);
			s.is_empty()
		}
		['n', 'e', 'l', s @ ..] => {
			super::push(out, 13068);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['k', 'o', s @ ..] => {
							super::push(out, 15214);
							s.is_empty()
						}
						['w', 'd', s @ ..] => {
							super::push(out, 15215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 14556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13069);
			s.is_empty()
		}
		['ę', s @ ..] => {
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 14557);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 13070);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wność.
pub(crate) fn n11075(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14559);
					s.is_empty()
				}
				['r', 'p', 's', s @ ..] => {
					super::push(out, 14560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13071);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14561);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14562);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 15218);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 15219);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 15220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 13072);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 13073);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14563);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zność.
pub(crate) fn n11076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10494);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14564);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 15435);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 15436);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15437);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 15438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 15221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'o', s @ ..] => {
							super::push(out, 15222);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 15439);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 't', s @ ..] => {
							super::push(out, 15223);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 15224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'b', s @ ..] => {
					super::push(out, 14565);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 14566);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							super::push(out, 15225);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 15440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 't', 'ę', s @ ..] => {
			super::push(out, 13074);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13075);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 14567);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -śność.
pub(crate) fn n11077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10495);
	s.is_empty()
}

/// Words ending -żność.
pub(crate) fn n11078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10496);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13076);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'b', 'z', 'o', s @ ..] => {
					super::push(out, 14568);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', 'o', s @ ..] => {
					super::push(out, 14570);
					s.is_empty()
				}
				['r', 'd', 'e', s @ ..] => {
					super::push(out, 14571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 13077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rość.
pub(crate) fn n6640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6387);
	match s {
		[] => true,
		['a', 'z', s @ ..] => {
			super::push(out, 10497);
			s.is_empty()
		}
		['d', 'ą', s @ ..] => {
			super::push(out, 10498);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 10499);
			s.is_empty()
		}
		['t', 's', 'o', s @ ..] => {
			super::push(out, 10500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tość.
pub(crate) fn n6641(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'w', 'o', 'k', 'a', s @ ..] => {
			super::push(out, 10501);
			s.is_empty()
		}
		['i', 'm', s @ ..] => {
			super::push(out, 10502);
			s.is_empty()
		}
		['r', 'a', 'w', 'a', s @ ..] => {
			super::push(out, 10503);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14572);
							s.is_empty()
						}
						['o', 'w', s @ ..] => {
							super::push(out, 14573);
							s.is_empty()
						}
						['w', 'y', s @ ..] => {
							super::push(out, 14574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', s @ ..] => {
					super::push(out, 13078);
					s.is_empty()
				}
				['y', 'z', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'n', s @ ..] => {
							super::push(out, 14575);
							s.is_empty()
						}
						['o', 'r', 'u', s @ ..] => {
							super::push(out, 14576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 13079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13080);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 13081);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wość.
pub(crate) fn n6642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6388);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 13082);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', 'ś', 'a', s @ ..] => {
					super::push(out, 13084);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14577);
							s.is_empty()
						}
						['d', 'ą', s @ ..] => {
							super::push(out, 14578);
							s.is_empty()
						}
						['g', 'e', s @ ..] => {
							super::push(out, 14579);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 14580);
							s.is_empty()
						}
						['p', 't', s @ ..] => {
							super::push(out, 14581);
							s.is_empty()
						}
						['t', 'o', s @ ..] => {
							match s {
								[] => true,
								['p', 'o', s @ ..] => {
									super::push(out, 15226);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 15227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ś', 'i', s @ ..] => {
							super::push(out, 14582);
							s.is_empty()
						}
						['ż', 'o', s @ ..] => {
							super::push(out, 14583);
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
				['b', s @ ..] => {
					super::push(out, 13085);
					match s {
						[] => true,
						['o', 'r', 'o', 'h', 'c', 'o', s @ ..] => {
							super::push(out, 14584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 's', s @ ..] => {
					super::push(out, 13086);
					s.is_empty()
				}
				['d', 'o', 'r', s @ ..] => {
					super::push(out, 13087);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 13088);
					s.is_empty()
				}
				['g', 'ę', s @ ..] => {
					super::push(out, 13089);
					s.is_empty()
				}
				['i', 'c', 'ł', 'p', 's', 'n', s @ ..] => {
					super::push(out, 13090);
					s.is_empty()
				}
				['k', 'u', s @ ..] => {
					super::push(out, 13091);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 13092);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13093);
					s.is_empty()
				}
				['n', 'a', 'l', 'k', s @ ..] => {
					super::push(out, 13094);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['o', 'i', s @ ..] => {
							super::push(out, 14585);
							s.is_empty()
						}
						['u', 'i', s @ ..] => {
							super::push(out, 14586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'w', s @ ..] => {
					super::push(out, 13095);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 13096);
					s.is_empty()
				}
				['ł', 'd', s @ ..] => {
					super::push(out, 13097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'z', 's', s @ ..] => {
			super::push(out, 10504);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zość.
pub(crate) fn n6643(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['i', 'n', 'm', s @ ..] => {
					super::push(out, 13098);
					s.is_empty()
				}
				['l', 'e', 'i', 'z', 'd', 'z', s @ ..] => {
					super::push(out, 13099);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13100);
					s.is_empty()
				}
				['w', 'e', s @ ..] => {
					super::push(out, 13101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łość.
pub(crate) fn n6644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6389);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', 'd', 'e', s @ ..] => {
					super::push(out, 13102);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 13103);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 14587);
							s.is_empty()
						}
						['m', 'u', 'z', 'o', 'r', 'y', s @ ..] => {
							super::push(out, 14588);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							super::push(out, 14589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13104);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13105);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 13106);
					s.is_empty()
				}
				['z', 'r', 'j', 'o', s @ ..] => {
					super::push(out, 13107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'o', s @ ..] => {
			super::push(out, 10506);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14590);
							s.is_empty()
						}
						['p', 'y', s @ ..] => {
							super::push(out, 14591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13109);
					s.is_empty()
				}
				['e', 'l', 'a', s @ ..] => {
					super::push(out, 13110);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 13111);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 14592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10507);
			s.is_empty()
		}
		['k', 'u', 'p', s @ ..] => {
			super::push(out, 10508);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', 'c', 'ś', s @ ..] => {
					super::push(out, 13112);
					s.is_empty()
				}
				['o', 'i', 'n', 'y', s @ ..] => {
					super::push(out, 13113);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 13114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'h', s @ ..] => {
			super::push(out, 10509);
			s.is_empty()
		}
		['z', 's', 'a', s @ ..] => {
			super::push(out, 10510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -owiec.
pub(crate) fn n7819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7463);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 11147);
			s.is_empty()
		}
		['a', 'k', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 11148);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 11149);
			match s {
				[] => true,
				['m', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13459);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13460);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['g', 'o', s @ ..] => {
									super::push(out, 15277);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 15453);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'ą', 'w', '\u{1}', s @ ..] => {
									super::push(out, 15278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'g', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13461);
					s.is_empty()
				}
				['y', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 13462);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13463);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 13464);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 13465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 11150);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', 'b', 'o', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 14715);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 14716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13466);
					s.is_empty()
				}
				['w', 'o', 'ł', 'g', 'i', 'm', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 13467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 11151);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13468);
					match s {
						[] => true,
						['ł', 'k', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 15279);
									s.is_empty()
								}
								['z', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 15280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'w', 'o', 'b', 'z', '\u{1}', s @ ..] => {
					super::push(out, 13469);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13470);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 14717);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 14718);
							s.is_empty()
						}
						['r', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 14719);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 14720);
							match s {
								[] => true,
								['a', 'z', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 15281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'o', 'k', 'a', '\u{1}', s @ ..] => {
					super::push(out, 13471);
					s.is_empty()
				}
				['ą', 'z', 'r', 'o', 'm', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 13472);
					s.is_empty()
				}
				['ę', 'z', 'r', 'b', 'o', 'o', 'r', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 13473);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 13474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'd', 'i', 'z', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 11152);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 11153);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13475);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 14721);
							s.is_empty()
						}
						['p', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 14722);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14723);
							match s {
								[] => true,
								['e', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 15282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13476);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15283);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 15284);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							super::push(out, 14724);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 15285);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 13477);
					match s {
						[] => true,
						['ó', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15286);
									s.is_empty()
								}
								['z', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 15287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 13478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 11154);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14725);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15288);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 14726);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 15289);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 15290);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 15291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14727);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 15292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 15293);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15454);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ł', 's', '\u{1}', s @ ..] => {
									super::push(out, 15294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 14728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 13479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11155);
			match s {
				[] => true,
				['c', 'ś', 'o', 'k', 'o', 's', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 13480);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 15295);
									s.is_empty()
								}
								['n', 'z', 'y', 'w', 'z', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 15296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 15297);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 15455);
											s.is_empty()
										}
										['ł', s @ ..] => super::t4::n16528(s, out),
										_ => false,
									}
								}
								['u', s @ ..] => super::t4::n16365(s, out),
								_ => false,
							}
						}
						['e', s @ ..] => super::t4::n15760(s, out),
						['g', s @ ..] => super::t4::n15761(s, out),
						['i', s @ ..] => super::t4::n15762(s, out),
						['o', 'h', 'a', 'm', '\u{1}', s @ ..] => super::t4::n15763(s, out),
						['p', 'a', 'w', '\u{1}', s @ ..] => super::t4::n15764(s, out),
						['r', 'a', 't', 'a', 'l', '\u{1}', s @ ..] => super::t4::n15765(s, out),
						['y', 'd', '\u{1}', s @ ..] => super::t4::n15766(s, out),
						_ => false,
					}
				}
				['z', 'd', 'e', 'i', 'm', '\u{1}', s @ ..] => super::t4::n14380(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t4::n11826(s, out),
		['k', s @ ..] => super::t4::n11827(s, out),
		['l', s @ ..] => super::t4::n11828(s, out),
		['m', s @ ..] => super::t4::n11829(s, out),
		['n', s @ ..] => super::t4::n11830(s, out),
		['p', s @ ..] => super::t4::n11831(s, out),
		['r', s @ ..] => super::t4::n11832(s, out),
		['s', s @ ..] => super::t4::n11833(s, out),
		['t', s @ ..] => super::t4::n11834(s, out),
		['w', s @ ..] => super::t4::n11835(s, out),
		['z', s @ ..] => super::t4::n11836(s, out),
		['ł', s @ ..] => super::t4::n11837(s, out),
		['ż', s @ ..] => super::t4::n11838(s, out),
		_ => false,
	}
}

/// Words ending -zwolenie.
pub(crate) fn n15950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14903);
	s.is_empty()
}

/// Words ending -tlenie.
pub(crate) fn n11980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11296);
	s.is_empty()
}

/// Words ending -tulenie.
pub(crate) fn n11981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11297);
	s.is_empty()
}

/// Words ending -eślenie.
pub(crate) fn n11982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11298);
	s.is_empty()
}

/// Words ending -zenie.
pub(crate) fn n7967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7595);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 13652);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 13653);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13654);
					s.is_empty()
				}
				['i', 'z', s @ ..] => {
					super::push(out, 13655);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 14904);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14905);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 14906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13656);
					s.is_empty()
				}
				['z', 'c', s @ ..] => {
					super::push(out, 13657);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 13658);
					match s {
						[] => true,
						['r', 'o', s @ ..] => {
							super::push(out, 14907);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14908);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14909);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 14910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14911);
							s.is_empty()
						}
						['w', 'o', 'p', 'o', 'd', s @ ..] => {
							super::push(out, 14912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 14913);
							s.is_empty()
						}
						['r', 'y', s @ ..] => {
							super::push(out, 14914);
							s.is_empty()
						}
						['w', 'o', 'p', 'e', s @ ..] => {
							super::push(out, 14915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'i', 'w', s @ ..] => {
					super::push(out, 13659);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 13660);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 13661);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 13662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 13663);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 13664);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13665);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					super::push(out, 13666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 13667);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 'w', s @ ..] => {
							super::push(out, 14916);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 14917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'n', s @ ..] => {
					super::push(out, 13668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żenie.
pub(crate) fn n7968(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 11299);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 13669);
					s.is_empty()
				}
				['w', 'e', 'c', s @ ..] => {
					super::push(out, 13670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 11300);
			s.is_empty()
		}
		['o', 'w', s @ ..] => {
			super::push(out, 11301);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word kamienicznik.
pub(crate) fn n16548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15476);
	s.is_empty()
}

/// Words ending -łodniowiec.
pub(crate) fn n16528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15456);
	s.is_empty()
}

/// Words ending -udniowiec.
pub(crate) fn n16365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15298);
	s.is_empty()
}

/// Words ending -eniowiec.
pub(crate) fn n15760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14729);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 15299);
			match s {
				[] => true,
				['b', 'e', 'z', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 15457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 15300);
			s.is_empty()
		}
		['z', 'r', 't', 'a', 'p', 'o', 'a', 'z', '\u{1}', s @ ..] => {
			super::push(out, 15301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gniowiec.
pub(crate) fn n15761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14730);
	s.is_empty()
}

/// Words ending -iniowiec.
pub(crate) fn n15762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14731);
	s.is_empty()
}

/// The word mahoniowiec.
pub(crate) fn n15763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14732);
	s.is_empty()
}

/// The word wapniowiec.
pub(crate) fn n15764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14733);
	s.is_empty()
}

/// The word latarniowiec.
pub(crate) fn n15765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14734);
	s.is_empty()
}

/// The word dyniowiec.
pub(crate) fn n15766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14735);
	s.is_empty()
}

/// The word miedziowiec.
pub(crate) fn n14380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13481);
	s.is_empty()
}

/// Words ending -jowiec.
pub(crate) fn n11826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11156);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13482);
			match s {
				[] => true,
				['r', 'k', 'o', 'c', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 14736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13483);
			match s {
				[] => true,
				['m', 'ż', '\u{1}', s @ ..] => {
					super::push(out, 14737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13484);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kowiec.
pub(crate) fn n11827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11157);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13486);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 13487);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13488);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 13489);
			match s {
				[] => true,
				['l', 'a', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 14738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 13490);
			s.is_empty()
		}
		['m', 'o', 'i', 'z', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 13491);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 13492);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 15302);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15303);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'b', 'ę', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14739);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', 'p', 'e', 'z', 'c', 'z', 's', 'y', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 15304);
							s.is_empty()
						}
						['z', 's', 'e', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 15305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 14740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13493);
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14741);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14742);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14744);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14745);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 15459);
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									super::push(out, 15460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14746);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 15307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 14747);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 15308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', s @ ..] => {
					super::push(out, 14748);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15309);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 13494);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14749);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 15311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 14750);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 15312);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 15462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 14751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 13495);
			match s {
				[] => true,
				['ą', s @ ..] => {
					super::push(out, 14752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13496);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14753);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 15313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', 'a', 'ł', '\u{1}', s @ ..] => {
			super::push(out, 13497);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 13498);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'i', 'c', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 15314);
							s.is_empty()
						}
						['u', 'b', 'y', 'z', '\u{1}', s @ ..] => {
							super::push(out, 15315);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 15316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15317);
							s.is_empty()
						}
						['u', 'd', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 15318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 14754);
					match s {
						[] => true,
						['i', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 15319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 13499);
			match s {
				[] => true,
				['i', 'w', 'ź', 'd', '\u{1}', s @ ..] => {
					super::push(out, 14755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 13500);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14756);
					match s {
						[] => true,
						['d', 'y', s @ ..] => {
							super::push(out, 15320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 14757);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 15321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lowiec.
pub(crate) fn n11828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11158);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13501);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 13502);
			s.is_empty()
		}
		['d', 'n', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 13503);
			s.is_empty()
		}
		['e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 13504);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 13505);
			s.is_empty()
		}
		['j', 'a', 't', 's', 'i', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 13506);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 13507);
			s.is_empty()
		}
		['o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 13508);
			s.is_empty()
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14759);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 15322);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'a', '\u{1}', s @ ..] => {
					super::push(out, 14760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', 'u', 'ż', '\u{1}', s @ ..] => {
			super::push(out, 13509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mowiec.
pub(crate) fn n11829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11159);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13510);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 14761);
					s.is_empty()
				}
				['s', 'l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'ż', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 13511);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 13512);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 14763);
					s.is_empty()
				}
				['t', 'a', '\u{1}', s @ ..] => {
					super::push(out, 14764);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 14765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'u', 't', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 13513);
			s.is_empty()
		}
		['u', 'g', '\u{1}', s @ ..] => {
			super::push(out, 13514);
			s.is_empty()
		}
		['y', 'ż', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 13515);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nowiec.
pub(crate) fn n11830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13516);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 14766);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13517);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14767);
					match s {
						[] => true,
						['r', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 15325);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 15326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14768);
					s.is_empty()
				}
				['m', 'ś', 'a', 'j', '\u{1}', s @ ..] => {
					super::push(out, 14769);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14771);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14772);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 15327);
							s.is_empty()
						}
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 15328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 13518);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13519);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 13520);
			match s {
				[] => true,
				['o', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 14774);
					s.is_empty()
				}
				['z', 'r', s @ ..] => {
					super::push(out, 14775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 's', 'y', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 13521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -powiec.
pub(crate) fn n11831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11161);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13522);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13523);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['-', 'p', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 15329);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 15330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13524);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rowiec.
pub(crate) fn n11832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11162);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13525);
			match s {
				[] => true,
				['d', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 14777);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 14778);
					s.is_empty()
				}
				['w', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 14779);
					s.is_empty()
				}
				['ż', 'ę', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 14780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 13526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14781);
					s.is_empty()
				}
				['e', 'ż', '\u{1}', s @ ..] => {
					super::push(out, 14782);
					s.is_empty()
				}
				['m', 'a', '\u{1}', s @ ..] => {
					super::push(out, 14783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 13527);
			match s {
				[] => true,
				['ę', 'w', '\u{1}', s @ ..] => {
					super::push(out, 14784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13528);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 14785);
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 15331);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 15332);
							s.is_empty()
						}
						['u', 'p', 'm', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 15333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13529);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 14786);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 14787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 13530);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 13531);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13532);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 14788);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14789);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13533);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 13534);
			match s {
				[] => true,
				['ę', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 14791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13535);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sowiec.
pub(crate) fn n11833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11163);
	match s {
		[] => true,
		['a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 13536);
			s.is_empty()
		}
		['d', 'i', 'a', '\u{1}', s @ ..] => {
			super::push(out, 13537);
			s.is_empty()
		}
		['e', 's', 'e', '\u{1}', s @ ..] => {
			super::push(out, 13538);
			s.is_empty()
		}
		['i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 13539);
			s.is_empty()
		}
		['n', 'a', 't', 's', 'y', 'd', 'o', 'g', 'u', 'ł', 'd', '\u{1}', s @ ..] => {
			super::push(out, 13540);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13541);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13542);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -towiec.
pub(crate) fn n11834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11164);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13543);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 14792);
					match s {
						[] => true,
						['a', 'ł', 'b', '\u{1}', s @ ..] => {
							super::push(out, 15335);
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
				['c', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 14793);
					s.is_empty()
				}
				['ż', 'd', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13544);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 14795);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14796);
					s.is_empty()
				}
				['z', 'a', '\u{1}', s @ ..] => {
					super::push(out, 14797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13545);
			match s {
				[] => true,
				['o', 'p', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15336);
							s.is_empty()
						}
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 15337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', 'r', 'k', 'o', '\u{1}', s @ ..] => {
			super::push(out, 13546);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wowiec.
pub(crate) fn n11835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11165);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13547);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 14798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 13548);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 13549);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 15338);
							s.is_empty()
						}
						['ń', s @ ..] => {
							super::push(out, 15339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zowiec.
pub(crate) fn n11836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11166);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14799);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13550);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13551);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14801);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 14802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łowiec.
pub(crate) fn n11837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11167);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13553);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 13554);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14803);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 14804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13555);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13556);
			match s {
				[] => true,
				['k', 'o', 'n', s @ ..] => {
					super::push(out, 14806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 13557);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13558);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14807);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żowiec.
pub(crate) fn n11838(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 14808);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 14809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 13559);
			s.is_empty()
		}
		_ => false,
	}
}

