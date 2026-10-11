use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['c', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 19);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 97);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 608);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 609);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 610);
											s.is_empty()
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1912);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 1913);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 611);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 612);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 613);
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', s @ ..] => {
									super::push(out, 98);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 614);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 615);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 616);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1914);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 1915);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 617);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 618);
											s.is_empty()
										}
										['ś', s @ ..] => {
											super::push(out, 619);
											match s {
												[] => true,
												['w', s @ ..] => {
													super::push(out, 1916);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 99);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 620);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 100);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 621);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 622);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 20);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 102);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 103);
									match s {
										[] => true,
										['o', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1917);
													s.is_empty()
												}
												['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 1918);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 104);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 105);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 106);
									s.is_empty()
								}
								['w', s @ ..] => super::t0::n110(s, out),
								['z', s @ ..] => super::t0::n111(s, out),
								_ => false,
							}
						}
						['z', s @ ..] => super::t0::n21(s, out),
						['ż', s @ ..] => super::t0::n22(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t0::n5(s, out),
				['y', s @ ..] => super::t0::n6(s, out),
				['ó', s @ ..] => super::t0::n7(s, out),
				['ą', s @ ..] => super::t0::n8(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n2(s, out),
		['ć', s @ ..] => super::t0::n3(s, out),
		_ => false,
	}
}

/// Words ending -wlec.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zlec.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	s.is_empty()
}

/// Words ending -zec.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 624);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 625);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 626);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 627);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3888);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3889);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 629);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 630);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 1919);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żec.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 109);
			s.is_empty()
		}
		['d', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 110);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uc.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['ł', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 111);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 112);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yc.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['z', 'r', 't', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 113);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 114);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -óc.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 115);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 116);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 117);
					match s {
						[] => true,
						['p', 's', s @ ..] => {
							super::push(out, 632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 118);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 119);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąc.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 23);
			match s {
				[] => true,
				['s', 'y', 'z', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 633);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1921);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3890);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 1922);
									s.is_empty()
								}
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
			super::push(out, 24);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 120);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 121);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 122);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 25);
			match s {
				[] => true,
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 634);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 635);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	s.is_empty()
}

/// Words ending -ć.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['a', s @ ..] => super::t0::n9(s, out),
		['e', s @ ..] => super::t0::n10(s, out),
		['i', s @ ..] => super::t0::n11(s, out),
		['u', s @ ..] => {
			super::push(out, 12);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 66);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1626);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1627);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 67);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1628);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1630);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 68);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1632);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 1633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 69);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 509);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1634);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1635);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1636);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 70);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1637);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 1638);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1639);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 3472);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ł', s @ ..] => {
									super::push(out, 1640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 71);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1641);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1642);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 72);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 510);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t0::n13(s, out),
		['é', s @ ..] => {
			super::push(out, 14);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 15);
			s.is_empty()
		}
		['ą', s @ ..] => super::t0::n16(s, out),
		['ś', s @ ..] => super::t0::n17(s, out),
		['ź', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 94);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 604);
							match s {
								[] => true,
								['w', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3884);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3885);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 6007);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 6008);
													s.is_empty()
												}
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
							super::push(out, 605);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1909);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 1910);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 3886);
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 3887);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 1911);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 95);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 96);
					match s {
						[] => true,
						['z', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ać.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 26);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 124);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 125);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 637);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 639);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 640);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 127);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 643);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 1924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 644);
							match s {
								[] => true,
								['r', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3891);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3892);
											s.is_empty()
										}
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
					super::push(out, 128);
					match s {
						[] => true,
						['c', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 645);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 129);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 130);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 648);
							match s {
								[] => true,
								['o', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3893);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 3894);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 3895);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 3896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 649);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 650);
							match s {
								[] => true,
								['k', 's', '\u{1}', s @ ..] => {
									super::push(out, 1925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 131);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 132);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 651);
							s.is_empty()
						}
						['k', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1926);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1927);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 1928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1929);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 1930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 133);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 652);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1931);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3897);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 1932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ó', 'i', 'z', 'd', '\u{1}', s @ ..] => {
					super::push(out, 134);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 135);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1933);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1934);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 3898);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 136);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 654);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 655);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t0::n683(s, out),
						['ł', s @ ..] => {
							super::push(out, 657);
							match s {
								[] => true,
								['p', 'o', s @ ..] => {
									super::push(out, 1937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 137);
					match s {
						[] => true,
						['i', 'b', s @ ..] => super::t0::n685(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n144(s, out),
				['r', s @ ..] => super::t0::n145(s, out),
				['u', s @ ..] => super::t0::n146(s, out),
				['y', s @ ..] => super::t0::n147(s, out),
				['ó', s @ ..] => super::t0::n148(s, out),
				['ą', s @ ..] => super::t0::n149(s, out),
				['ę', s @ ..] => super::t0::n150(s, out),
				['ł', s @ ..] => super::t0::n151(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n31(s, out),
		['f', s @ ..] => super::t0::n32(s, out),
		['g', s @ ..] => super::t0::n33(s, out),
		['h', s @ ..] => super::t0::n34(s, out),
		['i', s @ ..] => super::t0::n35(s, out),
		['j', s @ ..] => super::t0::n36(s, out),
		['k', s @ ..] => super::t0::n37(s, out),
		['l', s @ ..] => super::t0::n38(s, out),
		['m', s @ ..] => super::t0::n39(s, out),
		['n', s @ ..] => super::t0::n40(s, out),
		['p', s @ ..] => super::t0::n41(s, out),
		['r', s @ ..] => super::t0::n42(s, out),
		['s', s @ ..] => super::t0::n43(s, out),
		['t', s @ ..] => super::t0::n44(s, out),
		['w', s @ ..] => super::t0::n45(s, out),
		['z', s @ ..] => super::t0::n46(s, out),
		['ł', s @ ..] => super::t0::n47(s, out),
		['ż', s @ ..] => super::t0::n48(s, out),
		_ => false,
	}
}

/// Words ending -eć.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['i', s @ ..] => super::t0::n49(s, out),
		['j', s @ ..] => {
			super::push(out, 47);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 48);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 339);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1227);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1228);
							s.is_empty()
						}
						['z', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2892);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 340);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2894);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2895);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2897);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 341);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 2899);
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2900);
									s.is_empty()
								}
								['p', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 2901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 342);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 343);
					match s {
						[] => true,
						['c', 'u', 'r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2902);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'u', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 345);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1230);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 346);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2904);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 2905);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2906);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1233);
							match s {
								[] => true,
								['a', 'r', 'z', '\u{1}', s @ ..] => {
									super::push(out, 2907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1235);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1236);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1237);
							match s {
								[] => true,
								['z', 'c', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 2908);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 348);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 349);
					match s {
						[] => true,
						['ą', 'w', 'z', s @ ..] => {
							super::push(out, 1239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 350);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 351);
					match s {
						[] => true,
						['s', 'a', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2909);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 2910);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 5143);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 2911);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 352);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1240);
							match s {
								[] => true,
								['r', 'o', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5144);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 5145);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 1241);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5146);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 5147);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 5148);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 353);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1243);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1244);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1245);
							match s {
								[] => true,
								['b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 2912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2913);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 2914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 1246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => super::t0::n53(s, out),
		['ż', s @ ..] => super::t0::n54(s, out),
		_ => false,
	}
}

/// Words ending -ić.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 363);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1281);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 364);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1283);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1285);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2998);
									s.is_empty()
								}
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 2999);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3000);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 3001);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 5214);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 5215);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 53);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 366);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1288);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5216);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 5217);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 7145);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 7146);
													s.is_empty()
												}
												_ => false,
											}
										}
										['z', s @ ..] => {
											super::push(out, 5218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3002);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 3003);
									s.is_empty()
								}
								['z', 's', '\u{1}', s @ ..] => {
									super::push(out, 3004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 368);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 369);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1291);
							match s {
								[] => true,
								['h', 'c', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1292);
							match s {
								[] => true,
								['ź', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5219);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 5220);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => super::t0::n1385(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 370);
					s.is_empty()
				}
				['l', s @ ..] => super::t0::n381(s, out),
				['m', s @ ..] => {
					super::push(out, 372);
					s.is_empty()
				}
				['o', s @ ..] => super::t0::n383(s, out),
				['r', s @ ..] => super::t0::n384(s, out),
				['u', s @ ..] => super::t0::n385(s, out),
				['w', s @ ..] => {
					super::push(out, 376);
					s.is_empty()
				}
				['y', s @ ..] => super::t0::n387(s, out),
				['z', s @ ..] => super::t0::n388(s, out),
				['ą', s @ ..] => super::t0::n389(s, out),
				['ę', s @ ..] => super::t0::n390(s, out),
				['ń', s @ ..] => super::t0::n391(s, out),
				['ź', s @ ..] => super::t0::n392(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n57(s, out),
		['e', s @ ..] => super::t0::n58(s, out),
		['f', s @ ..] => super::t0::n59(s, out),
		['l', s @ ..] => super::t0::n60(s, out),
		['m', s @ ..] => super::t0::n61(s, out),
		['n', s @ ..] => super::t0::n62(s, out),
		['o', s @ ..] => super::t0::n63(s, out),
		['p', s @ ..] => super::t0::n64(s, out),
		['s', s @ ..] => super::t0::n65(s, out),
		['w', s @ ..] => super::t1::n66(s, out),
		['z', s @ ..] => super::t1::n67(s, out),
		_ => false,
	}
}

/// Words ending -yć.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 512);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 514);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1644);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1648);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3473);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 78);
			s.is_empty()
		}
		['z', s @ ..] => super::t1::n82(s, out),
		['ż', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1702);
							match s {
								[] => true,
								['z', 'd', 'a', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1703);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1704);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5765);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 5766);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t1::n1833(s, out),
						['s', s @ ..] => {
							super::push(out, 1706);
							s.is_empty()
						}
						['w', s @ ..] => super::t1::n1835(s, out),
						['z', s @ ..] => {
							super::push(out, 1708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t1::n551(s, out),
				['e', s @ ..] => super::t1::n552(s, out),
				['i', s @ ..] => super::t1::n553(s, out),
				['l', s @ ..] => super::t1::n554(s, out),
				['m', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['o', s @ ..] => super::t1::n556(s, out),
				['r', s @ ..] => super::t1::n557(s, out),
				['u', s @ ..] => super::t1::n558(s, out),
				['y', s @ ..] => super::t1::n559(s, out),
				['z', s @ ..] => super::t1::n560(s, out),
				['ó', s @ ..] => super::t1::n561(s, out),
				['ą', s @ ..] => super::t1::n562(s, out),
				['ę', s @ ..] => super::t1::n563(s, out),
				['ł', s @ ..] => super::t1::n564(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąć.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 546);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1744);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1745);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1746);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1749);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1750);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1751);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 1752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1753);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1754);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3650);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3651);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 1755);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1756);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1757);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1758);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1759);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1760);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1762);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1763);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1764);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1766);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1767);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1768);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1769);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1770);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3652);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 3653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1772);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 553);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 555);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 556);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				['z', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1776);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1777);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n88(s, out),
		['z', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1880);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1881);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5979);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 5980);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 5981);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 5982);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 5983);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 1882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 582);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 584);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ść.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 586);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1883);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1884);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3836);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 1885);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5984);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 5985);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1886);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1887);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3838);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 3839);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5986);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1888);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1889);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1890);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1891);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 3841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 1892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 587);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3842);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3843);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5987);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 5988);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 3844);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3845);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3846);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5989);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 3847);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 3848);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 588);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 1893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 589);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1894);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1895);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3851);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3852);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 3853);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3854);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3855);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 3856);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5990);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3857);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5991);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 5992);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 3858);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 3859);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 3860);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1896);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3862);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3863);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3864);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5993);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 5994);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 3865);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 5995);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 5996);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => super::t1::n4262(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => super::t1::n611(s, out),
				['l', s @ ..] => super::t1::n612(s, out),
				['z', s @ ..] => super::t1::n613(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n93(s, out),
		['j', s @ ..] => super::t1::n94(s, out),
		['ó', s @ ..] => super::t1::n95(s, out),
		['ą', s @ ..] => super::t1::n96(s, out),
		_ => false,
	}
}

/// Words ending -racać.
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 1936);
			s.is_empty()
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3899);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3900);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3901);
					match s {
						[] => true,
						['o', 'o', s @ ..] => {
							super::push(out, 6009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3902);
					match s {
						[] => true,
						['z', 'r', 'p', 'o', s @ ..] => {
							super::push(out, 6010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'w', s @ ..] => {
					super::push(out, 3903);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -biecać.
pub(crate) fn n685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	s.is_empty()
}

/// Words ending -icać.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcać.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	s.is_empty()
}

/// Words ending -ucać.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 660);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1938);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 661);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3904);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7520);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 7521);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 3905);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 6012);
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8263);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 8264);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ycać.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['s', 'e', s @ ..] => {
			super::push(out, 662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ócać.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['ł', 'k', 'y', s @ ..] => {
			super::push(out, 663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ącać.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	s.is_empty()
}

/// Words ending -ęcać.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 1940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łcać.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	s.is_empty()
}

/// Words ending -dać.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 146);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1941);
							s.is_empty()
						}
						['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1942);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1944);
							s.is_empty()
						}
						['o', 'p', s @ ..] => {
							super::push(out, 1945);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1946);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1947);
							match s {
								[] => true,
								['ś', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1948);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3908);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3909);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6013);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 6014);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 6015);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7522);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['y', 'z', s @ ..] => {
									super::push(out, 3910);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3911);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1949);
							match s {
								[] => true,
								['o', 'p', 's', s @ ..] => {
									super::push(out, 3912);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 6016);
											s.is_empty()
										}
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
					super::push(out, 669);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1950);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3913);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1951);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 671);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 1952);
							s.is_empty()
						}
						['d', 'o', 'p', s @ ..] => {
							super::push(out, 1953);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1954);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1955);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3916);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 3917);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6017);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1956);
							s.is_empty()
						}
						['y', 'z', s @ ..] => {
							super::push(out, 1957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3918);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3919);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3920);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3921);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 3922);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 3923);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1958);
							match s {
								[] => true,
								['s', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 674);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3926);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3927);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 6018);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 148);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 675);
					match s {
						[] => true,
						['w', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 676);
					match s {
						[] => true,
						['r', 'p', 'a', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 150);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3928);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3929);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											super::push(out, 6019);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3930);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6020);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 3931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n158(s, out),
		['o', s @ ..] => {
			super::push(out, 153);
			s.is_empty()
		}
		['r', s @ ..] => super::t1::n160(s, out),
		['u', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['i', s @ ..] => super::t1::n708(s, out),
				_ => false,
			}
		}
		['w', s @ ..] => super::t1::n162(s, out),
		['y', s @ ..] => super::t1::n163(s, out),
		['z', s @ ..] => super::t1::n164(s, out),
		['ą', s @ ..] => super::t1::n165(s, out),
		_ => false,
	}
}

/// Words ending -fać.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['c', 'y', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gać.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1968);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', 'n', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 3941);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1969);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3943);
									s.is_empty()
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 6023);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 6024);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 690);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 1972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3944);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3945);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3946);
									s.is_empty()
								}
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
			super::push(out, 163);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3947);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 3948);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 693);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 694);
					match s {
						[] => true,
						['r', 't', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['ś', 'y', s @ ..] => {
							super::push(out, 1975);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 3950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 697);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1977);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 1978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 1979);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3951);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3952);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3953);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3954);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				['y', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1982);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', 'z', s @ ..] => {
							super::push(out, 1985);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 1986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 703);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3956);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3957);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 6025);
											s.is_empty()
										}
										_ => false,
									}
								}
								['b', s @ ..] => {
									super::push(out, 3958);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 6026);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3959);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 6027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 3960);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 3961);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											super::push(out, 6028);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 3962);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6029);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => super::t1::n175(s, out),
		['ą', s @ ..] => {
			super::push(out, 170);
			match s {
				[] => true,
				['i', s @ ..] => super::t1::n740(s, out),
				['r', s @ ..] => super::t1::n741(s, out),
				_ => false,
			}
		}
		['ę', s @ ..] => super::t1::n177(s, out),
		['ł', s @ ..] => super::t1::n178(s, out),
		['ź', s @ ..] => super::t1::n179(s, out),
		_ => false,
	}
}

/// Words ending -hać.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 174);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2000);
							s.is_empty()
						}
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3976);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 3977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2001);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2002);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3978);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3979);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3980);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3981);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 2003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2004);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 3982);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3983);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6030);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 2005);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3984);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3985);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 3986);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3987);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3988);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3989);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 3990);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 3991);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 3992);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2006);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 3993);
									s.is_empty()
								}
								['p', 'z', 's', '\u{1}', s @ ..] => {
									super::push(out, 3994);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 2007);
							match s {
								[] => true,
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 721);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2008);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2009);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2010);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3996);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 2011);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3998);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 3999);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 4000);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4001);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4002);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2013);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2014);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2015);
							match s {
								[] => true,
								['z', 'r', s @ ..] => {
									super::push(out, 4004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 724);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2016);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t1::n757(s, out),
				['y', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4012);
									s.is_empty()
								}
								['w', s @ ..] => super::t1::n4412(s, out),
								['y', s @ ..] => super::t1::n4413(s, out),
								['z', s @ ..] => super::t1::n4414(s, out),
								_ => false,
							}
						}
						['m', s @ ..] => super::t1::n2186(s, out),
						['p', s @ ..] => super::t1::n2187(s, out),
						['r', s @ ..] => super::t1::n2188(s, out),
						['s', s @ ..] => super::t1::n2189(s, out),
						_ => false,
					}
				}
				['z', s @ ..] => super::t1::n759(s, out),
				['ą', s @ ..] => super::t1::n760(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t1::n182(s, out),
		_ => false,
	}
}

/// Words ending -iać.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 729);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4017);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 6041);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 6042);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 4018);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6043);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 4019);
									match s {
										[] => true,
										['o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 6044);
											s.is_empty()
										}
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
					super::push(out, 730);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2031);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 731);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2033);
							match s {
								[] => true,
								['c', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2034);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2036);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2037);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 2038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2039);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 2040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['a', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2041);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2042);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 734);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				['ś', s @ ..] => {
					super::push(out, 737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['g', 'u', s @ ..] => {
							super::push(out, 2045);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2046);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4021);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 4023);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6045);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 4024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 740);
					match s {
						[] => true,
						['i', 'm', 'z', s @ ..] => {
							super::push(out, 2047);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4025);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4026);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 4027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2048);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4028);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 's', s @ ..] => {
					super::push(out, 741);
					match s {
						[] => true,
						['ę', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 2049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', s @ ..] => {
					super::push(out, 742);
					match s {
						[] => true,
						['w', 'z', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 743);
					match s {
						[] => true,
						['a', 'g', 'y', s @ ..] => {
							super::push(out, 2051);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 4029);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'a', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 745);
					s.is_empty()
				}
				['ź', 'a', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2054);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4031);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											super::push(out, 6046);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 4032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t1::n785(s, out),
				['u', s @ ..] => super::t1::n786(s, out),
				['y', s @ ..] => super::t1::n787(s, out),
				['ę', s @ ..] => {
					super::push(out, 753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n189(s, out),
		['w', s @ ..] => super::t1::n190(s, out),
		['z', s @ ..] => super::t1::n191(s, out),
		_ => false,
	}
}

/// Words ending -jać.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 2088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2089);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2090);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6063);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 6064);
											s.is_empty()
										}
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
					super::push(out, 775);
					match s {
						[] => true,
						['z', 's', '\u{1}', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2092);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2093);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4066);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4067);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6065);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 6066);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2095);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2096);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2097);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 4068);
									s.is_empty()
								}
								['y', s @ ..] => {
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 6067);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 6068);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['l', 'k', 's', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 783);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2102);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4069);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4070);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6069);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 6070);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2103);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2104);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4071);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4072);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'w', s @ ..] => {
							super::push(out, 2105);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'o', s @ ..] => {
							super::push(out, 2106);
							match s {
								[] => true,
								['r', 'o', s @ ..] => {
									super::push(out, 4075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2107);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2108);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2110);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2111);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2112);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kać.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2115);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 792);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 793);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4076);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4077);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 4078);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											super::push(out, 6071);
											s.is_empty()
										}
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
					super::push(out, 794);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 795);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2117);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2118);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4079);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4080);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4081);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6072);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 6073);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 797);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2119);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4082);
									s.is_empty()
								}
								['ś', 'w', s @ ..] => {
									super::push(out, 4083);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2120);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2121);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4084);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 6074);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 6075);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 4085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4086);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4087);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 4088);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4089);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4090);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 6076);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 6077);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 4091);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4092);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 6078);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2123);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4093);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 6079);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 7532);
													s.is_empty()
												}
												_ => false,
											}
										}
										['z', s @ ..] => {
											super::push(out, 6080);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4094);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 4095);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4096);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4097);
									s.is_empty()
								}
								['y', s @ ..] => {
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 6081);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 6082);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 4098);
									s.is_empty()
								}
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
			super::push(out, 194);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 2124);
							s.is_empty()
						}
						['r', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 2125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 801);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 2126);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2129);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['z', 'b', '\u{1}', s @ ..] => super::t1::n841(s, out),
				_ => false,
			}
		}
		['l', 'u', 'i', 'p', 'ś', '\u{1}', s @ ..] => super::t1::n201(s, out),
		['m', s @ ..] => super::t1::n202(s, out),
		['o', s @ ..] => super::t1::n203(s, out),
		['p', 'u', 'k', '\u{1}', s @ ..] => super::t1::n204(s, out),
		['r', s @ ..] => super::t1::n205(s, out),
		['s', s @ ..] => super::t1::n206(s, out),
		['t', s @ ..] => super::t1::n207(s, out),
		['u', s @ ..] => super::t1::n208(s, out),
		['y', s @ ..] => super::t1::n209(s, out),
		['z', s @ ..] => super::t1::n210(s, out),
		['ą', s @ ..] => super::t1::n211(s, out),
		['ć', s @ ..] => super::t1::n212(s, out),
		['ę', s @ ..] => super::t1::n213(s, out),
		['ł', '\u{1}', s @ ..] => super::t1::n214(s, out),
		_ => false,
	}
}

/// Words ending -lać.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 848);
					match s {
						[] => true,
						['r', 'e', 'i', 'p', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4123);
									s.is_empty()
								}
								['o', 'p', s @ ..] => {
									super::push(out, 4124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 849);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2185);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2186);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 850);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2188);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 851);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 852);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2190);
							match s {
								[] => true,
								['c', 'u', s @ ..] => {
									super::push(out, 4126);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6100);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 6101);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 2191);
							match s {
								[] => true,
								['o', 'd', s @ ..] => {
									super::push(out, 4127);
									s.is_empty()
								}
								['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 853);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 854);
					match s {
						[] => true,
						['n', 'a', 'r', 't', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2192);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2194);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 856);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 857);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 858);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 860);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 862);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4129);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4130);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7534);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 7535);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 7536);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 7537);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 214);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 863);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2198);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 864);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2200);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 217);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4131);
									s.is_empty()
								}
								['y', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'y', s @ ..] => {
							super::push(out, 2203);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 4133);
									s.is_empty()
								}
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
			super::push(out, 221);
			match s {
				[] => true,
				['e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 867);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2204);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4134);
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4135);
									s.is_empty()
								}
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
			super::push(out, 222);
			match s {
				[] => true,
				['e', 'i', 'w', 'ś', 'a', s @ ..] => {
					super::push(out, 868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 869);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 870);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2205);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2206);
							match s {
								[] => true,
								['z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4136);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 871);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 872);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => super::t1::n2384(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t1::n916(s, out),
				['m', s @ ..] => super::t1::n917(s, out),
				['p', s @ ..] => super::t1::n918(s, out),
				['t', s @ ..] => super::t1::n919(s, out),
				['z', s @ ..] => super::t1::n920(s, out),
				_ => false,
			}
		}
		['w', s @ ..] => super::t1::n230(s, out),
		['y', s @ ..] => super::t1::n231(s, out),
		['z', s @ ..] => super::t1::n232(s, out),
		['ś', s @ ..] => super::t1::n233(s, out),
		_ => false,
	}
}

/// Words ending -mać.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 886);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2215);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2216);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2217);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 2218);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['i', 'n', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2220);
							s.is_empty()
						}
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 891);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['r', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2222);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2223);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 894);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 895);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 896);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 2226);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 4143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['ł', '\u{1}', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nać.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 2228);
							s.is_empty()
						}
						['ż', '\u{1}', s @ ..] => {
							super::push(out, 2229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 903);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2230);
							match s {
								[] => true,
								['o', 'o', s @ ..] => {
									super::push(out, 4144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 905);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4145);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4146);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4147);
									match s {
										[] => true,
										['w', 'o', s @ ..] => {
											super::push(out, 6106);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 6107);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 6108);
											s.is_empty()
										}
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
					super::push(out, 906);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2231);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 4148);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2232);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['ó', 'r', '\u{1}', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 241);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 911);
					match s {
						[] => true,
						['e', 'i', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4149);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pać.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 912);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 915);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 916);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2236);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 4151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 917);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2238);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4152);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4153);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4154);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 4155);
									s.is_empty()
								}
								['z', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6109);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 6110);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 918);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 919);
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2240);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2241);
							s.is_empty()
						}
						['d', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2242);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2243);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2244);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2245);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4156);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 4157);
									s.is_empty()
								}
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
			super::push(out, 244);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 2246);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 2247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 2248);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 923);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['ż', '\u{1}', s @ ..] => {
							super::push(out, 2250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['a', 'z', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2251);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2252);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2253);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'z', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2255);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 925);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 928);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 933);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							super::push(out, 2257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 935);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2259);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2260);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4158);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4159);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['c', 'z', 's', 'd', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ć', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 944);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rać.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['g', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 945);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 946);
					match s {
						[] => true,
						['z', 'd', 'y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 947);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2267);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 950);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 951);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4160);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4161);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 4162);
									s.is_empty()
								}
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
			super::push(out, 255);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 952);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 953);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4163);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 954);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2270);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2271);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ż', s @ ..] => {
							super::push(out, 2272);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4165);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 4166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 955);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 956);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 957);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 958);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['z', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['g', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 962);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2274);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4167);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 4168);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4169);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 6111);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 6112);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 4170);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4171);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6113);
											s.is_empty()
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['p', s @ ..] => {
													super::push(out, 7540);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 7541);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 6114);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 6115);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 2275);
							match s {
								[] => true,
								['ś', s @ ..] => {
									super::push(out, 4172);
									match s {
										[] => true,
										['o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 6116);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2276);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 6117);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 6118);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 4173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2277);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4174);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => super::t1::n2459(s, out),
						['z', s @ ..] => {
							super::push(out, 2279);
							match s {
								[] => true,
								['d', 'w', s @ ..] => {
									super::push(out, 4183);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 4184);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											super::push(out, 6121);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t1::n1014(s, out),
				['p', s @ ..] => super::t1::n1015(s, out),
				['t', s @ ..] => super::t1::n1016(s, out),
				['ż', s @ ..] => super::t1::n1017(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n264(s, out),
		['h', s @ ..] => super::t1::n265(s, out),
		['m', s @ ..] => super::t1::n266(s, out),
		['o', s @ ..] => super::t1::n267(s, out),
		['p', s @ ..] => super::t1::n268(s, out),
		['s', s @ ..] => super::t1::n269(s, out),
		['t', s @ ..] => super::t1::n270(s, out),
		['u', s @ ..] => super::t1::n271(s, out),
		['y', s @ ..] => super::t1::n272(s, out),
		['ó', s @ ..] => super::t1::n273(s, out),
		_ => false,
	}
}

/// Words ending -sać.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 268);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 995);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 996);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 998);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2298);
							s.is_empty()
						}
						['d', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4194);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4195);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6122);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2299);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 2300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 269);
			match s {
				[] => true,
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4196);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 4197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2302);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2303);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 271);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['r', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1005);
					match s {
						[] => true,
						['o', 'k', 'a', s @ ..] => {
							super::push(out, 2304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1007);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1009);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4199);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 6123);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'o', s @ ..] => {
									super::push(out, 4200);
									s.is_empty()
								}
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
			super::push(out, 277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tać.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1010);
					match s {
						[] => true,
						['m', 'o', s @ ..] => {
							super::push(out, 2306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4201);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1011);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2308);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2309);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 4203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2310);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2311);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4204);
									match s {
										[] => true,
										['z', 'r', 'p', 'o', s @ ..] => {
											super::push(out, 6124);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 2312);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2313);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 4205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['a', 'h', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2314);
							s.is_empty()
						}
						['b', 'z', s @ ..] => {
							super::push(out, 2315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1013);
					match s {
						[] => true,
						['s', 'y', s @ ..] => {
							super::push(out, 2316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2317);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2318);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4206);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 4207);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4208);
									s.is_empty()
								}
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
			super::push(out, 279);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2319);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4209);
									match s {
										[] => true,
										['w', 'ś', 'y', 'w', '\u{1}', s @ ..] => {
											super::push(out, 6125);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ł', s @ ..] => {
									super::push(out, 4210);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 6126);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2321);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2322);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2324);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2325);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 2326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['e', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 283);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2327);
							match s {
								[] => true,
								['r', 'h', 'c', 'a', s @ ..] => {
									super::push(out, 4211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 2328);
							match s {
								[] => true,
								['h', 'c', 'a', s @ ..] => {
									super::push(out, 4212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => super::t1::n1074(s, out),
				['h', s @ ..] => super::t1::n1075(s, out),
				['i', s @ ..] => {
					super::push(out, 1020);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2334);
							match s {
								[] => true,
								['e', 'z', 'c', 'z', 's', 'a', s @ ..] => {
									super::push(out, 4223);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2335);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => super::t1::n1077(s, out),
				['m', s @ ..] => super::t1::n1078(s, out),
				['p', s @ ..] => super::t1::n1079(s, out),
				['r', s @ ..] => {
					super::push(out, 1024);
					match s {
						[] => true,
						['m', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t1::n291(s, out),
		['r', s @ ..] => {
			super::push(out, 286);
			s.is_empty()
		}
		['s', s @ ..] => super::t1::n293(s, out),
		['u', s @ ..] => super::t1::n294(s, out),
		['y', s @ ..] => super::t1::n295(s, out),
		['z', s @ ..] => super::t1::n296(s, out),
		['ą', s @ ..] => super::t1::n297(s, out),
		['ę', s @ ..] => super::t1::n298(s, out),
		['ł', s @ ..] => super::t1::n299(s, out),
		['ś', s @ ..] => super::t1::n300(s, out),
		_ => false,
	}
}

/// Words ending -wać.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1051);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2385);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2386);
							match s {
								[] => true,
								['o', 'o', s @ ..] => {
									super::push(out, 4253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2387);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2388);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2389);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2390);
							match s {
								[] => true,
								['w', 'o', s @ ..] => {
									super::push(out, 4254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 2391);
							match s {
								[] => true,
								['o', 'w', s @ ..] => {
									super::push(out, 4255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1052);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 2392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1053);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2393);
							s.is_empty()
						}
						['s', 'e', s @ ..] => {
							super::push(out, 2394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1054);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2395);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2396);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4256);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1055);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4257);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											super::push(out, 6153);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4258);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4259);
									match s {
										[] => true,
										['z', 'r', 'p', 'a', s @ ..] => {
											super::push(out, 6154);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4260);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 4261);
									s.is_empty()
								}
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
			super::push(out, 296);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2397);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2398);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4262);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 6155);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 6156);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 4263);
									s.is_empty()
								}
								['ć', s @ ..] => {
									super::push(out, 4264);
									s.is_empty()
								}
								['ś', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6157);
											s.is_empty()
										}
										['o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 6158);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 6159);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2399);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 4265);
									match s {
										[] => true,
										['z', s @ ..] => {
											super::push(out, 6160);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ś', s @ ..] => {
									super::push(out, 4266);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2400);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4267);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4268);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 4269);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6161);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 6162);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2401);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2402);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4270);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 6163);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 2403);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4271);
									s.is_empty()
								}
								['d', 'o', 'p', 's', s @ ..] => {
									super::push(out, 4272);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 4273);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											super::push(out, 6164);
											s.is_empty()
										}
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
					super::push(out, 1057);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2404);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4274);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4275);
									s.is_empty()
								}
								['o', 'p', s @ ..] => {
									super::push(out, 4276);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2405);
							s.is_empty()
						}
						['h', 'c', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2406);
							s.is_empty()
						}
						['o', 'b', s @ ..] => super::t1::n2598(s, out),
						_ => false,
					}
				}
				['z', s @ ..] => super::t1::n1116(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n303(s, out),
		['o', s @ ..] => super::t1::n304(s, out),
		['r', s @ ..] => super::t1::n305(s, out),
		['t', s @ ..] => super::t1::n306(s, out),
		['u', s @ ..] => super::t1::n307(s, out),
		['y', s @ ..] => super::t1::n308(s, out),
		['z', s @ ..] => super::t1::n309(s, out),
		_ => false,
	}
}

/// Words ending -zać.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1115);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2727);
							s.is_empty()
						}
						['o', 'p', s @ ..] => {
							super::push(out, 2728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1117);
					match s {
						[] => true,
						['b', 'z', s @ ..] => {
							super::push(out, 2730);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2731);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2732);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2733);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 4945);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2734);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4947);
									s.is_empty()
								}
								['k', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 6965);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 6966);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 6967);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'o', s @ ..] => {
							super::push(out, 2735);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4948);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 4949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ł', s @ ..] => {
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4950);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 4951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1118);
					match s {
						[] => true,
						['a', 'i', 'w', 'ś', 'y', 'w', s @ ..] => {
							super::push(out, 2736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2737);
							s.is_empty()
						}
						['l', 'a', 'k', s @ ..] => {
							super::push(out, 2738);
							match s {
								[] => true,
								['o', 'o', s @ ..] => {
									super::push(out, 4953);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 2739);
							match s {
								[] => true,
								['r', 'p', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6968);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 6969);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 8097);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1120);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4954);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4955);
									match s {
										[] => true,
										['s', 'o', s @ ..] => {
											super::push(out, 6970);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2741);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1123);
					match s {
						[] => true,
						['d', 'z', 'b', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2743);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2744);
							s.is_empty()
						}
						['ż', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 2745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4956);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4957);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 6971);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 8098);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 8099);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['b', s @ ..] => {
									super::push(out, 4958);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4959);
									match s {
										[] => true,
										['i', 'p', s @ ..] => {
											super::push(out, 6972);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 8100);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4960);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 4961);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 6973);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 6974);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ę', s @ ..] => {
									super::push(out, 4962);
									match s {
										[] => true,
										['g', s @ ..] => super::t1::n7753(s, out),
										['z', s @ ..] => super::t1::n7754(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ą', s @ ..] => super::t1::n1184(s, out),
				['ę', s @ ..] => super::t1::n1185(s, out),
				['ń', s @ ..] => super::t1::n1186(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t1::n312(s, out),
		['e', s @ ..] => super::t1::n313(s, out),
		['i', s @ ..] => super::t1::n314(s, out),
		['r', s @ ..] => super::t1::n315(s, out),
		['s', s @ ..] => super::t1::n316(s, out),
		['u', s @ ..] => super::t1::n317(s, out),
		['y', s @ ..] => super::t1::n318(s, out),
		['ą', s @ ..] => super::t1::n319(s, out),
		['ł', s @ ..] => super::t1::n320(s, out),
		_ => false,
	}
}

/// Words ending -łać.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['i', 'z', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2790);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2791);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2792);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2793);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2794);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2795);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 2796);
							match s {
								[] => true,
								['ó', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 5006);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2797);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['o', 'i', 's', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2799);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2801);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 2802);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1154);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2804);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2805);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['u', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2807);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 2808);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2809);
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
				['\u{1}', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1156);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 2810);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5007);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 5008);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2811);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5009);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['d', 'o', 'p', s @ ..] => {
							super::push(out, 2813);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5010);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'y', 't', 'u', '\u{1}', s @ ..] => {
			super::push(out, 322);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1161);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 2814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żać.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1163);
					match s {
						[] => true,
						['s', 'o', s @ ..] => {
							super::push(out, 2815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1165);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2816);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['ż', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2818);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5012);
									s.is_empty()
								}
								['j', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7008);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 7009);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 7010);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 7011);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 7012);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 7013);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 7014);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 7015);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 7016);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 8104);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 327);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1167);
					match s {
						[] => true,
						['b', 'u', s @ ..] => {
							super::push(out, 2819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', 'k', 's', 'u', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2822);
							match s {
								[] => true,
								['y', 'w', 'z', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7017);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 7018);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2823);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ieć.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2825);
							s.is_empty()
						}
						['r', 'g', 'z', s @ ..] => {
							super::push(out, 2826);
							s.is_empty()
						}
						['ł', 's', 'a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2828);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'g', 'z', s @ ..] => {
							super::push(out, 2829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'z', 'r', 'g', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1176);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1177);
					match s {
						[] => true,
						['r', 'e', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 2830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', 'd', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1179);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5014);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5015);
									match s {
										[] => true,
										['d', 'o', 'l', 'z', s @ ..] => {
											super::push(out, 7019);
											s.is_empty()
										}
										['g', 'o', 'r', 'z', s @ ..] => {
											super::push(out, 7020);
											s.is_empty()
										}
										['k', 'a', 'r', 'z', s @ ..] => {
											super::push(out, 7021);
											s.is_empty()
										}
										['n', 'z', 'i', 'l', 'b', 'z', s @ ..] => {
											super::push(out, 7022);
											s.is_empty()
										}
										['p', 'o', 'r', 'h', 'c', 's', s @ ..] => {
											super::push(out, 7023);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 7024);
											s.is_empty()
										}
										['ł', 'r', 'a', 'k', 's', s @ ..] => {
											super::push(out, 7025);
											s.is_empty()
										}
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
					super::push(out, 1180);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5016);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5017);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5018);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5019);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5020);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5021);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5022);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5023);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 7026);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 7027);
											s.is_empty()
										}
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
					super::push(out, 1182);
					match s {
						[] => true,
						['l', 'o', 'p', 's', 'o', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5024);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['i', 'd', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1184);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5026);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1185);
					match s {
						[] => true,
						['ł', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5028);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ó', 'r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1186);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 1187);
					match s {
						[] => true,
						['l', 'd', 'y', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1188);
					s.is_empty()
				}
				['ś', s @ ..] => {
					super::push(out, 1189);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2833);
							match s {
								[] => true,
								['i', 'w', 'e', 'i', 'n', 'z', s @ ..] => {
									super::push(out, 5030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ę', s @ ..] => {
							match s {
								[] => true,
								['g', 'z', '\u{1}', s @ ..] => {
									super::push(out, 5031);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5032);
									match s {
										[] => true,
										['r', 'h', 'c', 'a', 'z', '\u{1}', s @ ..] => {
											super::push(out, 7028);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t1::n341(s, out),
		['n', s @ ..] => super::t1::n342(s, out),
		['p', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2865);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 2866);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t1::n5660(s, out),
								['y', s @ ..] => super::t1::n5661(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t1::n1288(s, out),
				['r', s @ ..] => super::t1::n1289(s, out),
				['u', s @ ..] => super::t1::n1290(s, out),
				['y', s @ ..] => super::t1::n1291(s, out),
				['ą', 'k', 's', s @ ..] => super::t1::n1292(s, out),
				['ę', s @ ..] => super::t1::n1293(s, out),
				['ś', 'u', '\u{1}', s @ ..] => super::t1::n1294(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n344(s, out),
		['w', s @ ..] => super::t1::n345(s, out),
		['z', s @ ..] => super::t1::n346(s, out),
		_ => false,
	}
}

/// Words ending -zeć.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1247);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['f', 's', '\u{1}', s @ ..] => {
									super::push(out, 5149);
									s.is_empty()
								}
								['o', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 5150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'i', 'z', 'd', 'z', s @ ..] => {
							super::push(out, 2915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1248);
					match s {
						[] => true,
						['e', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 2916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2917);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2918);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2919);
							match s {
								[] => true,
								['r', 'k', 's', 'a', s @ ..] => {
									super::push(out, 5152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1250);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 2920);
							match s {
								[] => true,
								['k', 'a', s @ ..] => {
									super::push(out, 5153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 2921);
							match s {
								[] => true,
								['d', 'z', s @ ..] => {
									super::push(out, 5154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1251);
					match s {
						[] => true,
						['i', 'm', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 5155);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['e', 'i', 'n', 'z', s @ ..] => {
							super::push(out, 2922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['a', 'h', s @ ..] => {
							super::push(out, 2923);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2924);
							s.is_empty()
						}
						['u', 'b', s @ ..] => {
							super::push(out, 2925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['o', 'i', 'w', 'z', s @ ..] => {
							super::push(out, 2926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1255);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2927);
							s.is_empty()
						}
						['b', 'a', s @ ..] => {
							super::push(out, 2928);
							s.is_empty()
						}
						['h', 'a', s @ ..] => {
							super::push(out, 2929);
							s.is_empty()
						}
						['m', 'a', s @ ..] => {
							super::push(out, 2930);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2932);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2933);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5157);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5158);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5159);
									match s {
										[] => true,
										['e', 't', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8134);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 8135);
													s.is_empty()
												}
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
							super::push(out, 2934);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2935);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2936);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5160);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5161);
									match s {
										[] => true,
										['k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8136);
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 8137);
													match s {
														[] => true,
														['n', s @ ..] => super::t1::n9462(s, out),
														['z', s @ ..] => super::t1::n9463(s, out),
														_ => false,
													}
												}
												['o', s @ ..] => {
													super::push(out, 8138);
													match s {
														[] => true,
														['p', s @ ..] => super::t1::n9464(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1257);
					match s {
						[] => true,
						['s', s @ ..] => super::t1::n3215(s, out),
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 1258);
					match s {
						[] => true,
						['l', 'ś', s @ ..] => {
							super::push(out, 2937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t1::n5737(s, out),
								_ => false,
							}
						}
						['z', 'r', 'b', 'a', s @ ..] => super::t1::n3217(s, out),
						_ => false,
					}
				}
				['ł', s @ ..] => super::t1::n1346(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t1::n364(s, out),
		['s', s @ ..] => super::t1::n365(s, out),
		_ => false,
	}
}

/// Words ending -żeć.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1275);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2982);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2983);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5212);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7144);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 5213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2984);
							s.is_empty()
						}
						['o', s @ ..] => {
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
				_ => false,
			}
		}
		['l', 'e', 'z', '\u{1}', s @ ..] => {
			super::push(out, 359);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['b', 'u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2990);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2991);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1276);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1277);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2993);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1278);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2995);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1280);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zebić.
pub(crate) fn n1385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1293);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 5221);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lbić.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['e', 'i', 'w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3006);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3007);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -obić.
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1294);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3008);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3009);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5223);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5224);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 7148);
									s.is_empty()
								}
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
			super::push(out, 1296);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1297);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3010);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5225);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3011);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3012);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['o', 'p', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5228);
							s.is_empty()
						}
						['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1299);
			match s {
				[] => true,
				['ż', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5230);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rbić.
pub(crate) fn n384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 3013);
					s.is_empty()
				}
				['k', 's', '\u{1}', s @ ..] => {
					super::push(out, 3014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'z', 'c', 'z', 's', '\u{1}', s @ ..] => {
			super::push(out, 1300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ubić.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1301);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1303);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3016);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3017);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 3018);
					s.is_empty()
				}
				['ś', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5232);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1304);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1305);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3019);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 1306);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ybić.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1307);
			match s {
				[] => true,
				['c', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1308);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5234);
							s.is_empty()
						}
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1309);
			s.is_empty()
		}
		['z', 'r', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3023);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3024);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zbić.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 1310);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ąbić.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3025);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5236);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5237);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ębić.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 380);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3026);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1312);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 3028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1313);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 3029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1314);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3031);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5240);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ńbić.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 381);
	match s {
		[] => true,
		['a', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3032);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3033);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -źbić.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	match s {
		[] => true,
		['e', 'z', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3034);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3035);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cić.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1316);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3036);
							s.is_empty()
						}
						['i', 'l', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3037);
							s.is_empty()
						}
						['o', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5242);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5243);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5244);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1317);
					match s {
						[] => true,
						['c', 'e', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5245);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5246);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1318);
					match s {
						[] => true,
						['i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 1319);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1320);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3039);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1321);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3041);
							s.is_empty()
						}
						['o', 'n', 'i', 'w', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3043);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3044);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5247);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 5248);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5249);
									s.is_empty()
								}
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
			super::push(out, 384);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1323);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3046);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5250);
									s.is_empty()
								}
								['ś', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7149);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 7150);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 8148);
													s.is_empty()
												}
												['z', s @ ..] => {
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
						['n', s @ ..] => {
							super::push(out, 3047);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 5252);
									s.is_empty()
								}
								['ś', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7151);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 7152);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 7153);
											match s {
												[] => true,
												['p', s @ ..] => {
													super::push(out, 8150);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1324);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 3049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1325);
					match s {
						[] => true,
						['z', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5254);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5255);
									s.is_empty()
								}
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
			super::push(out, 385);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3050);
							match s {
								[] => true,
								['l', 'z', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7154);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 7155);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1326);
					match s {
						[] => true,
						['a', 's', 'k', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1327);
					s.is_empty()
				}
				['h', 'c', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3053);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1328);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3055);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1329);
					match s {
						[] => true,
						['z', 'r', 'g', 'y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1330);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t1::n1432(s, out),
				['s', s @ ..] => super::t1::n1433(s, out),
				['ł', s @ ..] => super::t1::n1434(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t1::n400(s, out),
		['r', s @ ..] => super::t1::n401(s, out),
		['u', s @ ..] => super::t1::n402(s, out),
		['y', s @ ..] => super::t1::n403(s, out),
		['z', s @ ..] => super::t1::n404(s, out),
		['ó', s @ ..] => super::t1::n405(s, out),
		['ą', s @ ..] => super::t1::n406(s, out),
		['ę', s @ ..] => super::t1::n407(s, out),
		['ł', s @ ..] => super::t1::n408(s, out),
		['ś', s @ ..] => super::t1::n409(s, out),
		_ => false,
	}
}

/// Words ending -eić.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1365);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3107);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fić.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3109);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3110);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3111);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 5292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 3112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 401);
			s.is_empty()
		}
		['y', 's', '\u{1}', s @ ..] => {
			super::push(out, 402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lić.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 403);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1366);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1367);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3113);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3114);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1368);
					match s {
						[] => true,
						['u', 'o', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5293);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1369);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1370);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3116);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 3117);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 3118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1371);
					match s {
						[] => true,
						['o', 'k', 's', 'o', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5295);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3119);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3120);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5297);
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
						['s', s @ ..] => {
							super::push(out, 3121);
							s.is_empty()
						}
						['z', 's', 'y', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1373);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3123);
							s.is_empty()
						}
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3124);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3125);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 5298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 's', '\u{1}', s @ ..] => {
					super::push(out, 1374);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 1375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3126);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 404);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1376);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1378);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1382);
					match s {
						[] => true,
						['u', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 3130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1383);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3131);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5299);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5300);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1384);
					match s {
						[] => true,
						['r', 'k', 's', '\u{1}', s @ ..] => {
							super::push(out, 3133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1385);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 3134);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ą', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['ż', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5302);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ę', s @ ..] => {
					super::push(out, 1387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1388);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3136);
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5304);
									s.is_empty()
								}
								['ś', '\u{1}', s @ ..] => {
									super::push(out, 5305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3137);
							match s {
								[] => true,
								['e', 'r', 'e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5307);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5308);
									match s {
										[] => true,
										['r', 't', '\u{1}', s @ ..] => {
											super::push(out, 7173);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ś', s @ ..] => {
									super::push(out, 5309);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5310);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5312);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5313);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5314);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 5315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1389);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1390);
					s.is_empty()
				}
				['z', s @ ..] => super::t1::n1495(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n419(s, out),
		['i', s @ ..] => super::t1::n420(s, out),
		['k', s @ ..] => super::t1::n421(s, out),
		['o', s @ ..] => super::t1::n422(s, out),
		['p', s @ ..] => super::t1::n423(s, out),
		['r', s @ ..] => super::t1::n424(s, out),
		['t', s @ ..] => super::t1::n425(s, out),
		['u', s @ ..] => super::t1::n426(s, out),
		['y', s @ ..] => super::t1::n427(s, out),
		['ś', s @ ..] => super::t1::n428(s, out),
		['ź', s @ ..] => super::t1::n429(s, out),
		_ => false,
	}
}

/// Words ending -mić.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1430);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['a', 'n', 'z', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1434);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1435);
					match s {
						[] => true,
						['a', 'ł', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5340);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 3184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3185);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1438);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3187);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5342);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5343);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['ł', 't', '\u{1}', s @ ..] => {
					super::push(out, 1440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3188);
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 5344);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 3189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ć', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1442);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nić.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3190);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3191);
							match s {
								[] => true,
								['ł', 'a', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7181);
											s.is_empty()
										}
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 8159);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 8160);
													s.is_empty()
												}
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
							super::push(out, 3192);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											super::push(out, 7182);
											s.is_empty()
										}
										['y', 'w', '\u{1}', s @ ..] => {
											super::push(out, 7183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 3193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1447);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3194);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 3195);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3196);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 7184);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 7185);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3197);
							match s {
								[] => true,
								['t', 'o', s @ ..] => {
									super::push(out, 5345);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1448);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['z', 's', 'a', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5346);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5347);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['ł', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['r', 'h', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 3200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['z', 'r', 't', 'o', 'p', 'e', 'i', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1453);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 1454);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 3202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1456);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1457);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1458);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3203);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5349);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5350);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3204);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3205);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7186);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 7187);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 7188);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 431);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3206);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1459);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 3208);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5351);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 5352);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5353);
									s.is_empty()
								}
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5354);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5355);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 5356);
									match s {
										[] => true,
										['w', s @ ..] => {
											super::push(out, 7189);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8161);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3209);
							s.is_empty()
						}
						['s', 'e', 'j', '\u{1}', s @ ..] => super::t1::n3528(s, out),
						['w', 'r', 'e', 'z', 'c', '\u{1}', s @ ..] => super::t1::n3529(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t1::n1567(s, out),
				['z', s @ ..] => super::t1::n1568(s, out),
				['ż', s @ ..] => super::t1::n1569(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t1::n443(s, out),
		['h', s @ ..] => super::t1::n444(s, out),
		['i', s @ ..] => super::t1::n445(s, out),
		['j', s @ ..] => super::t1::n446(s, out),
		['k', s @ ..] => super::t1::n447(s, out),
		['l', s @ ..] => super::t1::n448(s, out),
		['m', s @ ..] => super::t1::n449(s, out),
		['n', s @ ..] => super::t1::n450(s, out),
		['o', s @ ..] => super::t1::n451(s, out),
		['p', s @ ..] => super::t1::n452(s, out),
		['r', s @ ..] => super::t1::n453(s, out),
		['t', s @ ..] => super::t1::n454(s, out),
		['u', s @ ..] => super::t1::n455(s, out),
		['w', s @ ..] => super::t1::n456(s, out),
		['y', s @ ..] => super::t1::n457(s, out),
		['z', s @ ..] => super::t1::n458(s, out),
		['ł', s @ ..] => super::t1::n459(s, out),
		['ś', s @ ..] => super::t1::n460(s, out),
		['ź', s @ ..] => super::t1::n461(s, out),
		['ż', s @ ..] => super::t1::n462(s, out),
		_ => false,
	}
}

/// Words ending -oić.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			super::push(out, 452);
			s.is_empty()
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 453);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1507);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5396);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['g', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 457);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1512);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3280);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3281);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5398);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5399);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5400);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 7198);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7199);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 5401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3282);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3284);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3285);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5402);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5403);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5404);
									s.is_empty()
								}
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
			super::push(out, 458);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', '\u{1}', s @ ..] => {
			super::push(out, 459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pić.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 460);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3287);
							s.is_empty()
						}
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 3288);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3289);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1519);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5405);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1521);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3292);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 3293);
							s.is_empty()
						}
						['ś', '\u{1}', s @ ..] => {
							super::push(out, 3294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3295);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5407);
									s.is_empty()
								}
								['z', 's', '\u{1}', s @ ..] => {
									super::push(out, 5408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3296);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 5409);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7200);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 7201);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 5410);
									s.is_empty()
								}
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
			super::push(out, 464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1526);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1528);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1531);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3298);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3299);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5411);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5412);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5413);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3300);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1533);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['ą', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3302);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1535);
					s.is_empty()
				}
				['d', 'l', s @ ..] => {
					super::push(out, 1536);
					match s {
						[] => true,
						['ó', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1537);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3305);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3306);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1538);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5416);
									s.is_empty()
								}
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'c', 'e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1539);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1540);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3308);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 3309);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 5418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 3310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 470);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 471);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 472);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1541);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1542);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5419);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t1::n1660(s, out),
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 474);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				['t', s @ ..] => super::t1::n1662(s, out),
				['z', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => super::t1::n486(s, out),
		['ś', s @ ..] => {
			super::push(out, 476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sić.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['b', 'ł', 'e', 'i', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
				['g', s @ ..] => {
					super::push(out, 1547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3316);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 3317);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 3318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1548);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1550);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5430);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5431);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5432);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1551);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 3320);
							s.is_empty()
						}
						['t', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 3321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1552);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3322);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 3323);
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
				['i', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5434);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3325);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 3326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1554);
					s.is_empty()
				}
				['z', 'r', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3327);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3328);
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
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3329);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1555);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 1556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 479);
			s.is_empty()
		}
		['m', 'a', 'ł', 't', '\u{1}', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3331);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3332);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3333);
							match s {
								[] => true,
								['k', 'z', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5435);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'r', 'a', 't', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1558);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1559);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3335);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5436);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7205);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 5437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3336);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5438);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5439);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'ę', 'i', 'm', 'z', s @ ..] => {
					super::push(out, 1561);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1562);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3337);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1563);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 1564);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1565);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1566);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1567);
					match s {
						[] => true,
						['z', 'r', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5441);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5442);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 5443);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 5444);
									s.is_empty()
								}
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
			super::push(out, 483);
			match s {
				[] => true,
				['r', 'p', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 484);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 485);
			s.is_empty()
		}
		_ => false,
	}
}

