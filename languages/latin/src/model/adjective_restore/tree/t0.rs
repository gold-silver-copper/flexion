use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9);
					match s {
						[] => true,
						['o', 'c', 'i', 'r', s @ ..] => {
							super::push(out, 31);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10);
					match s {
						[] => true,
						['e', 'g', 'i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 141);
									match s {
										[] => true,
										['u', 'n', '\u{1}', s @ ..] => {
											super::push(out, 504);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 142);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 143);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 505);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 506);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 507);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 144);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 145);
									match s {
										[] => true,
										['r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 508);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2);
			match s {
				[] => true,
				['u', 'q', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 32);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 33);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 146);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 509);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 510);
											match s {
												[] => true,
												['q', 's', 'u', s @ ..] => {
													match s {
														[] => true,
														['n', s @ ..] => super::t0::n5058(s, out),
														['t', s @ ..] => super::t0::n5059(s, out),
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
									super::push(out, 147);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 511);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 512);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n5(s, out),
		['s', s @ ..] => super::t0::n6(s, out),
		['t', s @ ..] => {
			super::push(out, 7);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 8);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n26(s, out),
				['e', s @ ..] => super::t0::n27(s, out),
				['i', s @ ..] => super::t0::n28(s, out),
				['n', s @ ..] => {
					super::push(out, 28);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 29);
					match s {
						[] => true,
						['c', s @ ..] => super::t0::n139(s, out),
						['l', s @ ..] => super::t0::n140(s, out),
						['n', s @ ..] => super::t0::n141(s, out),
						['r', s @ ..] => super::t0::n142(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t0::n31(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nusquisque.
pub(crate) fn n5058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4151);
	s.is_empty()
}

/// Words ending -tusquisque.
pub(crate) fn n5059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4152);
	s.is_empty()
}

/// Words ending -r.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 148);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 149);
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
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 513);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1658);
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
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n14(s, out),
		['o', s @ ..] => super::t0::n15(s, out),
		['u', s @ ..] => {
			super::push(out, 15);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 49);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 50);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 627);
									s.is_empty()
								}
								['t', 'p', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 51);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 184);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 185);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 186);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 52);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 17);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 188);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 189);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 1876);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													super::push(out, 4323);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 190);
							match s {
								[] => true,
								['o', s @ ..] => super::t0::n688(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 191);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 55);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
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
				['l', s @ ..] => super::t0::n59(s, out),
				['n', s @ ..] => super::t0::n60(s, out),
				['p', s @ ..] => super::t0::n61(s, out),
				['r', s @ ..] => super::t0::n62(s, out),
				['s', s @ ..] => {
					super::push(out, 60);
					s.is_empty()
				}
				['t', s @ ..] => super::t0::n64(s, out),
				['u', s @ ..] => super::t0::n65(s, out),
				['v', s @ ..] => super::t0::n66(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n20(s, out),
		['n', s @ ..] => super::t0::n21(s, out),
		['o', s @ ..] => super::t0::n22(s, out),
		['p', s @ ..] => super::t0::n23(s, out),
		['r', s @ ..] => super::t0::n24(s, out),
		['u', s @ ..] => super::t0::n25(s, out),
		_ => false,
	}
}

/// Words ending -ax.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 488);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1640);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4138);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4139);
									match s {
										[] => true,
										['l', 'a', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', 'f', 'e', s @ ..] => {
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
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 119);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				['u', 'n', s @ ..] => {
					super::push(out, 491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 122);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 123);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['g', 'u', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 1641);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 4140);
									s.is_empty()
								}
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
			super::push(out, 125);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 492);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 493);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 494);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1644);
							match s {
								[] => true,
								['t', 'i', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 7851);
											match s {
												[] => true,
												['u', 'r', '\u{1}', s @ ..] => {
													super::push(out, 11695);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'a', s @ ..] => {
											super::push(out, 7852);
											match s {
												[] => true,
												['l', '\u{1}', s @ ..] => {
													super::push(out, 11696);
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', 'a', s @ ..] => {
											super::push(out, 7853);
											match s {
												[] => true,
												['l', 'f', '\u{1}', s @ ..] => {
													super::push(out, 11697);
													s.is_empty()
												}
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
							super::push(out, 1645);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['q', 'o', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1647);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 128);
			match s {
				[] => true,
				['i', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ex.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 496);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1648);
							match s {
								[] => true,
								['u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 4141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1649);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4142);
									match s {
										[] => true,
										['l', 'u', 'm', s @ ..] => {
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
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4143);
									match s {
										[] => true,
										['q', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 11698);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11699);
													s.is_empty()
												}
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
							super::push(out, 1650);
							match s {
								[] => true,
								['e', 'c', s @ ..] => {
									super::push(out, 4144);
									match s {
										[] => true,
										['e', 'd', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 11700);
													match s {
														[] => true,
														['u', '\u{1}', s @ ..] => super::t0::n25782(s, out),
														_ => false,
													}
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
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1651);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4145);
									match s {
										[] => true,
										['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 7854);
											s.is_empty()
										}
										['s', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 7855);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'i', s @ ..] => {
									super::push(out, 4146);
									match s {
										[] => true,
										['u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 7856);
											s.is_empty()
										}
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
					super::push(out, 497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 498);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 499);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ix.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 131);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 132);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 500);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4147);
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
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 134);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1653);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4148);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4149);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 4150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1654);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1655);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1656);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cox.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	s.is_empty()
}

/// Words ending -lox.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 502);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nox.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	s.is_empty()
}

/// Words ending -rox.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['e', 'f', s @ ..] => {
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

/// Words ending -ux.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['d', 'e', 'd', s @ ..] => {
			super::push(out, 140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -er.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 150);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 151);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 514);
							match s {
								[] => true,
								['e', 'c', s @ ..] => {
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
						['r', s @ ..] => {
							super::push(out, 515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 152);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1659);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 153);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 154);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 516);
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 517);
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
		['c', s @ ..] => {
			super::push(out, 35);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 156);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 518);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 519);
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
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 520);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 36);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 521);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 522);
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
				['i', s @ ..] => super::t0::n166(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t0::n40(s, out),
		['h', s @ ..] => {
			super::push(out, 38);
			match s {
				[] => true,
				['c', 'l', 'u', 'p', s @ ..] => {
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
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 39);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
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
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 40);
			match s {
				[] => true,
				['e', 'g', s @ ..] => super::t0::n171(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t0::n44(s, out),
		['t', s @ ..] => super::t0::n45(s, out),
		_ => false,
	}
}

/// Words ending -or.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['e', 'd', 'e', s @ ..] => {
					super::push(out, 166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n47(s, out),
		['l', s @ ..] => {
			super::push(out, 45);
			match s {
				[] => true,
				['o', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 624);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 625);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1871);
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
								['n', s @ ..] => {
									super::push(out, 1872);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 4318);
											match s {
												[] => true,
												['i', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n18544(s, out),
														['l', '\u{1}', s @ ..] => super::t0::n18545(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 4319);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 4320);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 1873);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 4321);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', 'o', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 1874);
									match s {
										[] => true,
										['r', 'e', 'v', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 7931);
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t0::n18546(s, out),
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
									super::push(out, 1875);
									match s {
										[] => true,
										['c', 'u', s @ ..] => {
											super::push(out, 4322);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 183);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 47);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 48);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oides.
pub(crate) fn n688(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1877);
			match s {
				[] => true,
				['i', 'r', 'b', s @ ..] => {
					super::push(out, 4324);
					match s {
						[] => true,
						['m', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 7932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'r', 'y', 'h', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'n', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1878);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4325);
					match s {
						[] => true,
						['a', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1879);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1880);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'y', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'e', 'r', 'y', 'h', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 7934);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 7935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1881);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1882);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', 'y', 'h', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'y', 't', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 11743);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 14820);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -les.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 193);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 628);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 195);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nes.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['e', 'g', 'o', s @ ..] => {
			match s {
				[] => true,
				['m', 'o', 'r', 'h', 'c', 'o', 'e', 's', 'i', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['r', 'e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pes.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['b', s @ ..] => {
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
				['c', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1883);
							s.is_empty()
						}
						['i', 'f', s @ ..] => {
							super::push(out, 1884);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 634);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1885);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7936);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 7937);
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
							super::push(out, 1886);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 635);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1888);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1889);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4328);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 636);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1891);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4330);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7938);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1892);
							match s {
								[] => true,
								['c', 's', 'b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 4331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 637);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1893);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1894);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1895);
							match s {
								[] => true,
								['i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 638);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 7939);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11744);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 198);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 199);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -res.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 201);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tes.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	s.is_empty()
}

/// Words ending -ues.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	s.is_empty()
}

/// Words ending -ves.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['i', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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

/// Words ending -is.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 202);
					match s {
						[] => true,
						['u', 'l', 'e', s @ ..] => {
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
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 203);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['l', 'u', 'd', s @ ..] => {
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
		['d', s @ ..] => {
			super::push(out, 66);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 205);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 206);
					match s {
						[] => true,
						['r', 'i', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['g', 'i', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 7940);
													match s {
														[] => true,
														['a', '\u{1}', s @ ..] => super::t0::n18555(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 4333);
											match s {
												[] => true,
												['a', 'l', 'f', '\u{1}', s @ ..] => {
													super::push(out, 7941);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 207);
					match s {
						[] => true,
						['a', 'r', 'g', s @ ..] => {
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
										['v', s @ ..] => {
											super::push(out, 4334);
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
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 208);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 641);
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
				['r', 'i', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 209);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 642);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 643);
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
			super::push(out, 67);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 210);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 644);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1896);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1897);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 1898);
									match s {
										[] => true,
										['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 1899);
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
						['p', s @ ..] => {
							super::push(out, 645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n71(s, out),
		['m', s @ ..] => super::t0::n72(s, out),
		['n', s @ ..] => super::t0::n73(s, out),
		['p', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 233);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 695);
							s.is_empty()
						}
						['l', 'o', 'r', 'c', 'i', s @ ..] => {
							super::push(out, 696);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 234);
					match s {
						[] => true,
						['u', 't', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n75(s, out),
		['s', s @ ..] => super::t0::n76(s, out),
		['t', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 247);
					match s {
						[] => true,
						['s', 'i', 'n', s @ ..] => {
							super::push(out, 718);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 2093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 248);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 249);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 250);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 251);
					s.is_empty()
				}
				['s', s @ ..] => super::t0::n275(s, out),
				['y', s @ ..] => {
					super::push(out, 253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['g', s @ ..] => super::t0::n277(s, out),
				['n', s @ ..] => super::t0::n278(s, out),
				['q', s @ ..] => super::t0::n279(s, out),
				_ => false,
			}
		}
		['v', s @ ..] => super::t0::n79(s, out),
		_ => false,
	}
}

/// Words ending -ns.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => super::t0::n80(s, out),
		['e', s @ ..] => super::t0::n81(s, out),
		['o', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 300);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2751);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 5530);
											match s {
												[] => true,
												['u', 'r', '\u{1}', s @ ..] => {
													super::push(out, 8981);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'c', s @ ..] => {
											super::push(out, 5531);
											match s {
												[] => true,
												['n', 'u', 'p', '\u{1}', s @ ..] => {
													super::push(out, 8982);
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', 'a', 'l', s @ ..] => {
											super::push(out, 5532);
											match s {
												[] => true,
												['f', '\u{1}', s @ ..] => {
													super::push(out, 8983);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 975);
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

/// Words ending -os.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 976);
							s.is_empty()
						}
						['t', 'c', s @ ..] => {
							super::push(out, 977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 81);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['y', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 978);
							s.is_empty()
						}
						['z', 'a', '\u{1}', s @ ..] => {
							super::push(out, 979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 304);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 305);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 85);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 307);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 309);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 310);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 313);
					match s {
						[] => true,
						['a', 'c', 'o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 89);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 90);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ps.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 981);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2753);
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
								['d', s @ ..] => {
									super::push(out, 2754);
									match s {
										[] => true,
										['u', 'n', '\u{1}', s @ ..] => {
											super::push(out, 5533);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 2755);
									match s {
										[] => true,
										['u', 'r', '\u{1}', s @ ..] => {
											super::push(out, 5534);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2756);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5535);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 5536);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2757);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 8984);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n20454(s, out),
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
									super::push(out, 2758);
									match s {
										[] => true,
										['r', 'a', 'p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['m', 'o', 'c', '\u{1}', s @ ..] => {
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
								['v', s @ ..] => {
									super::push(out, 2759);
									match s {
										[] => true,
										['a', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 5537);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 982);
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
									super::push(out, 2760);
									s.is_empty()
								}
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
			super::push(out, 92);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 315);
					s.is_empty()
				}
				['l', 'a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 316);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 318);
					match s {
						[] => true,
						['c', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rs.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['l', 'o', s @ ..] => {
					super::push(out, 319);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 320);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 985);
							match s {
								[] => true,
								['a', 'v', '\u{1}', s @ ..] => {
									super::push(out, 2761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', 'e', 's', 'i', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
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
					super::push(out, 321);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 95);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -us.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 96);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
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
				['l', s @ ..] => {
					super::push(out, 322);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
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
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 323);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 324);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 987);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 2762);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2763);
									match s {
										[] => true,
										['t', 'u', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5538);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 988);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2765);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2766);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'i', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 325);
					match s {
						[] => true,
						['e', 'c', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 2767);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 326);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n101(s, out),
		['d', s @ ..] => super::t0::n102(s, out),
		['e', s @ ..] => super::t0::n103(s, out),
		['f', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n394(s, out),
				['e', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['l', 'i', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3083);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3084);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 6186);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 6187);
											s.is_empty()
										}
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
					super::push(out, 368);
					match s {
						[] => true,
						['d', 'o', s @ ..] => {
							super::push(out, 1146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 369);
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => super::t0::n3643(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t0::n398(s, out),
				['u', s @ ..] => super::t0::n399(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t0::n106(s, out),
		['i', s @ ..] => super::t0::n107(s, out),
		['l', s @ ..] => super::t0::n108(s, out),
		['m', s @ ..] => super::t0::n109(s, out),
		['n', s @ ..] => super::t0::n110(s, out),
		['o', s @ ..] => super::t0::n111(s, out),
		['p', s @ ..] => super::t0::n112(s, out),
		['r', s @ ..] => super::t0::n113(s, out),
		['s', s @ ..] => super::t0::n114(s, out),
		['t', s @ ..] => super::t0::n115(s, out),
		['u', s @ ..] => super::t0::n116(s, out),
		['v', s @ ..] => super::t0::n117(s, out),
		['x', s @ ..] => super::t0::n118(s, out),
		['z', s @ ..] => super::t0::n119(s, out),
		_ => false,
	}
}

/// The word undecemplex.
pub(crate) fn n25782(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ifer.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 523);
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
						['o', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4153);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7857);
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
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1660);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1661);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1662);
					match s {
						[] => true,
						['p', 'r', 'a', s @ ..] => {
							super::push(out, 4154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1663);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1664);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1665);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4155);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4156);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1666);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1668);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1670);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4158);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 4159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1672);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1673);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 4160);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1674);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4161);
							match s {
								[] => true,
								['i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 7859);
									s.is_empty()
								}
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
			super::push(out, 528);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1675);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1676);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1677);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1678);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1679);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1680);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4162);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 529);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1681);
					match s {
						[] => true,
						['o', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1682);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
					super::push(out, 1683);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4166);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 7860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['m', s @ ..] => {
					super::push(out, 1684);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
					super::push(out, 1685);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1686);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 530);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1690);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1691);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4167);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7861);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 7862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1692);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4168);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1694);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['y', s @ ..] => {
					super::push(out, 1695);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7863);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 7864);
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
		['s', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1696);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 1697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t0::n577(s, out),
		['u', s @ ..] => {
			super::push(out, 534);
			s.is_empty()
		}
		['v', s @ ..] => super::t0::n579(s, out),
		['x', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ger.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 160);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 537);
					match s {
						[] => true,
						['u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 539);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 540);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 541);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1708);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1709);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4179);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4180);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4181);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 4182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1710);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 4183);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 7865);
											s.is_empty()
										}
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
					super::push(out, 542);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1711);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1712);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1713);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1715);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1716);
							match s {
								[] => true,
								['a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4184);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4185);
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
						['i', s @ ..] => {
							super::push(out, 1717);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4186);
									match s {
										[] => true,
										['e', 's', '\u{1}', s @ ..] => {
											super::push(out, 7866);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1718);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1719);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4187);
									match s {
										[] => true,
										['l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 4188);
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
							super::push(out, 1720);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											match s {
												[] => true,
												['b', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 545);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1721);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1722);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4189);
									match s {
										[] => true,
										['e', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 7867);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'n', s @ ..] => super::t0::n5117(s, out),
								_ => false,
							}
						}
						['u', s @ ..] => super::t0::n1910(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t0::n590(s, out),
				['t', s @ ..] => super::t0::n591(s, out),
				['v', s @ ..] => super::t0::n592(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gener.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 549);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -per.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['s', 'o', 'r', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ter.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 552);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 553);
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
		['s', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 554);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['d', 'u', s @ ..] => {
							super::push(out, 1741);
							match s {
								[] => true,
								['l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4194);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['x', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ior.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 167);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 557);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 559);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1744);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4195);
									match s {
										[] => true,
										['f', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 11701);
													match s {
														[] => true,
														['i', '\u{1}', s @ ..] => super::t0::n25783(s, out),
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
									super::push(out, 4196);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1745);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1746);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 560);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 561);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1749);
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
				['r', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 564);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1750);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4198);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4199);
									match s {
										[] => true,
										['l', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 11702);
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t0::n25784(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1751);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 1752);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1753);
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 7869);
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
							super::push(out, 1754);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1755);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4200);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 565);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4202);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 7870);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11703);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11704);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 7871);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 7872);
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
										['m', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 566);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 171);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 172);
			s.is_empty()
		}
		['l', s @ ..] => super::t1::n186(s, out),
		['m', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1770);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['i', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t1::n5145(s, out),
								['n', s @ ..] => super::t1::n5146(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n188(s, out),
		['p', s @ ..] => super::t1::n189(s, out),
		['r', s @ ..] => super::t1::n190(s, out),
		['s', s @ ..] => super::t1::n191(s, out),
		['t', s @ ..] => super::t1::n192(s, out),
		['u', s @ ..] => super::t1::n193(s, out),
		['v', s @ ..] => super::t1::n194(s, out),
		['x', s @ ..] => super::t1::n195(s, out),
		_ => false,
	}
}

/// The word ignicolor.
pub(crate) fn n18544(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word lignicolor.
pub(crate) fn n18545(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word diversicolor.
pub(crate) fn n18546(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word atroviridis.
pub(crate) fn n18555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11745);
	s.is_empty()
}

/// Words ending -lis.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['a', s @ ..] => super::t1::n235(s, out),
		['e', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1984);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 8046);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 8047);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 8048);
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
							super::push(out, 1985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n237(s, out),
		['l', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 675);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 677);
					match s {
						[] => true,
						['c', 'i', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 4546);
									match s {
										[] => true,
										['u', 'r', '\u{1}', s @ ..] => {
											super::push(out, 8145);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'b', s @ ..] => {
									super::push(out, 4547);
									match s {
										[] => true,
										['i', 'r', 'c', '\u{1}', s @ ..] => {
											super::push(out, 8146);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'c', s @ ..] => {
									super::push(out, 4548);
									match s {
										[] => true,
										['n', 'u', 'p', '\u{1}', s @ ..] => {
											super::push(out, 8147);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 4549);
									match s {
										[] => true,
										['a', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 8148);
											s.is_empty()
										}
										['e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 8149);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 216);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 678);
					s.is_empty()
				}
				['d', 'e', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -mis.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['f', 'n', 'i', s @ ..] => {
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
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2016);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 679);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 680);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4550);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 8150);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 8151);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 8152);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4551);
									s.is_empty()
								}
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
			super::push(out, 220);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2018);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 682);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 4552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['f', s @ ..] => super::t1::n2256(s, out),
						['n', s @ ..] => {
							super::push(out, 2021);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4556);
									s.is_empty()
								}
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
			super::push(out, 223);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2022);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2023);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2024);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4557);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nis.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2025);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 690);
					match s {
						[] => true,
						['c', 'e', 'd', s @ ..] => {
							super::push(out, 2027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2028);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['l', 'o', s @ ..] => {
							super::push(out, 2030);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4559);
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
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2031);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4560);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 4561);
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
		['n', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2032);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4562);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2033);
							match s {
								[] => true,
								['e', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 4564);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2034);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2035);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 4565);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2036);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
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
										['f', s @ ..] => {
											super::push(out, 8171);
											match s {
												[] => true,
												['u', 'r', '\u{1}', s @ ..] => {
													super::push(out, 11916);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 8172);
											match s {
												[] => true,
												['u', 'f', '\u{1}', s @ ..] => {
													super::push(out, 11917);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 8173);
											match s {
												[] => true,
												['a', 'l', 'p', '\u{1}', s @ ..] => {
													super::push(out, 11918);
													s.is_empty()
												}
												['i', 'p', 's', '\u{1}', s @ ..] => {
													super::push(out, 11919);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 8174);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 11920);
													match s {
														[] => true,
														['l', 'c', '\u{1}', s @ ..] => super::t1::n26004(s, out),
														_ => false,
													}
												}
												['u', 'c', s @ ..] => {
													super::push(out, 11921);
													match s {
														[] => true,
														['s', 'b', 'o', '\u{1}', s @ ..] => super::t1::n26005(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 8175);
											match s {
												[] => true,
												['a', 'l', '\u{1}', s @ ..] => {
													super::push(out, 11922);
													s.is_empty()
												}
												['c', 'n', 'u', 'p', '\u{1}', s @ ..] => {
													super::push(out, 11923);
													s.is_empty()
												}
												['u', 'c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 11924);
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 8176);
											match s {
												[] => true,
												['a', 'l', 'f', '\u{1}', s @ ..] => {
													super::push(out, 11925);
													s.is_empty()
												}
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
							super::push(out, 2037);
							match s {
								[] => true,
								['u', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4566);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => super::t1::n2278(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n774(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t1::n253(s, out),
		['r', s @ ..] => super::t1::n254(s, out),
		['u', s @ ..] => super::t1::n255(s, out),
		_ => false,
	}
}

/// Words ending -ris.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2040);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4572);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8180);
											s.is_empty()
										}
										['g', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 4573);
									match s {
										[] => true,
										['u', 'n', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11926);
													s.is_empty()
												}
												['a', 't', 'x', 'u', 'i', '\u{1}', s @ ..] => {
													super::push(out, 11927);
													s.is_empty()
												}
												['b', s @ ..] => {
													super::push(out, 11928);
													match s {
														[] => true,
														['u', 's', '\u{1}', s @ ..] => super::t1::n26006(s, out),
														_ => false,
													}
												}
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
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 700);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2042);
							s.is_empty()
						}
						['l', s @ ..] => super::t1::n2291(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t1::n785(s, out),
				['m', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t1::n787(s, out),
				['t', s @ ..] => super::t1::n788(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t1::n259(s, out),
		['c', s @ ..] => {
			super::push(out, 237);
			s.is_empty()
		}
		['e', s @ ..] => super::t1::n261(s, out),
		['o', s @ ..] => super::t1::n262(s, out),
		['r', s @ ..] => {
			super::push(out, 240);
			s.is_empty()
		}
		['t', s @ ..] => super::t1::n264(s, out),
		['u', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sis.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['n', s @ ..] => super::t1::n266(s, out),
		['o', s @ ..] => {
			super::push(out, 244);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['a', 't', 'i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2090);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2091);
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4667);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'a', s @ ..] => {
							super::push(out, 2092);
							match s {
								[] => true,
								['l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4668);
									s.is_empty()
								}
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
			super::push(out, 246);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stis.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 719);
			match s {
				[] => true,
				['r', 'g', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4669);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 720);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
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

/// Words ending -guis.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['n', 'i', 'l', 'e', s @ ..] => {
			super::push(out, 721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nuis.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	match s {
		[] => true,
		['e', 't', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -quis.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	s.is_empty()
}

/// Words ending -vis.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
				['u', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4670);
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
		['e', s @ ..] => {
			super::push(out, 258);
			match s {
				[] => true,
				['r', 'b', s @ ..] => {
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 259);
			match s {
				[] => true,
				['l', 'c', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2095);
							match s {
								[] => true,
								['d', s @ ..] => {
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
						['o', s @ ..] => {
							super::push(out, 2096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 260);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2097);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ans.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 724);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2099);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2100);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4671);
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
				['l', 'a', s @ ..] => {
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
				['m', 'u', 'l', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 725);
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
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
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
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['e', 'c', 'a', s @ ..] => {
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
						['u', s @ ..] => {
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
										['b', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 8287);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8288);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
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
												['a', 'm', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2101);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 4672);
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
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4673);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 8289);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => super::t1::n11692(s, out),
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => super::t1::n5764(s, out),
								['x', 'e', '\u{1}', s @ ..] => super::t1::n5765(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => super::t1::n2370(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n287(s, out),
		['d', s @ ..] => super::t1::n288(s, out),
		['e', s @ ..] => super::t1::n289(s, out),
		['f', s @ ..] => super::t1::n290(s, out),
		['g', s @ ..] => super::t1::n291(s, out),
		['h', s @ ..] => super::t1::n292(s, out),
		['i', s @ ..] => super::t1::n293(s, out),
		['l', s @ ..] => super::t1::n294(s, out),
		['m', s @ ..] => super::t1::n295(s, out),
		['n', s @ ..] => super::t1::n296(s, out),
		['o', s @ ..] => super::t1::n297(s, out),
		['p', s @ ..] => super::t1::n298(s, out),
		['r', s @ ..] => super::t1::n299(s, out),
		['s', s @ ..] => super::t1::n300(s, out),
		['t', s @ ..] => super::t1::n301(s, out),
		['u', s @ ..] => super::t1::n302(s, out),
		['v', s @ ..] => super::t1::n303(s, out),
		['x', s @ ..] => super::t1::n304(s, out),
		['z', s @ ..] => super::t1::n305(s, out),
		_ => false,
	}
}

/// Words ending -ens.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 859);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2496);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5138);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 8597);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 8598);
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
									super::push(out, 5139);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 5140);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
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
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5141);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5142);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
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
						['t', s @ ..] => {
							super::push(out, 2499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 860);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2500);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2501);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n963(s, out),
				['l', s @ ..] => {
					super::push(out, 862);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 863);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2507);
							match s {
								[] => true,
								['l', s @ ..] => super::t1::n6710(s, out),
								_ => false,
							}
						}
						['u', s @ ..] => super::t1::n2866(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t1::n966(s, out),
				['u', s @ ..] => super::t1::n967(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n308(s, out),
		['d', s @ ..] => super::t1::n309(s, out),
		['g', s @ ..] => super::t1::n310(s, out),
		['h', s @ ..] => super::t1::n311(s, out),
		['i', s @ ..] => super::t1::n312(s, out),
		['l', s @ ..] => super::t1::n313(s, out),
		['m', s @ ..] => super::t1::n314(s, out),
		['n', s @ ..] => super::t1::n315(s, out),
		['o', s @ ..] => super::t1::n316(s, out),
		['p', s @ ..] => super::t2::n317(s, out),
		['r', s @ ..] => super::t2::n318(s, out),
		['s', s @ ..] => super::t2::n319(s, out),
		['t', s @ ..] => super::t2::n320(s, out),
		['u', s @ ..] => super::t2::n321(s, out),
		['v', s @ ..] => super::t2::n322(s, out),
		['x', s @ ..] => super::t2::n323(s, out),
		_ => false,
	}
}

/// The word atriceps.
pub(crate) fn n20454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12395);
	s.is_empty()
}

/// Words ending -cus.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 990);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2768);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5539);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5540);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5541);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2769);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 2770);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2771);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5542);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 5543);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5544);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2772);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5545);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5546);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5547);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 8985);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2773);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5548);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5549);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5550);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5551);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5552);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
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
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2774);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5553);
									match s {
										[] => true,
										['d', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 8986);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5554);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5555);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5556);
									match s {
										[] => true,
										['n', 'o', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 8987);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 991);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['o', 'o', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 997);
					match s {
						[] => true,
						['r', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'e', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n358(s, out),
		['l', s @ ..] => {
			super::push(out, 331);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1017);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 5759);
									s.is_empty()
								}
								['r', 'u', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5760);
									s.is_empty()
								}
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
			super::push(out, 334);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1021);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2866);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => super::t2::n7856(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t2::n3379(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t2::n1141(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t2::n364(s, out),
		['y', s @ ..] => super::t2::n365(s, out),
		_ => false,
	}
}

/// Words ending -dus.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1025);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1026);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'l', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 339);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1027);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1028);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2869);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1030);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2871);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 5767);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 5768);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5769);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2872);
							match s {
								[] => true,
								['a', s @ ..] => {
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
						['r', s @ ..] => {
							super::push(out, 2873);
							match s {
								[] => true,
								['u', 't', s @ ..] => {
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
						['u', s @ ..] => super::t2::n3391(s, out),
						_ => false,
					}
				}
				['c', s @ ..] => super::t2::n1150(s, out),
				['d', s @ ..] => super::t2::n1151(s, out),
				['f', s @ ..] => super::t2::n1152(s, out),
				['g', s @ ..] => super::t2::n1153(s, out),
				['l', s @ ..] => super::t2::n1154(s, out),
				['m', s @ ..] => super::t2::n1155(s, out),
				['n', s @ ..] => {
					super::push(out, 1037);
					s.is_empty()
				}
				['p', s @ ..] => super::t2::n1157(s, out),
				['r', s @ ..] => super::t2::n1158(s, out),
				['t', s @ ..] => super::t2::n1159(s, out),
				['u', s @ ..] => super::t2::n1160(s, out),
				['v', s @ ..] => super::t2::n1161(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n369(s, out),
		['n', s @ ..] => super::t2::n370(s, out),
		['o', s @ ..] => super::t2::n371(s, out),
		['r', s @ ..] => super::t2::n372(s, out),
		['u', s @ ..] => super::t2::n373(s, out),
		_ => false,
	}
}

/// Words ending -eus.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1057);
					match s {
						[] => true,
						['e', 'h', s @ ..] => {
							super::push(out, 2965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1058);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2966);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2967);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1061);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1062);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2970);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2971);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6070);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1063);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2972);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1064);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2974);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2975);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6072);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2976);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2977);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 6074);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 6075);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6076);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2978);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1066);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2980);
							s.is_empty()
						}
						['o', 'r', 'u', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6077);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6078);
									match s {
										[] => true,
										['d', 'n', 'i', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 13263);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1067);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2981);
							match s {
								[] => true,
								['z', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 6079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 2982);
							match s {
								[] => true,
								['e', s @ ..] => {
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
						['h', s @ ..] => {
							super::push(out, 2983);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2984);
							match s {
								[] => true,
								['s', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 6080);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2985);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2986);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1068);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2988);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 6082);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6083);
									match s {
										[] => true,
										['h', 'p', 'u', 'e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
													super::push(out, 13264);
													s.is_empty()
												}
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
							super::push(out, 2989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1069);
					s.is_empty()
				}
				['i', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n376(s, out),
		['d', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1082);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1083);
					s.is_empty()
				}
				['i', s @ ..] => super::t2::n1204(s, out),
				['n', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				['o', s @ ..] => super::t2::n1206(s, out),
				['u', s @ ..] => super::t2::n1207(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t2::n378(s, out),
		['h', s @ ..] => super::t2::n379(s, out),
		['i', s @ ..] => super::t2::n380(s, out),
		['l', s @ ..] => super::t2::n381(s, out),
		['m', s @ ..] => super::t2::n382(s, out),
		['n', s @ ..] => super::t2::n383(s, out),
		['o', s @ ..] => super::t2::n384(s, out),
		['p', s @ ..] => super::t2::n385(s, out),
		['r', s @ ..] => super::t2::n386(s, out),
		['s', s @ ..] => super::t2::n387(s, out),
		['t', s @ ..] => super::t2::n388(s, out),
		['u', s @ ..] => super::t2::n389(s, out),
		['v', s @ ..] => super::t2::n390(s, out),
		['x', s @ ..] => super::t2::n391(s, out),
		_ => false,
	}
}

/// Words ending -agus.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['p', 'o', 'o', s @ ..] => {
					super::push(out, 3080);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1143);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3081);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 6175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'i', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 6176);
							s.is_empty()
						}
						['v', 'a', s @ ..] => {
							super::push(out, 6177);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 9852);
									s.is_empty()
								}
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
			super::push(out, 1144);
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
		['v', s @ ..] => {
			super::push(out, 1145);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6178);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6179);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9853);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
									super::push(out, 9854);
									match s {
										[] => true,
										['l', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['a', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 6180);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6181);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6182);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9855);
									match s {
										[] => true,
										['r', 'a', s @ ..] => {
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
								['g', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 9856);
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
						['r', s @ ..] => {
							super::push(out, 6183);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6184);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 9857);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 13272);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 13273);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'u', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['r', 'y', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6185);
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
		_ => false,
	}
}

/// The word perlongus.
pub(crate) fn n3643(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ogus.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1147);
			s.is_empty()
		}
		['l', 'a', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1148);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugus.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1149);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3085);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6188);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1150);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3088);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['r', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hus.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['c', 'o', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['d', 'e', 'l', 'o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', 'r', 't', 'i', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['m', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'n', 'a', s @ ..] => {
			match s {
				[] => true,
				['r', 'c', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1153);
					match s {
						[] => true,
						['y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ius.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['u', 'd', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3091);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['c', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9858);
											s.is_empty()
										}
										['l', 'l', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 9859);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1156);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3093);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3094);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 6190);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 6191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3095);
							match s {
								[] => true,
								['a', s @ ..] => super::t2::n8393(s, out),
								['b', s @ ..] => {
									super::push(out, 6193);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6194);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 6195);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3096);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3097);
							match s {
								[] => true,
								['e', s @ ..] => super::t2::n8397(s, out),
								['u', s @ ..] => super::t2::n8398(s, out),
								_ => false,
							}
						}
						['p', s @ ..] => super::t2::n3663(s, out),
						['r', s @ ..] => super::t2::n3664(s, out),
						['s', s @ ..] => super::t2::n3665(s, out),
						['t', s @ ..] => super::t2::n3666(s, out),
						['v', s @ ..] => super::t2::n3667(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t2::n1284(s, out),
				['o', s @ ..] => super::t2::n1285(s, out),
				['r', s @ ..] => super::t2::n1286(s, out),
				['s', s @ ..] => super::t2::n1287(s, out),
				['u', s @ ..] => super::t2::n1288(s, out),
				['y', s @ ..] => super::t2::n1289(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t2::n406(s, out),
		['e', s @ ..] => super::t2::n407(s, out),
		['g', s @ ..] => super::t2::n408(s, out),
		['h', s @ ..] => super::t2::n409(s, out),
		['l', s @ ..] => super::t2::n410(s, out),
		['m', s @ ..] => super::t2::n411(s, out),
		['n', s @ ..] => super::t2::n412(s, out),
		['o', s @ ..] => super::t2::n413(s, out),
		['p', s @ ..] => super::t2::n414(s, out),
		['r', s @ ..] => super::t2::n415(s, out),
		['s', s @ ..] => super::t2::n416(s, out),
		['t', s @ ..] => super::t2::n417(s, out),
		['v', s @ ..] => super::t2::n418(s, out),
		['x', s @ ..] => super::t2::n419(s, out),
		_ => false,
	}
}

/// Words ending -lus.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['p', 'e', 'c', 'o', 'r', 'c', 'i', s @ ..] => {
							super::push(out, 3240);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 6352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1241);
					match s {
						[] => true,
						['h', 'c', 'n', 'o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1242);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3242);
							match s {
								[] => true,
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6353);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3243);
							match s {
								[] => true,
								['o', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1243);
					match s {
						[] => true,
						['e', 's', 'i', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6355);
									match s {
										[] => true,
										['c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 10063);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6356);
									match s {
										[] => true,
										['u', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 10064);
											s.is_empty()
										}
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
					super::push(out, 1244);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6357);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10065);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6358);
									match s {
										[] => true,
										['t', 'u', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 10066);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'n', 'i', s @ ..] => {
									super::push(out, 6359);
									match s {
										[] => true,
										['t', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 10067);
											s.is_empty()
										}
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
					super::push(out, 1245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1246);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3244);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'o', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1247);
					match s {
						[] => true,
						['p', 'o', s @ ..] => {
							match s {
								[] => true,
								['m', 'r', s @ ..] => {
									super::push(out, 6360);
									match s {
										[] => true,
										['e', 'h', 't', 'o', 'r', 'a', 'e', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 10068);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									super::push(out, 6361);
									match s {
										[] => true,
										['x', '\u{1}', s @ ..] => {
											super::push(out, 10069);
											s.is_empty()
										}
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
					super::push(out, 1248);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1249);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1250);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1251);
					match s {
						[] => true,
						['q', 'a', s @ ..] => {
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
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3245);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 3247);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 6362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3248);
							match s {
								[] => true,
								['r', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3249);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3250);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3251);
							match s {
								[] => true,
								['a', 'c', 'i', s @ ..] => super::t2::n8602(s, out),
								_ => false,
							}
						}
						['s', s @ ..] => super::t2::n3842(s, out),
						['t', s @ ..] => {
							super::push(out, 3253);
							s.is_empty()
						}
						['u', s @ ..] => super::t2::n3844(s, out),
						['v', s @ ..] => super::t2::n3845(s, out),
						['x', s @ ..] => {
							super::push(out, 3256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t2::n1387(s, out),
				['y', s @ ..] => super::t2::n1388(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n424(s, out),
		['p', s @ ..] => super::t2::n425(s, out),
		['u', s @ ..] => super::t2::n426(s, out),
		['y', s @ ..] => super::t2::n427(s, out),
		_ => false,
	}
}

/// Words ending -mus.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 399);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3296);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 10114);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 10115);
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
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n430(s, out),
		['l', s @ ..] => {
			super::push(out, 402);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 403);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1292);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6430);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10144);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 10145);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 6431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1293);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1294);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1295);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1297);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6432);
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
				['o', s @ ..] => {
					super::push(out, 1298);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 3312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1300);
					match s {
						[] => true,
						['g', 'i', 't', 'u', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1301);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3314);
							s.is_empty()
						}
						['u', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1302);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 1303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1304);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nus.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['a', s @ ..] => super::t2::n437(s, out),
		['e', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1327);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3385);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6522);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 6523);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 6524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1328);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 3387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1329);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3388);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3389);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3390);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 6525);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 6526);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6527);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 10204);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6528);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 10205);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6529);
									match s {
										[] => true,
										['c', 'u', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 10206);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 6530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1330);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1331);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1332);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3391);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['p', s @ ..] => {
							super::push(out, 3392);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6531);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1333);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3393);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3394);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3395);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1334);
					match s {
						[] => true,
						['m', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 1335);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3397);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3398);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3399);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 6532);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3400);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1336);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3402);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3403);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1337);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1339);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'a', 'u', 'q', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'l', 'a', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1340);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3405);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1341);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'i', s @ ..] => {
							super::push(out, 3407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n440(s, out),
		['m', s @ ..] => super::t2::n441(s, out),
		['n', s @ ..] => super::t2::n442(s, out),
		['o', s @ ..] => super::t2::n443(s, out),
		['r', s @ ..] => super::t2::n444(s, out),
		['u', s @ ..] => super::t2::n445(s, out),
		['y', s @ ..] => {
			super::push(out, 417);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ous.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 418);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 419);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 420);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1373);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1375);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pus.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 422);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 423);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 424);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['a', 'p', 'i', 'v', 'a', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1377);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1378);
					s.is_empty()
				}
				['n', 'a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['c', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 427);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							super::push(out, 3490);
							match s {
								[] => true,
								['u', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6664);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6665);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 13496);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n28786(s, out),
														['m', '\u{1}', s @ ..] => super::t2::n28787(s, out),
														_ => false,
													}
												}
												['i', s @ ..] => {
													super::push(out, 13497);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6666);
									match s {
										[] => true,
										['y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 10255);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 429);
			match s {
				[] => true,
				['i', 'r', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3491);
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 3492);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3493);
							match s {
								[] => true,
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
		['y', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1381);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1382);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3494);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rus.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1383);
					match s {
						[] => true,
						['r', 'a', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6667);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1384);
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
				['d', s @ ..] => {
					super::push(out, 1385);
					match s {
						[] => true,
						['i', 'r', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3498);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 6668);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3499);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 6669);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 6670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1387);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 6671);
									match s {
										[] => true,
										['l', 'u', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1388);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1389);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 6672);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 6673);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 6674);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 6675);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6676);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6677);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 6678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1390);
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
				['t', s @ ..] => {
					super::push(out, 1391);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1392);
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
								['b', s @ ..] => {
									super::push(out, 6679);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 432);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1393);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1394);
					match s {
						[] => true,
						['d', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 3501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 434);
			s.is_empty()
		}
		['e', s @ ..] => super::t2::n464(s, out),
		['h', s @ ..] => {
			super::push(out, 436);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1408);
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
				['l', s @ ..] => {
					super::push(out, 1409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n467(s, out),
		['r', s @ ..] => {
			super::push(out, 439);
			match s {
				[] => true,
				['u', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'b', '\u{1}', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1419);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['e', 'p', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t2::n470(s, out),
		['y', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['p', 'r', 'o', 'p', 'o', 'n', 'i', 'l', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sus.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1432);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3551);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3552);
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
						['i', s @ ..] => {
							super::push(out, 3553);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3554);
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
				['u', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['s', s @ ..] => {
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
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1435);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
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
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1438);
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
				['m', s @ ..] => {
					super::push(out, 1439);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3556);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3557);
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
					super::push(out, 1440);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 1442);
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
			super::push(out, 445);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3558);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', s @ ..] => {
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
						['n', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3559);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 6830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => super::t2::n4245(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n1587(s, out),
				['n', s @ ..] => super::t2::n1588(s, out),
				['r', s @ ..] => super::t2::n1589(s, out),
				['v', s @ ..] => super::t2::n1590(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n475(s, out),
		['n', s @ ..] => super::t2::n476(s, out),
		['o', s @ ..] => super::t2::n477(s, out),
		['p', s @ ..] => super::t2::n478(s, out),
		['r', s @ ..] => super::t2::n479(s, out),
		['s', s @ ..] => super::t2::n480(s, out),
		['u', s @ ..] => super::t2::n481(s, out),
		['y', s @ ..] => super::t2::n482(s, out),
		_ => false,
	}
}

/// Words ending -tus.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['a', s @ ..] => super::t3::n483(s, out),
		['c', s @ ..] => super::t3::n484(s, out),
		['e', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1516);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1517);
					match s {
						[] => true,
						['a', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7564);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3898);
							match s {
								[] => true,
								['d', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1519);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
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
										['r', 'i', '\u{1}', s @ ..] => {
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
				['l', s @ ..] => {
					super::push(out, 1521);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3899);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3900);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7566);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 7567);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7568);
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
						['p', s @ ..] => {
							super::push(out, 3901);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7569);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11463);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 11464);
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
									match s {
										[] => true,
										['i', s @ ..] => {
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
										['o', 'c', s @ ..] => {
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
								['p', s @ ..] => super::t3::n10366(s, out),
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t3::n1671(s, out),
				['o', s @ ..] => super::t3::n1672(s, out),
				['p', s @ ..] => super::t3::n1673(s, out),
				['r', s @ ..] => super::t3::n1674(s, out),
				['s', s @ ..] => super::t3::n1675(s, out),
				['u', s @ ..] => super::t3::n1676(s, out),
				['v', s @ ..] => super::t3::n1677(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n486(s, out),
		['l', s @ ..] => super::t3::n487(s, out),
		['m', s @ ..] => super::t3::n488(s, out),
		['n', s @ ..] => super::t3::n489(s, out),
		['o', s @ ..] => super::t3::n490(s, out),
		['p', s @ ..] => super::t3::n491(s, out),
		['r', s @ ..] => super::t3::n492(s, out),
		['s', s @ ..] => super::t3::n493(s, out),
		['t', s @ ..] => super::t3::n494(s, out),
		['u', s @ ..] => super::t3::n495(s, out),
		['x', s @ ..] => super::t3::n496(s, out),
		['y', s @ ..] => super::t3::n497(s, out),
		_ => false,
	}
}

/// Words ending -uus.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1597);
					match s {
						[] => true,
						['v', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 1598);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4090);
							s.is_empty()
						}
						['p', 's', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7779);
									match s {
										[] => true,
										['o', 'c', s @ ..] => {
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
								['o', s @ ..] => {
									super::push(out, 7780);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1599);
					match s {
						[] => true,
						['n', s @ ..] => {
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
					super::push(out, 1600);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4091);
							match s {
								[] => true,
								['p', s @ ..] => {
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
						['i', s @ ..] => {
							super::push(out, 4092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1601);
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
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
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4093);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4094);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7782);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
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
								['e', s @ ..] => {
									super::push(out, 7783);
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
						['s', s @ ..] => {
							super::push(out, 4095);
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 4096);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7784);
									match s {
										[] => true,
										['d', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['a', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1603);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1604);
					match s {
						[] => true,
						['i', 'b', s @ ..] => {
							super::push(out, 4097);
							match s {
								[] => true,
								['a', 't', '\u{1}', s @ ..] => {
									super::push(out, 7785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4098);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7786);
									s.is_empty()
								}
								_ => false,
							}
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
				_ => false,
			}
		}
		['n', s @ ..] => super::t3::n502(s, out),
		['q', s @ ..] => super::t3::n503(s, out),
		['r', s @ ..] => super::t3::n504(s, out),
		['t', s @ ..] => super::t3::n505(s, out),
		_ => false,
	}
}

/// Words ending -vus.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1616);
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
				['l', s @ ..] => {
					super::push(out, 1617);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', 'i', 'r', 'i', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 1619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n508(s, out),
		['l', s @ ..] => {
			super::push(out, 480);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
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
				['i', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'b', 'l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 481);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
				['u', 'c', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -xus.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4125);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 7843);
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 14815);
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
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
								['f', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7844);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 7845);
									match s {
										[] => true,
										['n', 'e', 'g', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4126);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
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
						['o', s @ ..] => {
							super::push(out, 4127);
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
				['p', s @ ..] => {
					super::push(out, 1633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4128);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1634);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4129);
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
						['n', s @ ..] => {
							super::push(out, 4130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4131);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7846);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 7847);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4132);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'c', 'u', 'r', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4133);
							match s {
								[] => true,
								['i', s @ ..] => super::t3::n10975(s, out),
								['o', s @ ..] => super::t3::n10976(s, out),
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => super::t3::n5025(s, out),
						['s', s @ ..] => super::t3::n5026(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t3::n1794(s, out),
				['n', s @ ..] => super::t3::n1795(s, out),
				['r', s @ ..] => super::t3::n1796(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t3::n515(s, out),
		['u', s @ ..] => super::t3::n516(s, out),
		_ => false,
	}
}

/// Words ending -zus.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	s.is_empty()
}

/// Words ending -tifer.
pub(crate) fn n577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 533);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1698);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1699);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4170);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1700);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4172);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1701);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4174);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1702);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1703);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1704);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1705);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vifer.
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4176);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4177);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nstriger.
pub(crate) fn n5117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4190);
	s.is_empty()
}

/// Words ending -uriger.
pub(crate) fn n1910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1723);
	match s {
		[] => true,
		['a', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 4191);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 7868);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -siger.
pub(crate) fn n590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 546);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1724);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiger.
pub(crate) fn n591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 547);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1726);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1727);
			match s {
				[] => true,
				['u', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 4192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1728);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1729);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1730);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1731);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1732);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1733);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -viger.
pub(crate) fn n592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 548);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1735);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1736);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word infelicior.
pub(crate) fn n25783(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word dilucidior.
pub(crate) fn n25784(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

