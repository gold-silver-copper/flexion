use flexion::Path;

/// Words ending -io.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 868);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2640);
					match s {
						[] => true,
						['c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2641);
					s.is_empty()
				}
				['n', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2642);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2643);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5591);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 8221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'c', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2645);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8222);
									match s {
										[] => true,
										['c', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 9980);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'b', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 8223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2646);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2647);
					match s {
						[] => true,
						['g', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2648);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['c', 'o', 'n', s @ ..] => {
									match s {
										[] => true,
										['e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 9981);
											s.is_empty()
										}
										['i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 9982);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 8224);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['e', 'f', '\u{1}', s @ ..] => {
													super::push(out, 10800);
													s.is_empty()
												}
												['i', 'f', '\u{1}', s @ ..] => {
													super::push(out, 10801);
													s.is_empty()
												}
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
					super::push(out, 2649);
					match s {
						[] => true,
						['g', 'a', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2650);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2651);
					match s {
						[] => true,
						['i', 'v', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5594);
							s.is_empty()
						}
						['s', 'e', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2652);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 't', 's', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8225);
									s.is_empty()
								}
								['l', 'o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8226);
									s.is_empty()
								}
								['n', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', 'o', 'p', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jo.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['\u{12}', 'o', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 871);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2653);
					match s {
						[] => true,
						['b', 'a', 'y', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5597);
							s.is_empty()
						}
						['v', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', 's', 'u', 'k', 'o', 'k', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ko.
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 872);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2655);
					s.is_empty()
				}
				['e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2656);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2657);
					match s {
						[] => true,
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 5599);
							s.is_empty()
						}
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2658);
					s.is_empty()
				}
				['e', 'b', 'a', 'k', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2659);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5601);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5602);
							match s {
								[] => true,
								['a', 'h', 'u', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8228);
									s.is_empty()
								}
								['r', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5603);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8230);
									match s {
										[] => true,
										['b', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 9983);
											s.is_empty()
										}
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
					super::push(out, 2660);
					match s {
						[] => true,
						['e', 'n', 'a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5604);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5605);
							s.is_empty()
						}
						['o', 'j', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5606);
							s.is_empty()
						}
						['u', 'k', 'a', 'l', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', 'h', 'c', 'u', 's', 't', '\u{1}', s @ ..] => {
							super::push(out, 5608);
							s.is_empty()
						}
						['s', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lo.
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 875);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2661);
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 5610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'a', s @ ..] => {
					super::push(out, 2662);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 5611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2663);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 877);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2665);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 5612);
							match s {
								[] => true,
								['e', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8231);
									s.is_empty()
								}
								['f', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8232);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2666);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 5614);
							match s {
								[] => true,
								['s', 't', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8233);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5615);
							match s {
								[] => true,
								['t', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2667);
					match s {
						[] => true,
						['m', 's', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 5616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2668);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5617);
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									super::push(out, 8235);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 9984);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8236);
									s.is_empty()
								}
								['i', 'n', 'i', 'm', 'm', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8237);
									s.is_empty()
								}
								['n', 'a', 's', 's', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8238);
									s.is_empty()
								}
								['r', 'a', 'b', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8239);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 'g', 'g', 'i', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 9985);
											s.is_empty()
										}
										['n', 'u', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9986);
											s.is_empty()
										}
										['t', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9987);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5618);
							match s {
								[] => true,
								['d', 'a', s @ ..] => {
									super::push(out, 8240);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 8241);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 8242);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9988);
											match s {
												[] => true,
												['l', '\u{1}', s @ ..] => {
													super::push(out, 10802);
													s.is_empty()
												}
												['r', '\u{1}', s @ ..] => {
													super::push(out, 10803);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 9989);
											s.is_empty()
										}
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
					super::push(out, 2669);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5619);
							s.is_empty()
						}
						['c', 'c', 'o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5620);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5621);
							match s {
								[] => true,
								['d', 'u', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 8243);
									s.is_empty()
								}
								['v', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5622);
							s.is_empty()
						}
						['r', 'a', 't', 's', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 5623);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2670);
					match s {
						[] => true,
						['b', 'm', 'a', 't', 'c', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5625);
							s.is_empty()
						}
						['k', 's', 'a', 'n', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 878);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 879);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5627);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 5628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'j', '\u{1}', s @ ..] => {
					super::push(out, 2671);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['a', 'h', 's', 'i', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 2673);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2674);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2675);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5629);
							match s {
								[] => true,
								['g', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 8245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 5630);
							s.is_empty()
						}
						['k', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5631);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5632);
							match s {
								[] => true,
								['s', 'i', 't', 'r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8246);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'm', 'a', 'm', 'a', 'y', '\u{1}', s @ ..] => {
					super::push(out, 2676);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2677);
					match s {
						[] => true,
						['g', 'i', 'h', 'c', 'u', 's', 't', '\u{1}', s @ ..] => {
							super::push(out, 5633);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 5634);
							match s {
								[] => true,
								['a', 'n', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8247);
									s.is_empty()
								}
								['e', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8248);
									s.is_empty()
								}
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

/// Words ending -no.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 881);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2678);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2679);
					match s {
						[] => true,
						['v', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2680);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2681);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5636);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 5637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2682);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5638);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 8249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 882);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2683);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5639);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8250);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 9990);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 8251);
									match s {
										[] => true,
										['o', 'v', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 10804);
													s.is_empty()
												}
												['o', s @ ..] => {
													match s {
														[] => true,
														['t', 'a', 'r', 't', 's', '\u{1}', s @ ..] => super::t3::n12350(s, out),
														['y', 'r', 'c', '\u{1}', s @ ..] => super::t3::n12351(s, out),
														_ => false,
													}
												}
												['r', 'e', 'p', s @ ..] => {
													match s {
														[] => true,
														['u', 's', '\u{1}', s @ ..] => super::t3::n12352(s, out),
														['y', 'h', '\u{1}', s @ ..] => super::t3::n12353(s, out),
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
							super::push(out, 5640);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5641);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5642);
							match s {
								[] => true,
								['m', 'y', 't', '\u{1}', s @ ..] => {
									super::push(out, 8252);
									s.is_empty()
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
						['c', s @ ..] => {
							super::push(out, 5643);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8253);
									s.is_empty()
								}
								['i', 't', 't', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 8254);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8255);
									match s {
										[] => true,
										['a', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 9991);
											s.is_empty()
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['c', 'c', 'o', 'b', '\u{1}', s @ ..] => {
													super::push(out, 10805);
													s.is_empty()
												}
												['r', 'e', 'p', s @ ..] => {
													match s {
														[] => true,
														['e', 'p', '\u{1}', s @ ..] => super::t3::n12354(s, out),
														['p', 'e', 'p', '\u{1}', s @ ..] => super::t3::n12355(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'i', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 5644);
							s.is_empty()
						}
						['h', 'c', 'c', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5645);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'c', 'c', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8257);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 8258);
									match s {
										[] => true,
										['e', 't', 'r', 'o', 't', '\u{1}', s @ ..] => {
											super::push(out, 9992);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => super::t3::n6104(s, out),
						['n', s @ ..] => {
							super::push(out, 5647);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8262);
									s.is_empty()
								}
								['g', 'o', 'l', 'o', 'b', '\u{1}', s @ ..] => super::t3::n9036(s, out),
								_ => false,
							}
						}
						['p', 'l', 'a', '\u{1}', s @ ..] => super::t3::n6106(s, out),
						['r', s @ ..] => super::t3::n6107(s, out),
						['s', 's', 'i', 'r', 'g', '\u{1}', s @ ..] => super::t3::n6108(s, out),
						['t', s @ ..] => super::t3::n6109(s, out),
						['v', 'a', 't', 't', 'o', '\u{1}', s @ ..] => super::t3::n6110(s, out),
						['z', s @ ..] => super::t3::n6111(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t3::n2893(s, out),
				['r', 'o', s @ ..] => super::t3::n2894(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oo.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 284);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 883);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 884);
			match s {
				[] => true,
				['d', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 885);
			match s {
				[] => true,
				['m', 'l', 'a', 'u', 'q', 'o', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 2688);
					s.is_empty()
				}
				['s', 'a', 't', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2689);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -po.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 886);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['m', 'a', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2690);
					s.is_empty()
				}
				['n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2691);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2692);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5657);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 8269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', 'e', s @ ..] => {
			super::push(out, 887);
			match s {
				[] => true,
				['p', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 2693);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 888);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2694);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'r', 'e', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 2695);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2696);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2697);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2698);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 5660);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8270);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 5661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5662);
							s.is_empty()
						}
						['i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2699);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 5664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2700);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2701);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2703);
					match s {
						[] => true,
						['c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5666);
							s.is_empty()
						}
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2704);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2705);
					match s {
						[] => true,
						['d', 'a', s @ ..] => {
							super::push(out, 5668);
							s.is_empty()
						}
						['e', 'u', 'q', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5669);
							s.is_empty()
						}
						['f', 'f', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5670);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 5671);
							match s {
								[] => true,
								['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8272);
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8273);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 8274);
									s.is_empty()
								}
								['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8275);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8276);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 10002);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8277);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 10003);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8278);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5672);
							match s {
								[] => true,
								['a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 8279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5673);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5674);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5675);
							s.is_empty()
						}
						['z', 'n', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2706);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5677);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 5678);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8280);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10004);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2707);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5680);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5681);
							match s {
								[] => true,
								['o', 'k', 'o', 'p', 'u', '\u{1}', s @ ..] => {
									super::push(out, 8281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2708);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5682);
							match s {
								[] => true,
								['i', 'm', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8282);
									s.is_empty()
								}
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

/// Words ending -so.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 891);
			match s {
				[] => true,
				['p', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2710);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 5683);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5684);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2711);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 893);
			match s {
				[] => true,
				['g', 'n', 'o', 'e', 'y', 'p', 'e', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 2713);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2714);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2715);
					match s {
						[] => true,
						['f', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 5685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2716);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 5686);
							match s {
								[] => true,
								['l', 'a', 'f', 'f', 'u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2717);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5687);
							match s {
								[] => true,
								['i', 'h', 'c', 'u', '\u{1}', s @ ..] => {
									super::push(out, 8285);
									s.is_empty()
								}
								['o', 't', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 8286);
									s.is_empty()
								}
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

/// Words ending -to.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 894);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 895);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2718);
					s.is_empty()
				}
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2719);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2720);
					match s {
						[] => true,
						['a', 'm', 's', '\u{1}', s @ ..] => {
							super::push(out, 5688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2721);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 5689);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5690);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5691);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2722);
					s.is_empty()
				}
				['s', 'a', s @ ..] => {
					super::push(out, 2723);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 5692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2724);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5693);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 8288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2726);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5694);
							match s {
								[] => true,
								['o', 'v', 'v', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5695);
							match s {
								[] => true,
								['o', 'f', 'f', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8290);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5696);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 8291);
									match s {
										[] => true,
										['e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10005);
											s.is_empty()
										}
										['o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10006);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'f', 'f', 'a', 'r', 'g', 's', '\u{1}', s @ ..] => {
							super::push(out, 5697);
							s.is_empty()
						}
						['j', 'n', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5698);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 8292);
									s.is_empty()
								}
								['l', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5699);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8294);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 10007);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 10008);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5700);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5701);
							match s {
								[] => true,
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 8295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5702);
							match s {
								[] => true,
								['e', 't', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 8296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5703);
							match s {
								[] => true,
								['m', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 8297);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8298);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 10009);
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'u', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'u', 'q', 's', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2727);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2728);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5704);
							match s {
								[] => true,
								['e', 'k', 'u', 's', 't', '\u{1}', s @ ..] => {
									super::push(out, 8300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5705);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 8301);
									match s {
										[] => true,
										['a', s @ ..] => super::t3::n11004(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', 'o', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5706);
							s.is_empty()
						}
						['l', s @ ..] => super::t3::n6166(s, out),
						['n', 'o', 'b', '\u{1}', s @ ..] => super::t3::n6167(s, out),
						['r', 'o', s @ ..] => super::t3::n6168(s, out),
						['t', s @ ..] => super::t3::n6169(s, out),
						['u', s @ ..] => super::t3::n6170(s, out),
						_ => false,
					}
				}
				['k', 'a', 'l', 'i', 'f', '\u{1}', s @ ..] => super::t3::n2938(s, out),
				['l', s @ ..] => super::t3::n2939(s, out),
				['n', s @ ..] => super::t3::n2940(s, out),
				['o', s @ ..] => super::t3::n2941(s, out),
				['s', s @ ..] => super::t3::n2942(s, out),
				['t', s @ ..] => super::t3::n2943(s, out),
				['u', s @ ..] => super::t3::n2944(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['\u{11}', 'l', '\u{1}', s @ ..] => {
			super::push(out, 897);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 898);
			match s {
				[] => true,
				['s', 't', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2735);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vo.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 899);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 900);
			match s {
				[] => true,
				['a', 'm', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2736);
					s.is_empty()
				}
				['j', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2737);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 901);
			match s {
				[] => true,
				['a', 'c', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2739);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wo.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	s.is_empty()
}

/// Words ending -xo.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	s.is_empty()
}

/// Words ending -yo.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 293);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['g', 'n', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 2740);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5720);
							s.is_empty()
						}
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 5721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2741);
					match s {
						[] => true,
						['i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5722);
							s.is_empty()
						}
						['n', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 5723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5724);
							s.is_empty()
						}
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 5725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 903);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zo.
pub(crate) fn n295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 294);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 2742);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2743);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 5726);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 904);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2744);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2745);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2746);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['k', 'l', 'a', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 10019);
											s.is_empty()
										}
										['p', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10020);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'k', 'l', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 8319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8320);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 10021);
											s.is_empty()
										}
										['b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 10022);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'i', 'r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8321);
									s.is_empty()
								}
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

/// Words ending -er.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 316);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['e', 'd', s @ ..] => {
					super::push(out, 2785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2786);
					s.is_empty()
				}
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2788);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5749);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 8334);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							super::push(out, 5750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2789);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5751);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 8335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5752);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5753);
							match s {
								[] => true,
								['u', 'o', 'l', s @ ..] => {
									super::push(out, 8336);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 10023);
											s.is_empty()
										}
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
					super::push(out, 2790);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5754);
							match s {
								[] => true,
								['e', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8337);
									s.is_empty()
								}
								['n', 'i', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2791);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5755);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5756);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5757);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 8340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2792);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2793);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5758);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8341);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 10024);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5759);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8342);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 10025);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 8343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5760);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2794);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2795);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5762);
							match s {
								[] => true,
								['i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 8345);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2796);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2797);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5764);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 8346);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 10026);
											s.is_empty()
										}
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
					super::push(out, 2798);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2799);
					match s {
						[] => true,
						['p', 'm', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2800);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5766);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2801);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2802);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2803);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5769);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 5770);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5771);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 8347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 5772);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5773);
							match s {
								[] => true,
								['h', 's', '\u{1}', s @ ..] => {
									super::push(out, 8348);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5774);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8349);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2804);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5776);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 8350);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 10027);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 10028);
											s.is_empty()
										}
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
					super::push(out, 2805);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 5777);
							match s {
								[] => true,
								['a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2806);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t3::n1003(s, out),
		_ => false,
	}
}

/// Words ending -kr.
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 320);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2834);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 2835);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mr.
pub(crate) fn n323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 322);
	match s {
		[] => true,
		['\u{10}', '\u{1}', s @ ..] => {
			super::push(out, 937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -or.
pub(crate) fn n324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 323);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 938);
			match s {
				[] => true,
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 939);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2838);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2839);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 5799);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'm', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2840);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2841);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2842);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2843);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5801);
							s.is_empty()
						}
						['s', 'e', s @ ..] => {
							super::push(out, 5802);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 8364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'h', s @ ..] => {
					super::push(out, 2844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 940);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2845);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2846);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 5803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2847);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 2848);
					match s {
						[] => true,
						['a', 'f', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2849);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2850);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 5805);
							match s {
								[] => true,
								['i', 's', 'n', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'd', 'n', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2851);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5807);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 8366);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8367);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10032);
											match s {
												[] => true,
												['w', 'i', 's', '\u{1}', s @ ..] => {
													super::push(out, 10811);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8368);
									match s {
										[] => true,
										['e', 'p', s @ ..] => {
											super::push(out, 10033);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 8369);
									match s {
										[] => true,
										['e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 10034);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', 'e', 'r', s @ ..] => {
							super::push(out, 5808);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8370);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 10035);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'd', 'a', s @ ..] => {
							super::push(out, 5809);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pr.
pub(crate) fn n325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 324);
	match s {
		[] => true,
		['\u{11}', 'i', '\u{1}', s @ ..] => {
			super::push(out, 941);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rr.
pub(crate) fn n326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 325);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5811);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2852);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 5813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sr.
pub(crate) fn n327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 326);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 942);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2853);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 5814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 327);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 943);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2855);
					s.is_empty()
				}
				['h', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 944);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2857);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'r', s @ ..] => {
					super::push(out, 2858);
					match s {
						[] => true,
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2859);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5817);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 5818);
							match s {
								[] => true,
								['a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8371);
									s.is_empty()
								}
								['o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8372);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2860);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2861);
					s.is_empty()
				}
				['r', 'y', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2862);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2863);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5820);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 8373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					super::push(out, 2864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2865);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2866);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2867);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5821);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['a', 't', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10036);
											s.is_empty()
										}
										['e', 'r', 'o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 10037);
											s.is_empty()
										}
										['o', 's', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10038);
											s.is_empty()
										}
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
					super::push(out, 2868);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wr.
pub(crate) fn n329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 328);
	match s {
		[] => true,
		['\u{13}', 'd', 'o', 'f', 'd', 'd', 'e', 't', 's', 'i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 946);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gas (2 syllables).
pub(crate) fn n3103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2888);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 5836);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8376);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ias (2 syllables).
pub(crate) fn n3104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2889);
	match s {
		[] => true,
		['h', 'p', 'i', 'x', '\u{1}', s @ ..] => {
			super::push(out, 5837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -las (2 syllables).
pub(crate) fn n3105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2890);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 5838);
			match s {
				[] => true,
				['h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5839);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mas (2 syllables).
pub(crate) fn n3106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2891);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5841);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 8378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 5842);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nas (2 syllables).
pub(crate) fn n3107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2892);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 5843);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 5844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pas (2 syllables).
pub(crate) fn n3108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2893);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5845);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5846);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8381);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 10039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 't', '\u{1}', s @ ..] => {
			super::push(out, 5847);
			s.is_empty()
		}
		['t', 'u', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 5848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ras (2 syllables).
pub(crate) fn n3109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2894);
	match s {
		[] => true,
		['a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 5849);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5850);
			match s {
				[] => true,
				['n', 'e', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 8382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5851);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 8383);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 8384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tas (2 syllables).
pub(crate) fn n3110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2895);
	match s {
		[] => true,
		['i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5853);
			s.is_empty()
		}
		['n', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -as (3 syllables).
pub(crate) fn n1023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 949);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2897);
			match s {
				[] => true,
				['l', 'u', 'c', 'e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 5855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2898);
			match s {
				[] => true,
				['d', 'e', 'n', 'u', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 5856);
					s.is_empty()
				}
				['i', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5857);
					s.is_empty()
				}
				['n', 'o', 'g', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2899);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2900);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2901);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5860);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5861);
					s.is_empty()
				}
				['r', 'e', 'l', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2902);
			match s {
				[] => true,
				['i', 'd', 'o', 'n', 'a', 'l', 'l', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 5863);
					s.is_empty()
				}
				['u', 't', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2903);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2904);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', 'c', 'a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8385);
							s.is_empty()
						}
						['l', 'g', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'o', 'k', 'u', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 5865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2905);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5866);
					s.is_empty()
				}
				['i', 'l', 'e', 'p', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5867);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5868);
					match s {
						[] => true,
						['l', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 8387);
							s.is_empty()
						}
						['m', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2906);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5869);
					match s {
						[] => true,
						['l', 'b', 'o', 't', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8389);
							s.is_empty()
						}
						['r', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8390);
							s.is_empty()
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
				['b', s @ ..] => {
					super::push(out, 5870);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5871);
					match s {
						[] => true,
						['c', 'o', 'h', 't', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 8391);
							s.is_empty()
						}
						['t', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 8392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5872);
					s.is_empty()
				}
				['o', 'd', 'y', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5873);
					s.is_empty()
				}
				['r', 'a', 'b', 'm', 'e', '\u{1}', s @ ..] => {
					super::push(out, 5874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2908);
			match s {
				[] => true,
				['a', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5875);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5876);
					match s {
						[] => true,
						['n', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 8393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5877);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bs.
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 950);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5878);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2910);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 2911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 951);
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2912);
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2913);
					s.is_empty()
				}
				['i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2914);
					s.is_empty()
				}
				['r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2915);
					s.is_empty()
				}
				['t', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2916);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'e', 'r', 'k', 'r', 'o', 'm', 'r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 952);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cs.
pub(crate) fn n334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 953);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5880);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 5881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 954);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2919);
					s.is_empty()
				}
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2920);
					s.is_empty()
				}
				['o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2921);
					s.is_empty()
				}
				['t', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['a', 'm', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2923);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 956);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2925);
					match s {
						[] => true,
						['h', 't', 'e', 'r', 'o', 'g', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5882);
							s.is_empty()
						}
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
pub(crate) fn n335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2927);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2929);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 5884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 2930);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2931);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2932);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -es.
pub(crate) fn n336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2933);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2934);
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 2935);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2936);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2937);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 5886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2938);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2939);
					s.is_empty()
				}
				['u', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2941);
					match s {
						[] => true,
						['b', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 5887);
							s.is_empty()
						}
						['e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 5888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2942);
					match s {
						[] => true,
						['a', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5889);
							s.is_empty()
						}
						['r', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 5890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 's', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2943);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 8394);
									s.is_empty()
								}
								['u', 't', '\u{1}', s @ ..] => {
									super::push(out, 8395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 5891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2944);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5892);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 5894);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2945);
					match s {
						[] => true,
						['j', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2946);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['e', 'j', '\u{1}', s @ ..] => {
									super::push(out, 8397);
									s.is_empty()
								}
								['h', 's', '\u{1}', s @ ..] => {
									super::push(out, 8398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 5897);
							s.is_empty()
						}
						['m', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 5898);
							s.is_empty()
						}
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5899);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5900);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 10040);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 10041);
											s.is_empty()
										}
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
					super::push(out, 2947);
					match s {
						[] => true,
						['d', 'n', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5901);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 8399);
									s.is_empty()
								}
								['r', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 5902);
							s.is_empty()
						}
						['o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 5903);
							s.is_empty()
						}
						['t', 'a', 'e', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 5904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2948);
					match s {
						[] => true,
						['a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 5905);
							s.is_empty()
						}
						['i', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 5906);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5907);
							match s {
								[] => true,
								['e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8401);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2949);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 5908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2950);
					match s {
						[] => true,
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5909);
							s.is_empty()
						}
						['s', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2951);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5911);
							s.is_empty()
						}
						['l', 'a', 's', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5912);
							s.is_empty()
						}
						['p', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5913);
							s.is_empty()
						}
						['s', 'y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2952);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 5915);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 8403);
									s.is_empty()
								}
								['r', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 8404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2953);
					match s {
						[] => true,
						['a', 'l', 'y', 'x', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5916);
							s.is_empty()
						}
						['o', 'd', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', 'i', 'p', 'e', 'l', 'c', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2954);
					s.is_empty()
				}
				['g', 'r', 'e', 'n', 'a', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2955);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2956);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 5918);
							match s {
								[] => true,
								['e', 'o', 'c', 'a', 'c', '\u{1}', s @ ..] => super::t3::n9184(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t3::n3174(s, out),
				['l', s @ ..] => super::t3::n3175(s, out),
				['m', s @ ..] => super::t3::n3176(s, out),
				['n', s @ ..] => super::t3::n3177(s, out),
				['o', s @ ..] => super::t3::n3178(s, out),
				['p', 'a', 'n', 'a', 'k', 'c', 'a', 'j', '\u{1}', s @ ..] => super::t3::n3179(s, out),
				['s', s @ ..] => super::t3::n3180(s, out),
				['t', s @ ..] => super::t3::n3181(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fs.
pub(crate) fn n337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 962);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2965);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 5933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5934);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 5935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gs.
pub(crate) fn n338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 964);
			match s {
				[] => true,
				['c', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2969);
					s.is_empty()
				}
				['g', 'i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2970);
					s.is_empty()
				}
				['n', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2971);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2972);
					s.is_empty()
				}
				['u', 'o', 'j', '\u{1}', s @ ..] => {
					super::push(out, 2973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 965);
			match s {
				[] => true,
				['n', 'i', 'v', 'o', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 966);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hs.
pub(crate) fn n339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2975);
					s.is_empty()
				}
				['s', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['c', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2977);
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 2978);
					s.is_empty()
				}
				['t', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2980);
					match s {
						[] => true,
						['a', 'd', 'p', 'i', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5936);
							s.is_empty()
						}
						['o', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 5937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -is.
pub(crate) fn n340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 970);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5938);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 5939);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 5940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2982);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 2983);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2984);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 5941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 2985);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2986);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5942);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2988);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2989);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 2990);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2991);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2992);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 5945);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 5946);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 5947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 2994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 971);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2995);
					match s {
						[] => true,
						['l', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 5950);
							s.is_empty()
						}
						['u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2996);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2997);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2998);
					match s {
						[] => true,
						['f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5952);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5953);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 8413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2999);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 5954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3000);
					match s {
						[] => true,
						['a', 'r', 'h', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 5955);
							s.is_empty()
						}
						['u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3001);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5957);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5958);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3002);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5959);
							match s {
								[] => true,
								['a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8415);
									s.is_empty()
								}
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8416);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 5960);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5961);
							match s {
								[] => true,
								['o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8417);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5962);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 8418);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 8419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8420);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3003);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5963);
							match s {
								[] => true,
								['n', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => super::t3::n6435(s, out),
						['o', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5965);
							s.is_empty()
						}
						['r', 'e', s @ ..] => super::t3::n6437(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t3::n3222(s, out),
				['o', s @ ..] => super::t3::n3223(s, out),
				['p', s @ ..] => super::t3::n3224(s, out),
				['r', s @ ..] => super::t3::n3225(s, out),
				['s', s @ ..] => super::t3::n3226(s, out),
				['t', s @ ..] => super::t3::n3227(s, out),
				['u', s @ ..] => super::t3::n3228(s, out),
				['v', s @ ..] => super::t3::n3229(s, out),
				['w', s @ ..] => super::t3::n3230(s, out),
				['x', s @ ..] => super::t3::n3231(s, out),
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t3::n1048(s, out),
		_ => false,
	}
}

/// The word js (0 syllables).
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	s.is_empty()
}

/// Words ending -ks.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3027);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 6061);
							s.is_empty()
						}
						['o', 'l', 'b', 'x', '\u{1}', s @ ..] => {
							super::push(out, 6062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3029);
					match s {
						[] => true,
						['e', 'l', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6063);
							s.is_empty()
						}
						['o', 'm', 'm', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6064);
							s.is_empty()
						}
						['u', 'b', 'r', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['l', 'a', 'w', 'r', 'e', 'p', 'm', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 3031);
					s.is_empty()
				}
				['n', 'i', 'l', 'a', 'm', 'y', 'n', 'n', 'i', 'k', 's', '\u{1}', s @ ..] => {
					super::push(out, 3032);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3033);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ls.
pub(crate) fn n343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 342);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 975);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6066);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 6067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3036);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 3037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 977);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3038);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6069);
							s.is_empty()
						}
						['m', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'f', 'n', 'r', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 978);
			match s {
				[] => true,
				['e', 'f', 'o', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3040);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6071);
							s.is_empty()
						}
						['e', 'b', 'y', 'r', 'i', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ms.
pub(crate) fn n344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 343);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 979);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3041);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3042);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3043);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3044);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3045);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 980);
			match s {
				[] => true,
				['a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3047);
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 3048);
					s.is_empty()
				}
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3049);
					s.is_empty()
				}
				['m', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3050);
					s.is_empty()
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3051);
					s.is_empty()
				}
				['u', 'o', 'h', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 981);
			match s {
				[] => true,
				['o', 't', 's', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3053);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3054);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ns.
pub(crate) fn n345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 344);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 982);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3055);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3056);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 3057);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 3058);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 3059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6073);
							s.is_empty()
						}
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6074);
							s.is_empty()
						}
						['l', 'g', '\u{1}', s @ ..] => {
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
				['e', s @ ..] => {
					super::push(out, 3061);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 6077);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 6078);
							s.is_empty()
						}
						['i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 6079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3063);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 984);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3064);
					match s {
						[] => true,
						['d', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6082);
							s.is_empty()
						}
						['e', 'l', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6083);
							s.is_empty()
						}
						['l', 'g', 'u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 6084);
							s.is_empty()
						}
						['m', 'r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6085);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6086);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8522);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 8523);
									s.is_empty()
								}
								['t', 'u', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 8524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'x', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 6087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6088);
							s.is_empty()
						}
						['i', 'p', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 6089);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6090);
							s.is_empty()
						}
						['m', 'e', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 6091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3065);
					match s {
						[] => true,
						['b', 'b', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6092);
							s.is_empty()
						}
						['g', 'g', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8525);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3067);
					match s {
						[] => true,
						['c', 'i', 'l', 'p', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6093);
							s.is_empty()
						}
						['n', 'a', 'l', 'p', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6094);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6095);
							match s {
								[] => true,
								['o', 'f', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8527);
									s.is_empty()
								}
								['t', 'y', 'b', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8528);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3068);
					match s {
						[] => true,
						['c', 'u', 'd', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6096);
							s.is_empty()
						}
						['d', 'i', 'c', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6097);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6098);
							match s {
								[] => true,
								['n', 'i', 'f', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 6099);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6100);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8531);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10108);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3069);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 6102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['m', 'm', 'u', 's', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6103);
							s.is_empty()
						}
						['p', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6104);
							s.is_empty()
						}
						['t', 't', 'u', 'b', 'e', 'o', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 6105);
							s.is_empty()
						}
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
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 345);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3070);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3071);
					s.is_empty()
				}
				['e', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 3072);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 3073);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 3074);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3075);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 987);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3077);
					match s {
						[] => true,
						['b', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 6107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3078);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6108);
							s.is_empty()
						}
						['e', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 6109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3079);
					match s {
						[] => true,
						['n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3080);
					match s {
						[] => true,
						['k', 'l', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6111);
							s.is_empty()
						}
						['p', 'i', 'x', '\u{1}', s @ ..] => {
							super::push(out, 6112);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 10109);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 10110);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', '\u{1}', s @ ..] => {
									super::push(out, 8532);
									s.is_empty()
								}
								['n', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8533);
									s.is_empty()
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8534);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3081);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['k', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 6114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3083);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 6115);
							s.is_empty()
						}
						['l', 'a', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6116);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6117);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6118);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6119);
							s.is_empty()
						}
						['r', 'e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 6120);
							s.is_empty()
						}
						['s', 'o', s @ ..] => {
							super::push(out, 6121);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 8536);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 8537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3085);
					match s {
						[] => true,
						['h', 't', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
						['g', 'n', s @ ..] => {
							match s {
								[] => true,
								['o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8538);
									s.is_empty()
								}
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6124);
							s.is_empty()
						}
						['s', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 6125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3087);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 6126);
							s.is_empty()
						}
						['i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 6127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3088);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6128);
							match s {
								[] => true,
								['h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8540);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8541);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'd', 'n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6129);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6130);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8542);
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									super::push(out, 8543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6131);
							match s {
								[] => true,
								['p', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 8544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'y', '\u{1}', s @ ..] => {
							super::push(out, 6132);
							s.is_empty()
						}
						['u', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 6133);
							s.is_empty()
						}
						['y', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6135);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6136);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 8545);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 8546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6137);
							match s {
								[] => true,
								['u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t3::n1065(s, out),
		_ => false,
	}
}

/// Words ending -ps.
pub(crate) fn n347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 989);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3103);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3104);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3105);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 3106);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 3107);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3108);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6169);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 6170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 3110);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3111);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 3113);
					s.is_empty()
				}
				['x', '\u{1}', s @ ..] => {
					super::push(out, 3114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 990);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				['i', 't', '\u{1}', s @ ..] => {
					super::push(out, 3116);
					s.is_empty()
				}
				['m', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6172);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 6173);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 6174);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 6175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 6176);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3118);
					match s {
						[] => true,
						['i', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 991);
			match s {
				[] => true,
				['a', 'h', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3120);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3121);
					match s {
						[] => true,
						['c', 'n', s @ ..] => {
							super::push(out, 6179);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 8559);
									s.is_empty()
								}
								['i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3122);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3123);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6180);
							match s {
								[] => true,
								['k', 'y', 'k', '\u{1}', s @ ..] => {
									super::push(out, 8561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6181);
							match s {
								[] => true,
								['e', 'x', '\u{1}', s @ ..] => {
									super::push(out, 8562);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 992);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['c', 'i', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 6182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3126);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6183);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8564);
									match s {
										[] => true,
										['t', 'c', 'y', 'n', '\u{1}', s @ ..] => {
											super::push(out, 10118);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'g', 'e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8565);
									s.is_empty()
								}
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

/// Words ending -qs.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 347);
	s.is_empty()
}

/// Words ending -rs.
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 348);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 993);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6184);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 6185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 't', '\u{1}', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3128);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 6186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 3129);
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3131);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 3132);
					match s {
						[] => true,
						['d', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 3134);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 994);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3136);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6188);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6189);
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							super::push(out, 6190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3137);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 6191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3138);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 6192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3139);
					s.is_empty()
				}
				['u', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 995);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				['u', 'o', 'c', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 996);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ss.
pub(crate) fn n350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 349);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 997);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3144);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				['n', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				['q', '\u{1}', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				['w', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 998);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3150);
					match s {
						[] => true,
						['q', '\u{1}', s @ ..] => {
							super::push(out, 6193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3151);
					s.is_empty()
				}
				['h', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3152);
					s.is_empty()
				}
				['o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3153);
					s.is_empty()
				}
				['t', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				['u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3156);
					match s {
						[] => true,
						['i', 'p', 's', 'l', 'h', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['a', 'b', 'e', 't', 'i', 'h', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3158);
					s.is_empty()
				}
				['i', 'm', 'e', 'r', 'e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3159);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 3160);
					match s {
						[] => true,
						['t', 'a', 'b', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3161);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6196);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 8566);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10119);
											s.is_empty()
										}
										['i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10120);
											s.is_empty()
										}
										['o', 'l', 'i', 'k', '\u{1}', s @ ..] => {
											super::push(out, 10121);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'r', 'e', 'f', 'f', 'e', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ts.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 350);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1001);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3162);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 3163);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3164);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 6198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 1002);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3165);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 6199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3166);
					match s {
						[] => true,
						['e', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 6200);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 6201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3167);
					s.is_empty()
				}
				['i', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				['n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				['o', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6202);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 6203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3170);
					match s {
						[] => true,
						['g', 'y', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 8567);
									s.is_empty()
								}
								['e', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 8568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'f', 'k', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6204);
							s.is_empty()
						}
						['l', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6205);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6206);
							s.is_empty()
						}
						['r', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1004);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3171);
					match s {
						[] => true,
						['i', 'f', 'a', 'm', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6208);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							super::push(out, 6209);
							match s {
								[] => true,
								['l', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 8569);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 8570);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 6210);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['k', 'r', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 10122);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 10123);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 10851);
													s.is_empty()
												}
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
					super::push(out, 3172);
					s.is_empty()
				}
				['s', 'i', 'h', 'a', 'm', 'a', 'n', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3174);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -us.
pub(crate) fn n352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3175);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3176);
					s.is_empty()
				}
				['g', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3177);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 3178);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3179);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6211);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 8571);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 8572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 6212);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3180);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6214);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6216);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 6217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 3183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => super::t3::n1083(s, out),
		['\u{13}', s @ ..] => super::t3::n1084(s, out),
		_ => false,
	}
}

/// Words ending -vs.
pub(crate) fn n353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1008);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3221);
					s.is_empty()
				}
				['n', 't', '\u{1}', s @ ..] => {
					super::push(out, 3222);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ws.
pub(crate) fn n354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1009);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 1010);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3223);
					s.is_empty()
				}
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3225);
					match s {
						[] => true,
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6362);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 6363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'g', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1011);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xs.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1012);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ys.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3229);
					match s {
						[] => true,
						['w', 'g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3231);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 6365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'b', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3232);
					s.is_empty()
				}
				['t', 't', 'i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 3233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				['n', 'i', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3235);
					s.is_empty()
				}
				['t', 's', 'o', 'k', 'e', 't', 'n', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zs.
pub(crate) fn n357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1015);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3237);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ft.
pub(crate) fn n361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 360);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3261);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 3262);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['a', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3264);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6386);
							s.is_empty()
						}
						['r', 'c', s @ ..] => {
							super::push(out, 6387);
							match s {
								[] => true,
								['d', 'n', 'u', 'o', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 8818);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 8819);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 8820);
									match s {
										[] => true,
										['i', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 10234);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 10235);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8821);
									s.is_empty()
								}
								['r', 'i', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8822);
									s.is_empty()
								}
								['t', 'h', 'g', 'i', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8823);
									s.is_empty()
								}
								['w', 'o', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 8824);
									s.is_empty()
								}
								['y', 'k', 's', '\u{1}', s @ ..] => {
									super::push(out, 8825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3265);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1024);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3266);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6389);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8826);
									match s {
										[] => true,
										['s', 'l', 'l', 'e', 's', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 10236);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 8827);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6390);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 10891);
													match s {
														[] => true,
														['a', 'p', 's', s @ ..] => super::t3::n12401(s, out),
														_ => false,
													}
												}
												['l', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
													super::push(out, 10892);
													s.is_empty()
												}
												['n', 'i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 10893);
													s.is_empty()
												}
												['r', 'u', 's', 'a', 'e', 'l', 'p', '\u{1}', s @ ..] => {
													super::push(out, 10894);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 10237);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['c', 'u', 'a', 's', '\u{1}', s @ ..] => super::t3::n12402(s, out),
														['v', s @ ..] => super::t3::n12403(s, out),
														_ => false,
													}
												}
												['i', 'a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
													super::push(out, 10895);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 8828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'a', 'h', 'c', 's', 'e', 'g', 't', 'f', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3267);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word hundredweight (3 syllables).
pub(crate) fn n9652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8829);
	s.is_empty()
}

/// The word thousandweight (3 syllables).
pub(crate) fn n9653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8830);
	s.is_empty()
}

/// Words ending -it.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1027);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['a', 'b', 'd', s @ ..] => {
					super::push(out, 3273);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3274);
					match s {
						[] => true,
						['g', 'n', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3275);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'r', s @ ..] => {
					super::push(out, 3276);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3277);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6395);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 8831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3278);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6397);
							match s {
								[] => true,
								['z', 'e', 'k', '\u{1}', s @ ..] => {
									super::push(out, 8832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 3279);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 6398);
							match s {
								[] => true,
								['i', 'z', 't', '\u{1}', s @ ..] => {
									super::push(out, 8833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3280);
					s.is_empty()
				}
				['u', 'r', s @ ..] => {
					super::push(out, 3281);
					match s {
						[] => true,
						['f', 'i', 'w', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kt.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['\u{13}', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['e', 'f', 'f', 'e', 's', 'g', 'n', 'u', 'd', 'm', 'e', 'r', 'f', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3282);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lt.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1031);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					super::push(out, 3283);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'e', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['w', 's', 'n', 'e', 'b', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3285);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nt.
pub(crate) fn n367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1034);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3287);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6401);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 6402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3288);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6403);
							match s {
								[] => true,
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8834);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3289);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6404);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 8835);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 10238);
											match s {
												[] => true,
												['o', 'k', '\u{1}', s @ ..] => {
													super::push(out, 10896);
													s.is_empty()
												}
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
							super::push(out, 6405);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 6406);
							match s {
								[] => true,
								['r', 'e', 's', 'n', 'a', s @ ..] => {
									super::push(out, 8836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3290);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6407);
							match s {
								[] => true,
								['r', 'u', s @ ..] => {
									super::push(out, 8837);
									match s {
										[] => true,
										['t', 'i', 'b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 10239);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6408);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 6409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ot.
pub(crate) fn n368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['\u{11}', 'o', 'f', '\u{1}', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3291);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6410);
							match s {
								[] => true,
								['u', 't', '\u{1}', s @ ..] => {
									super::push(out, 8838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3292);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3293);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3294);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3295);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3296);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6412);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6413);
							match s {
								[] => true,
								['b', 'u', 'l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8839);
									s.is_empty()
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['a', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 10240);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 10241);
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													super::push(out, 10897);
													s.is_empty()
												}
												['i', 'h', '\u{1}', s @ ..] => {
													super::push(out, 10898);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['g', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8840);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8841);
									s.is_empty()
								}
								['s', 'w', 'o', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8842);
									s.is_empty()
								}
								['t', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8843);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 8844);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10242);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', 'a', 'l', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 8845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6414);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 6415);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6416);
							match s {
								[] => true,
								['d', 'i', 'u', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 8846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3297);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3299);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3300);
					match s {
						[] => true,
						['s', 'e', 'p', 'a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3301);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3302);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3303);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6419);
							match s {
								[] => true,
								['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8847);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'u', 't', '\u{1}', s @ ..] => {
											super::push(out, 10243);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 10244);
											match s {
												[] => true,
												['b', 'm', 'u', 'b', '\u{1}', s @ ..] => {
													super::push(out, 10899);
													s.is_empty()
												}
												['d', 'd', 'a', 'p', '\u{1}', s @ ..] => {
													super::push(out, 10900);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 10245);
											match s {
												[] => true,
												['a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 10901);
													s.is_empty()
												}
												['o', 'f', '\u{1}', s @ ..] => {
													super::push(out, 10902);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['g', 'n', 'i', 'k', 'c', 'o', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 8848);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8849);
									match s {
										[] => true,
										['d', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10246);
											s.is_empty()
										}
										['l', 'i', 'k', '\u{1}', s @ ..] => {
											super::push(out, 10247);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 10248);
											s.is_empty()
										}
										['p', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 10249);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'n', 'e', 'v', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8850);
									s.is_empty()
								}
								['y', 'p', 'p', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6420);
							s.is_empty()
						}
						['n', 'r', 'e', 'p', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6421);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pt (0 syllables).
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			super::push(out, 1040);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rt.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 3305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3307);
					match s {
						[] => true,
						['s', 'n', 'o', 'i', 't', 'k', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'w', 'l', 'l', 'u', 'n', 'a', 't', 'e', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 3308);
					s.is_empty()
				}
				['o', 'w', 'r', 'e', 'd', s @ ..] => {
					super::push(out, 3309);
					match s {
						[] => true,
						['n', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -st.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1044);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 1045);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3310);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6426);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3312);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3313);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6427);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 8853);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3314);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3315);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6429);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8854);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['d', 'l', 'i', 'w', '\u{1}', s @ ..] => {
													super::push(out, 10903);
													s.is_empty()
												}
												['t', 'r', 'a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 10904);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', 'i', s @ ..] => {
							super::push(out, 6430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3316);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6431);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6432);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8855);
									match s {
										[] => true,
										['r', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 10250);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6433);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8856);
									s.is_empty()
								}
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

/// Words ending -tt.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 3317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'a', 'l', s @ ..] => {
			super::push(out, 1050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ut.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1051);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3319);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 3320);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 6434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3322);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3323);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3324);
					match s {
						[] => true,
						['o', 'y', '\u{1}', s @ ..] => {
							super::push(out, 6435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3325);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3327);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3328);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6437);
							match s {
								[] => true,
								['g', 'n', 'u', 'l', 'a', 'q', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8857);
									s.is_empty()
								}
								['m', 'i', 'v', 'a', 'n', 'u', 'n', '\u{1}', s @ ..] => {
									super::push(out, 8858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3329);
					match s {
						[] => true,
						['i', 'c', s @ ..] => {
							match s {
								[] => true,
								['c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 8859);
									s.is_empty()
								}
								['n', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 8860);
									s.is_empty()
								}
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

/// The word cwt (0 syllables).
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	s.is_empty()
}

/// Words ending -xt.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['\u{12}', 'e', 't', 'r', 'u', s @ ..] => {
			super::push(out, 1054);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fu (2 syllables).
pub(crate) fn n1144(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3351);
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 6451);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8870);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 8871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 6452);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3352);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gu.
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3353);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6454);
							match s {
								[] => true,
								['o', 'y', '\u{1}', s @ ..] => {
									super::push(out, 8872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3354);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8873);
									s.is_empty()
								}
								['z', 'u', 'm', 'u', '\u{1}', s @ ..] => {
									super::push(out, 8874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3355);
					match s {
						[] => true,
						['f', 'a', 'r', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 6456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hu.
pub(crate) fn n384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	match s {
		[] => true,
		['\u{11}', 'z', '\u{1}', s @ ..] => {
			super::push(out, 1062);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['c', 'n', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3356);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3357);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6457);
							match s {
								[] => true,
								['i', 'j', '\u{1}', s @ ..] => {
									super::push(out, 8875);
									s.is_empty()
								}
								['o', 'h', 'z', '\u{1}', s @ ..] => {
									super::push(out, 8876);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3358);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 6458);
							s.is_empty()
						}
						['u', 'h', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3359);
					s.is_empty()
				}
				['s', 'u', 'o', 'y', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 's', s @ ..] => {
			super::push(out, 1063);
			match s {
				[] => true,
				['a', 'k', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3361);
					s.is_empty()
				}
				['i', 'g', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3362);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iu.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 384);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 3363);
					s.is_empty()
				}
				['d', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3364);
					s.is_empty()
				}
				['x', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ku.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 386);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 6460);
							s.is_empty()
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 6461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3366);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3367);
					match s {
						[] => true,
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6462);
							s.is_empty()
						}
						['u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 6464);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 6465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1066);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3368);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6466);
							s.is_empty()
						}
						['h', 'c', 'n', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 6467);
							s.is_empty()
						}
						['j', 'a', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6468);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'k', '\u{1}', s @ ..] => {
									super::push(out, 8877);
									s.is_empty()
								}
								['k', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['h', 's', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8879);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10255);
											s.is_empty()
										}
										['d', '\u{1}', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 3369);
					s.is_empty()
				}
				['k', 'u', 'r', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3370);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6469);
							s.is_empty()
						}
						['l', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6470);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['m', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 8880);
									s.is_empty()
								}
								['n', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 8881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 6471);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 6472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'e', 'r', 'r', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3371);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6473);
							s.is_empty()
						}
						['h', 's', 'n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6474);
							s.is_empty()
						}
						['l', 'l', 'u', '\u{1}', s @ ..] => {
							super::push(out, 6475);
							s.is_empty()
						}
						['n', 'e', 'l', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6476);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8882);
									s.is_empty()
								}
								['p', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 8883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'u', 'd', 'n', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6477);
							s.is_empty()
						}
						['t', 'u', 'k', 'u', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8884);
									s.is_empty()
								}
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 8885);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lu.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1067);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1068);
			match s {
				[] => true,
				['k', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 3372);
					s.is_empty()
				}
				['l', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3373);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 6479);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['u', 'z', 'm', 'u', '\u{1}', s @ ..] => {
					super::push(out, 3374);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mu.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 388);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['s', 'i', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3375);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3376);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nu.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 389);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3377);
					s.is_empty()
				}
				['g', 'n', 'e', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 3378);
					s.is_empty()
				}
				['i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3379);
					s.is_empty()
				}
				['r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3381);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3382);
					s.is_empty()
				}
				['i', 'a', 'm', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3383);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3384);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6481);
							match s {
								[] => true,
								['c', 's', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8886);
									s.is_empty()
								}
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

/// Words ending -ou.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1070);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['d', 'u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3385);
					s.is_empty()
				}
				['s', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3386);
					s.is_empty()
				}
				['t', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3387);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3388);
					match s {
						[] => true,
						['h', 'u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 6482);
							s.is_empty()
						}
						['k', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6483);
							s.is_empty()
						}
						['r', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3389);
					match s {
						[] => true,
						['u', 'o', 'l', 'u', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 6485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'b', 'i', 'n', 'u', 'u', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3390);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pu.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1071);
			s.is_empty()
		}
		['\u{12}', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 1072);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -qu.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['\u{12}', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 1073);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ru.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3391);
					match s {
						[] => true,
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 6486);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 6487);
							s.is_empty()
						}
						['y', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 3392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', 'g', 'i', 'h', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3393);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3394);
					match s {
						[] => true,
						['c', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6489);
							s.is_empty()
						}
						['s', 'o', 'd', 'n', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6490);
							s.is_empty()
						}
						['t', 'i', 'k', 'e', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3395);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6492);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8887);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10257);
											s.is_empty()
										}
										['a', '\u{1}', s @ ..] => {
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
				['u', 'b', 'm', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -su.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1074);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3397);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 3398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6493);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 6494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6495);
							s.is_empty()
						}
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1075);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', 'r', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 6497);
							s.is_empty()
						}
						['z', 'i', 'e', 'w', 'u', 'w', '\u{1}', s @ ..] => {
							super::push(out, 6498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8888);
									s.is_empty()
								}
								['j', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 8889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'c', 'a', 'r', 'a', 'r', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 6499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 6500);
							s.is_empty()
						}
						['e', 'r', 'i', 'e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6501);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', 's', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 8890);
									s.is_empty()
								}
								['j', 'u', 'j', '\u{1}', s @ ..] => {
									super::push(out, 8891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'b', 'i', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'r', 'a', 'b', 'm', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3401);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tu.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['\u{11}', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1076);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', 'w', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3402);
					s.is_empty()
				}
				['k', 'o', 'k', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'a', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['r', 'e', 'f', 's', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uu.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{11}', 'y', 'i', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 1078);
			s.is_empty()
		}
		['\u{13}', 'y', 'n', 'u', 'k', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vu.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1080);
			s.is_empty()
		}
		['\u{13}', 'a', 'n', 'a', 'k', 'a', 'n', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 1081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xu.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 1082);
			s.is_empty()
		}
		['\u{12}', 'm', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1083);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yu.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3405);
					s.is_empty()
				}
				['i', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3406);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1084);
			match s {
				[] => true,
				['g', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 3408);
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3409);
					s.is_empty()
				}
				['s', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3410);
					s.is_empty()
				}
				['u', 'k', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['n', 'u', 'k', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3412);
					s.is_empty()
				}
				['u', 'k', 'i', 'k', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hylothorax (3 syllables).
pub(crate) fn n12026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10905);
	s.is_empty()
}

/// Words ending -lex (2 syllables).
pub(crate) fn n3686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3441);
	match s {
		[] => true,
		['l', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6517);
			s.is_empty()
		}
		['o', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 6518);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 6519);
			s.is_empty()
		}
		['u', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mex (2 syllables).
pub(crate) fn n3687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3442);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6521);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 8898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nex (2 syllables).
pub(crate) fn n3688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3443);
	s.is_empty()
}

/// Words ending -pex (2 syllables).
pub(crate) fn n3689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3444);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 6523);
			s.is_empty()
		}
		['s', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6524);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word imbrex (2 syllables).
pub(crate) fn n3690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3445);
	s.is_empty()
}

/// Words ending -tex (2 syllables).
pub(crate) fn n3691(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 6525);
			s.is_empty()
		}
		['p', 'y', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6526);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6527);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8899);
					s.is_empty()
				}
				['o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8900);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ex (3 syllables).
pub(crate) fn n1205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1105);
	match s {
		[] => true,
		['f', 'i', 't', 'n', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3446);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3447);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3448);
			match s {
				[] => true,
				['l', 'o', 'p', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6528);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6529);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8901);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 10262);
									match s {
										[] => true,
										['e', 'o', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10906);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 10907);
											match s {
												[] => true,
												['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
													super::push(out, 11239);
													s.is_empty()
												}
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
					super::push(out, 6530);
					match s {
						[] => true,
						['m', 'i', s @ ..] => {
							super::push(out, 8902);
							s.is_empty()
						}
						['u', 'd', 'o', s @ ..] => {
							super::push(out, 8903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'i', 'k', 'l', 'o', 'z', 't', '\u{1}', s @ ..] => {
			super::push(out, 3449);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3450);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6531);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3451);
			match s {
				[] => true,
				['a', 'l', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6533);
					s.is_empty()
				}
				['e', 'l', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 6534);
					s.is_empty()
				}
				['i', 'c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6535);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6536);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6537);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8904);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 10263);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10264);
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10265);
									s.is_empty()
								}
								['e', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10266);
									s.is_empty()
								}
								['i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 10267);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['l', 'l', 'a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11240);
													s.is_empty()
												}
												['t', 'x', 'u', 'j', '\u{1}', s @ ..] => {
													super::push(out, 11241);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'e', 'r', 'd', 'a', '\u{1}', s @ ..] => {
											super::push(out, 10908);
											s.is_empty()
										}
										['s', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10909);
											s.is_empty()
										}
										['x', 'e', '\u{1}', s @ ..] => {
											super::push(out, 10910);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'r', 'f', 'f', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6538);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word rfx (0 syllables).
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 418);
	s.is_empty()
}

/// The word lhx (0 syllables).
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 419);
	s.is_empty()
}

/// Words ending -ix.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 420);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1106);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3452);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 6539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1107);
			match s {
				[] => true,
				['b', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3453);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3454);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3455);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 6540);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 8905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3456);
					match s {
						[] => true,
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3457);
					match s {
						[] => true,
						['e', 'o', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6542);
							s.is_empty()
						}
						['i', 'o', 'h', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6543);
							s.is_empty()
						}
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3458);
					match s {
						[] => true,
						['m', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6545);
							s.is_empty()
						}
						['r', 'f', 'u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3459);
					match s {
						[] => true,
						['h', 't', 'o', 'i', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6547);
							s.is_empty()
						}
						['p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6548);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6549);
							match s {
								[] => true,
								['a', 'e', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8906);
									s.is_empty()
								}
								['r', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 8907);
									s.is_empty()
								}
								['u', 't', '\u{1}', s @ ..] => {
									super::push(out, 8908);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1108);
			match s {
				[] => true,
				['d', 'n', 'e', 'p', 'p', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6550);
							s.is_empty()
						}
						['o', 's', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3460);
					s.is_empty()
				}
				['l', 'e', 'h', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 't', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 6552);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['m', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8909);
									s.is_empty()
								}
								['n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3461);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3462);
					match s {
						[] => true,
						['p', 'e', 'r', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6554);
							s.is_empty()
						}
						['v', 't', 'a', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3463);
					match s {
						[] => true,
						['a', 'm', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 6556);
							s.is_empty()
						}
						['h', 't', 'o', 't', 'p', 'e', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6557);
							s.is_empty()
						}
						['t', s @ ..] => super::t3::n7095(s, out),
						_ => false,
					}
				}
				['v', 'r', 'e', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6558);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8915);
									s.is_empty()
								}
								['r', 't', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8916);
									s.is_empty()
								}
								['t', 'c', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8917);
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8918);
									s.is_empty()
								}
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

/// The word brujx (1 syllables).
pub(crate) fn n424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 421);
	s.is_empty()
}

/// Words ending -lx.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 422);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1109);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3464);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3465);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3466);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mx.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1110);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3467);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3468);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nx.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1111);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3469);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 6560);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 6561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3470);
					s.is_empty()
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['h', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 6562);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 6563);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 6564);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 6565);
							s.is_empty()
						}
						['r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1112);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3471);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6567);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8919);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 6568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6569);
							s.is_empty()
						}
						['p', 'l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 6570);
							s.is_empty()
						}
						['r', 'y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6571);
							s.is_empty()
						}
						['t', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3472);
					s.is_empty()
				}
				['y', 'r', 'a', s @ ..] => {
					match s {
						[] => true,
						['h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6573);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 6574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1113);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3473);
					match s {
						[] => true,
						['c', 'i', 'x', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6575);
							s.is_empty()
						}
						['l', 'a', 'h', 'p', 'o', 's', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6577);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8920);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6578);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8921);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6579);
							match s {
								[] => true,
								['i', 'l', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8922);
									s.is_empty()
								}
								['l', 'a', 's', 'o', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 10281);
											s.is_empty()
										}
										['r', 'd', 'y', 'h', '\u{1}', s @ ..] => {
											super::push(out, 10282);
											s.is_empty()
										}
										['s', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10283);
											s.is_empty()
										}
										['y', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10284);
											s.is_empty()
										}
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
					super::push(out, 3474);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['h', 'p', 'o', s @ ..] => {
									match s {
										[] => true,
										['g', 'n', 'y', 'r', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 10285);
											s.is_empty()
										}
										['l', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 10286);
											s.is_empty()
										}
										['p', 'y', 'h', '\u{1}', s @ ..] => {
											super::push(out, 10287);
											s.is_empty()
										}
										['t', 'a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10288);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'o', s @ ..] => {
									match s {
										[] => true,
										['p', 'y', 'h', '\u{1}', s @ ..] => {
											super::push(out, 10289);
											s.is_empty()
										}
										['y', 'h', '\u{1}', s @ ..] => {
											super::push(out, 10290);
											s.is_empty()
										}
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

/// Words ending -ox.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 425);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3475);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3476);
					match s {
						[] => true,
						['h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 3477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['k', 's', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1116);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3479);
					s.is_empty()
				}
				['d', 'o', s @ ..] => {
					super::push(out, 3480);
					match s {
						[] => true,
						['h', 't', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3481);
					match s {
						[] => true,
						['o', 'd', 'r', 'o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rx.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 426);
	match s {
		[] => true,
		['\u{10}', '\u{1}', s @ ..] => {
			super::push(out, 1117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tx.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1118);
			s.is_empty()
		}
		['\u{11}', 's', 'e', 'i', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ux.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3482);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 6583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3483);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1121);
			match s {
				[] => true,
				['a', 'e', 'd', 'r', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3485);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3486);
					match s {
						[] => true,
						['r', 't', 'r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6584);
							s.is_empty()
						}
						['s', 's', 'u', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3487);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6586);
							s.is_empty()
						}
						['l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1122);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3488);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3489);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6588);
							s.is_empty()
						}
						['i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6589);
							s.is_empty()
						}
						['o', 'l', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'a', 'b', 'r', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3490);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xx.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	match s {
		[] => true,
		['\u{10}', 'x', '\u{1}', s @ ..] => {
			super::push(out, 1123);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yx.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 430);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['c', 'c', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3491);
					s.is_empty()
				}
				['l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3492);
					s.is_empty()
				}
				['n', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['c', 'c', 'o', 'c', 'o', 'r', 'c', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3494);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3495);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6591);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['i', 'p', 'e', '\u{1}', s @ ..] => {
											super::push(out, 10291);
											s.is_empty()
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['r', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 10926);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 10927);
													s.is_empty()
												}
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
					super::push(out, 3496);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3497);
					match s {
						[] => true,
						['e', 't', 'p', 'o', s @ ..] => {
							match s {
								[] => true,
								['i', 'a', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8923);
									s.is_empty()
								}
								['r', 'o', 'v', 'i', 'n', 'm', 'o', '\u{1}', s @ ..] => {
									super::push(out, 8924);
									s.is_empty()
								}
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

/// The word mestizx (2 syllables).
pub(crate) fn n434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 431);
	s.is_empty()
}

/// Words ending -ey.
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1134);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3525);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['g', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3527);
					s.is_empty()
				}
				['k', 'o', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 3528);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3529);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 3530);
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					super::push(out, 3531);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 6614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3532);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6615);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8930);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 10293);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6616);
							match s {
								[] => true,
								['r', 'u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3533);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6617);
							s.is_empty()
						}
						['r', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 6618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1136);
			match s {
				[] => true,
				['g', 'n', 'a', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3534);
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					super::push(out, 3535);
					match s {
						[] => true,
						['o', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gy.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 1139);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1140);
			match s {
				[] => true,
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3539);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3540);
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							super::push(out, 6620);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hy.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3541);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 3542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1143);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					super::push(out, 3543);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 6621);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3544);
					s.is_empty()
				}
				['w', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1144);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3546);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6623);
							match s {
								[] => true,
								['r', 't', 's', 'y', 'd', 'o', 'l', 'u', 'c', 's', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8933);
									s.is_empty()
								}
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

/// Words ending -ky.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3547);
					s.is_empty()
				}
				['h', 's', 'o', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3548);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', 's', '\u{1}', s @ ..] => {
							super::push(out, 6624);
							s.is_empty()
						}
						['u', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 6625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 'u', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6626);
							s.is_empty()
						}
						['n', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 6627);
							s.is_empty()
						}
						['o', 'r', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 't', 'e', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1146);
			match s {
				[] => true,
				['c', 'i', 'd', 'd', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3550);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3551);
					match s {
						[] => true,
						['s', 'o', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6629);
							s.is_empty()
						}
						['z', 'o', 'r', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 'm', 'o', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6631);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6632);
							match s {
								[] => true,
								['a', 'j', s @ ..] => {
									match s {
										[] => true,
										['i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10294);
											s.is_empty()
										}
										['o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10295);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['t', 't', 'u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 10296);
											s.is_empty()
										}
										['w', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 10297);
											s.is_empty()
										}
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

/// Words ending -ly.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 443);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1147);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3552);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 3553);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 3554);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 3555);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3556);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 3557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3558);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 6633);
							s.is_empty()
						}
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6634);
							s.is_empty()
						}
						['q', '\u{1}', s @ ..] => {
							super::push(out, 6635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3559);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3560);
					match s {
						[] => true,
						['e', 'w', s @ ..] => {
							super::push(out, 6636);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 8934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3561);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3562);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3563);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6638);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8935);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10298);
											s.is_empty()
										}
										['n', '\u{1}', s @ ..] => {
											super::push(out, 10299);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 3564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1148);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3565);
					match s {
						[] => true,
						['k', 'e', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 6639);
							s.is_empty()
						}
						['o', 'c', 'o', 'r', 'e', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6640);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6641);
							match s {
								[] => true,
								['o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'm', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 3566);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3567);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -my.
pub(crate) fn n448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 1149);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1150);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 8937);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8939);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'j', '\u{1}', s @ ..] => {
							super::push(out, 6643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1151);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3568);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 6644);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8941);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10300);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 8942);
									match s {
										[] => true,
										['o', 't', s @ ..] => {
											super::push(out, 10301);
											match s {
												[] => true,
												['c', 'o', s @ ..] => {
													super::push(out, 10928);
													match s {
														[] => true,
														['r', 'p', '\u{1}', s @ ..] => super::t3::n12417(s, out),
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
		_ => false,
	}
}

/// Words ending -ny.
pub(crate) fn n449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 445);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 1152);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3569);
					s.is_empty()
				}
				['i', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3570);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3571);
					match s {
						[] => true,
						['h', 'o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 6645);
							s.is_empty()
						}
						['u', 't', '\u{1}', s @ ..] => {
							super::push(out, 6646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3572);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6647);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 6648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3574);
					match s {
						[] => true,
						['m', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6649);
							s.is_empty()
						}
						['t', 'n', 'o', 'k', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3575);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3576);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									super::push(out, 8943);
									s.is_empty()
								}
								['i', 'r', 'e', s @ ..] => {
									super::push(out, 8944);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 't', s @ ..] => {
							super::push(out, 6651);
							match s {
								[] => true,
								['u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8945);
									s.is_empty()
								}
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

/// Words ending -oy.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1154);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 3577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1155);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3578);
					match s {
						[] => true,
						['m', 'a', 's', 't', '\u{1}', s @ ..] => {
							super::push(out, 6652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -py.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1157);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1158);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3580);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6653);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1159);
			match s {
				[] => true,
				['a', 'r', 't', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 3582);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ry.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1160);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 3583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1161);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3584);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 6655);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 6656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3585);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3586);
					s.is_empty()
				}
				['u', 'a', s @ ..] => {
					super::push(out, 3587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1162);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3588);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6657);
							match s {
								[] => true,
								['r', 'd', 'n', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6658);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8947);
									match s {
										[] => true,
										['g', 'n', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 10302);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'i', s @ ..] => {
							super::push(out, 6659);
							match s {
								[] => true,
								['l', 'i', s @ ..] => {
									super::push(out, 8948);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 10303);
											s.is_empty()
										}
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
					super::push(out, 3589);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6660);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8949);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 10304);
											s.is_empty()
										}
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
					super::push(out, 3590);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6661);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 8950);
									match s {
										[] => true,
										['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10305);
											s.is_empty()
										}
										['i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10306);
											s.is_empty()
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['c', 'i', 'p', '\u{1}', s @ ..] => {
													super::push(out, 10929);
													s.is_empty()
												}
												['r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 10930);
													s.is_empty()
												}
												['t', 'm', 'e', 'f', '\u{1}', s @ ..] => {
													super::push(out, 10931);
													s.is_empty()
												}
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
							super::push(out, 6662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3591);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6663);
							match s {
								[] => true,
								['e', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 8951);
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
						['a', s @ ..] => {
							super::push(out, 6664);
							match s {
								[] => true,
								['g', 'n', 'e', 'l', 'g', '\u{1}', s @ ..] => {
									super::push(out, 8952);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', 'k', s @ ..] => {
							super::push(out, 6665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3592);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6666);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 8953);
									match s {
										[] => true,
										['o', 'c', 'p', 'u', '\u{1}', s @ ..] => {
											super::push(out, 10307);
											s.is_empty()
										}
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
					super::push(out, 3593);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6667);
							match s {
								[] => true,
								['d', 'a', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8954);
									s.is_empty()
								}
								['r', 'e', 't', 'n', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8955);
									s.is_empty()
								}
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

/// Words ending -sy.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 1163);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1164);
			match s {
				[] => true,
				['d', 'i', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 3594);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3595);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1165);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'l', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6669);
							s.is_empty()
						}
						['g', 'a', 'l', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6670);
							s.is_empty()
						}
						['i', 'b', 'm', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3596);
					match s {
						[] => true,
						['b', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3597);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ty.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1166);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3598);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3599);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6673);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 6674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1167);
			match s {
				[] => true,
				['a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3600);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3601);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3602);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6675);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 8956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3603);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6676);
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							super::push(out, 6677);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 6678);
							s.is_empty()
						}
						['l', 'z', '\u{1}', s @ ..] => {
							super::push(out, 6679);
							s.is_empty()
						}
						['m', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6680);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6681);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 6682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'w', 'q', '\u{1}', s @ ..] => {
					super::push(out, 3604);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3605);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1168);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3607);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6683);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8957);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 10308);
											match s {
												[] => true,
												['x', s @ ..] => {
													super::push(out, 10932);
													match s {
														[] => true,
														['a', 'i', s @ ..] => super::t3::n12418(s, out),
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
									super::push(out, 8958);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 10309);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 10933);
													match s {
														[] => true,
														['t', s @ ..] => super::t3::n12419(s, out),
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
					super::push(out, 3608);
					match s {
						[] => true,
						['a', 'i', 'r', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3609);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uy.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1169);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3610);
					s.is_empty()
				}
				['u', 'n', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1170);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3612);
					s.is_empty()
				}
				['l', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3613);
					s.is_empty()
				}
				['m', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3615);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 3616);
					match s {
						[] => true,
						['n', 'a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6686);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6687);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 8959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'a', 'p', 'u', 'k', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vy.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 1171);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wy.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3618);
					s.is_empty()
				}
				['g', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3619);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 3620);
					s.is_empty()
				}
				['n', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3621);
					s.is_empty()
				}
				['r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3622);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 3623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 3624);
					s.is_empty()
				}
				['p', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xy.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['n', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3626);
					s.is_empty()
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3628);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3629);
					match s {
						[] => true,
						['p', 'o', s @ ..] => {
							match s {
								[] => true,
								['l', 'p', 'i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8960);
									s.is_empty()
								}
								['m', 'u', 'e', 'n', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3630);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6688);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8962);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 't', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8963);
									s.is_empty()
								}
								['m', 'o', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 6689);
							match s {
								[] => true,
								['l', 'a', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 6690);
							match s {
								[] => true,
								['y', 'r', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8966);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'h', 'p', 'o', 'r', 'o', 'l', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6691);
							s.is_empty()
						}
						['p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6692);
							s.is_empty()
						}
						['r', 'p', s @ ..] => {
							super::push(out, 6693);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8968);
									s.is_empty()
								}
								['o', 'r', 'u', '\u{1}', s @ ..] => {
									super::push(out, 8969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rz.
pub(crate) fn n473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 468);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1189);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 1190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sz.
pub(crate) fn n474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 469);
	match s {
		[] => true,
		['\u{11}', 'o', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1191);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1192);
			match s {
				[] => true,
				['a', 'v', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tz.
pub(crate) fn n475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 470);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 1193);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3671);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 3672);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 3673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 1194);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3674);
					s.is_empty()
				}
				['o', 'k', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 3675);
					s.is_empty()
				}
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1195);
			match s {
				[] => true,
				['e', 'g', 'e', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 3677);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3678);
					match s {
						[] => true,
						['r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3679);
					match s {
						[] => true,
						['b', 'b', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 6703);
							s.is_empty()
						}
						['l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 1196);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3680);
					match s {
						[] => true,
						['v', 'o', 'k', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3681);
					match s {
						[] => true,
						['w', 'o', 'k', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3682);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uz.
pub(crate) fn n476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 471);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1197);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3683);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 3684);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 3685);
					s.is_empty()
				}
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3686);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 3687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1198);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yz.
pub(crate) fn n478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1199);
			match s {
				[] => true,
				['o', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3688);
					s.is_empty()
				}
				['x', '\u{1}', s @ ..] => {
					super::push(out, 3689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 1200);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zz.
pub(crate) fn n479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 1201);
			s.is_empty()
		}
		['\u{12}', 'i', 't', 's', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1202);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -istropha (3 syllables).
pub(crate) fn n12067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10940);
	s.is_empty()
}

/// Words ending -eia (3 syllables).
pub(crate) fn n1411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1293);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 3797);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6760);
					match s {
						[] => true,
						['a', 'p', 'o', 'i', 'r', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3798);
			match s {
				[] => true,
				['p', 'o', s @ ..] => {
					match s {
						[] => true,
						['h', 't', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8994);
							s.is_empty()
						}
						['l', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gia (3 syllables).
pub(crate) fn n1412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1294);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3799);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6761);
					s.is_empty()
				}
				['o', 'b', 'i', 'h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3800);
			match s {
				[] => true,
				['l', 'u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6763);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hia (3 syllables).
pub(crate) fn n1413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1295);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3801);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6764);
					match s {
						[] => true,
						['l', 'e', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8996);
							s.is_empty()
						}
						['t', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6765);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8998);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10324);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 10941);
											s.is_empty()
										}
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
			super::push(out, 3802);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3803);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lia (3 syllables).
pub(crate) fn n1416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1298);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3805);
			match s {
				[] => true,
				['d', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6766);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6767);
					s.is_empty()
				}
				['m', 'i', 's', 'e', 'g', 'a', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 6768);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6769);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8999);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['e', 'd', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n12541(s, out),
														['n', 'i', 'u', 'q', '\u{1}', s @ ..] => super::t3::n12542(s, out),
														_ => false,
													}
												}
												['i', s @ ..] => {
													match s {
														[] => true,
														['r', 't', '\u{1}', s @ ..] => super::t3::n12543(s, out),
														['v', '\u{1}', s @ ..] => super::t3::n12544(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 10942);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'f', s @ ..] => {
					super::push(out, 6770);
					match s {
						[] => true,
						['e', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 9000);
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
				['a', 'c', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6771);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'o', s @ ..] => {
			super::push(out, 3806);
			match s {
				[] => true,
				['r', 'u', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3807);
			match s {
				[] => true,
				['h', 'p', 'o', s @ ..] => {
					super::push(out, 6774);
					s.is_empty()
				}
				['m', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mia (3 syllables).
pub(crate) fn n1417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1299);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3808);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3809);
			match s {
				[] => true,
				['n', 'o', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3810);
			match s {
				[] => true,
				['n', 'o', 'n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 6777);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nia (3 syllables).
pub(crate) fn n1418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1300);
	match s {
		[] => true,
		['e', 'g', s @ ..] => {
			super::push(out, 3811);
			match s {
				[] => true,
				['i', 'r', 't', 'a', 'd', 'n', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3812);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3813);
			match s {
				[] => true,
				['c', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3814);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3815);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 6780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oia (3 syllables).
pub(crate) fn n1419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1301);
	match s {
		[] => true,
		['n', 'i', 'r', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pia (3 syllables).
pub(crate) fn n1420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1302);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					super::push(out, 6781);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['s', 'y', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9001);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 9002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ria (3 syllables).
pub(crate) fn n1421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1303);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3817);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9003);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 10325);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10943);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 10944);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 11256);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'e', 'c', 'i', 'd', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'n', 'o', 'h', 'c', 'o', 't', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3818);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3819);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6783);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9005);
							s.is_empty()
						}
						['n', 'y', 'h', 't', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9006);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9007);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 10326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', s @ ..] => {
							super::push(out, 9008);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 10327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'a', 't', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 3820);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3821);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3822);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6784);
					match s {
						[] => true,
						['o', 'p', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9009);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3823);
			match s {
				[] => true,
				['j', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 6785);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sia (3 syllables).
pub(crate) fn n1422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1304);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3824);
			match s {
				[] => true,
				['m', 'g', 'e', 'l', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6786);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3825);
			match s {
				[] => true,
				['h', 't', 's', 'e', 'a', 'n', s @ ..] => {
					super::push(out, 6788);
					match s {
						[] => true,
						['y', 's', '\u{1}', s @ ..] => {
							super::push(out, 9011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', 'c', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9012);
							s.is_empty()
						}
						['k', 'k', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3826);
			match s {
				[] => true,
				['n', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 6789);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tia (3 syllables).
pub(crate) fn n1423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1305);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3827);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 6790);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'e', 'f', 'f', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6792);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6793);
					match s {
						[] => true,
						['h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'n', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -via (3 syllables).
pub(crate) fn n1424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1306);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3829);
			match s {
				[] => true,
				['r', 't', 's', 'i', 'n', 'i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6794);
					s.is_empty()
				}
				['t', 'a', 'r', 't', 's', 'i', 'n', 'i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6795);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -la (3 syllables).
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 502);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['b', 'm', s @ ..] => {
					super::push(out, 3865);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6811);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 3866);
					match s {
						[] => true,
						['a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1346);
			match s {
				[] => true,
				['h', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3867);
					s.is_empty()
				}
				['s', 'p', 'y', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3868);
					s.is_empty()
				}
				['u', 'q', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 3869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3870);
					match s {
						[] => true,
						['o', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 3871);
					s.is_empty()
				}
				['h', 'p', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 3872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3873);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3874);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6815);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 10328);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 10329);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'u', '\u{1}', s @ ..] => {
									super::push(out, 9018);
									s.is_empty()
								}
								['r', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 9019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9020);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 10330);
											s.is_empty()
										}
										['t', 'n', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 10331);
											s.is_empty()
										}
										['v', 'a', 'n', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10332);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 6816);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6817);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6818);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['h', 's', '\u{1}', s @ ..] => {
											super::push(out, 10333);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 10334);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6819);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 9021);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 10335);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'a', 'x', 'o', 'o', 'z', '\u{1}', s @ ..] => {
									super::push(out, 9022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', s @ ..] => {
							super::push(out, 6820);
							match s {
								[] => true,
								['l', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 9023);
									s.is_empty()
								}
								['y', 'c', '\u{1}', s @ ..] => super::t3::n9871(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => super::t3::n7379(s, out),
						['p', s @ ..] => super::t3::n7380(s, out),
						['r', s @ ..] => super::t3::n7381(s, out),
						['s', s @ ..] => super::t3::n7382(s, out),
						['t', s @ ..] => super::t3::n7383(s, out),
						['z', s @ ..] => super::t3::n7384(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t3::n4153(s, out),
				['o', 'b', 'a', '\u{1}', s @ ..] => super::t3::n4154(s, out),
				['u', 'p', 'a', 'p', '\u{1}', s @ ..] => super::t3::n4155(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t3::n1469(s, out),
		['s', s @ ..] => super::t3::n1470(s, out),
		['u', s @ ..] => super::t3::n1471(s, out),
		['y', s @ ..] => super::t3::n1472(s, out),
		_ => false,
	}
}

/// Words ending -ma (3 syllables).
pub(crate) fn n513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 505);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1364);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3922);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6888);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 9097);
									s.is_empty()
								}
								['t', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 9098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', 'i', 'r', 'a', 's', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3923);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 3924);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6889);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9099);
									s.is_empty()
								}
								['a', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3925);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3926);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3927);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6891);
							match s {
								[] => true,
								['k', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 9101);
									s.is_empty()
								}
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
			super::push(out, 1365);
			match s {
				[] => true,
				['b', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3928);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3929);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['r', 'o', 'p', 'y', 'h', '\u{1}', s @ ..] => {
									super::push(out, 9102);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10356);
											s.is_empty()
										}
										['r', 'e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
											super::push(out, 10357);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'r', 'i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6892);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 9103);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'b', s @ ..] => {
					super::push(out, 3930);
					match s {
						[] => true,
						['m', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6893);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 't', 'y', 'h', 'c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3931);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6895);
							s.is_empty()
						}
						['e', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6896);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6897);
							s.is_empty()
						}
						['n', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 6898);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', 'p', 'o', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 9105);
									s.is_empty()
								}
								['l', 'u', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9106);
									s.is_empty()
								}
								['m', 'o', 'r', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9107);
									s.is_empty()
								}
								['r', 'o', 'l', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9108);
									s.is_empty()
								}
								['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'c', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6900);
							s.is_empty()
						}
						['o', 'c', 'i', 'l', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'o', 't', 'e', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3932);
					s.is_empty()
				}
				['z', 'c', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1366);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3934);
					match s {
						[] => true,
						['l', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6902);
							s.is_empty()
						}
						['m', 'o', 'y', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6903);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', 'o', 'h', 'c', 'i', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 9110);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 9111);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 10358);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'p', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3935);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6904);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9112);
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									super::push(out, 9113);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9114);
									s.is_empty()
								}
								['o', 'r', 'h', 't', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9115);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6905);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9116);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
											super::push(out, 10359);
											s.is_empty()
										}
										['r', 'e', 't', 'p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 10955);
													s.is_empty()
												}
												['o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
													super::push(out, 10956);
													s.is_empty()
												}
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
				['n', 'o', 'l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3936);
					s.is_empty()
				}
				['o', 'd', 'r', 'e', 't', 'n', 'u', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3937);
					s.is_empty()
				}
				['u', 'r', 'e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3938);
					s.is_empty()
				}
				['y', 'r', 'e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 3939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1367);
			match s {
				[] => true,
				['c', 'a', 'r', 'd', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 3940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n1489(s, out),
		['l', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['a', 'g', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3947);
					s.is_empty()
				}
				['e', 't', 'o', 't', 'y', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t3::n1491(s, out),
		['o', s @ ..] => super::t3::n1492(s, out),
		['r', s @ ..] => super::t3::n1493(s, out),
		['s', s @ ..] => super::t3::n1494(s, out),
		['t', s @ ..] => super::t3::n1495(s, out),
		['u', s @ ..] => super::t3::n1496(s, out),
		['y', s @ ..] => super::t3::n1497(s, out),
		_ => false,
	}
}

/// The word nozawana (3 syllables).
pub(crate) fn n9992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9136);
	s.is_empty()
}

/// The word motswana (3 syllables).
pub(crate) fn n7539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6950);
	s.is_empty()
}

/// Words ending -ena (3 syllables).
pub(crate) fn n1509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1387);
	match s {
		[] => true,
		['a', 't', 'c', 'y', 'l', 'h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3987);
			s.is_empty()
		}
		['b', 'a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 3988);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3989);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'g', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 3990);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6952);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 9137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'c', 'y', 'l', 'h', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gna (3 syllables).
pub(crate) fn n1510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1388);
	s.is_empty()
}

/// Words ending -ina (3 syllables).
pub(crate) fn n1511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1389);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3992);
			match s {
				[] => true,
				['k', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3993);
			match s {
				[] => true,
				['o', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 6955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'i', 'f', 'f', 'o', '\u{1}', s @ ..] => {
			super::push(out, 3994);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3995);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6956);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3996);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6957);
					match s {
						[] => true,
						['n', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 9139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'c', 'u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3997);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3998);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3999);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6959);
					s.is_empty()
				}
				['e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4000);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6961);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 9140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4001);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4002);
			match s {
				[] => true,
				['a', 'i', s @ ..] => {
					super::push(out, 6962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'i', 'h', 'r', '\u{1}', s @ ..] => {
			super::push(out, 4003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lna (3 syllables).
pub(crate) fn n1512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1390);
	match s {
		[] => true,
		['u', 'o', 'i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
			super::push(out, 4004);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word alumna (3 syllables).
pub(crate) fn n1513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1391);
	s.is_empty()
}

/// Words ending -nna (3 syllables).
pub(crate) fn n1514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1392);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4005);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4006);
			match s {
				[] => true,
				['t', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6963);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ona (3 syllables).
pub(crate) fn n1515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1393);
	match s {
		[] => true,
		['g', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 4007);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4008);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6964);
					match s {
						[] => true,
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 9141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4009);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4010);
			match s {
				[] => true,
				['a', 't', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6965);
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rna (3 syllables).
pub(crate) fn n1516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1394);
	match s {
		[] => true,
		['e', 't', 's', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4011);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -una (3 syllables).
pub(crate) fn n1517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1395);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4012);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9142);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', 'y', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 10370);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10371);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10372);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['a', 'm', '\u{1}', s @ ..] => {
													super::push(out, 11258);
													s.is_empty()
												}
												['i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 11259);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 10957);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10373);
									s.is_empty()
								}
								['t', 'i', 's', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10374);
									s.is_empty()
								}
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
			super::push(out, 4013);
			match s {
				[] => true,
				['i', 'h', 'c', 'a', 'w', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6967);
					s.is_empty()
				}
				['o', 'r', 'o', 'g', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 4014);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4015);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4016);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4017);
			match s {
				[] => true,
				['u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4018);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vna (3 syllables).
pub(crate) fn n1518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1396);
	s.is_empty()
}

/// Words ending -para (3 syllables).
pub(crate) fn n4349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4047);
	match s {
		[] => true,
		['i', 'v', 'o', '\u{1}', s @ ..] => {
			super::push(out, 6981);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9144);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9145);
					match s {
						[] => true,
						['i', 'g', 'r', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 10375);
							s.is_empty()
						}
						['y', 'g', '\u{1}', s @ ..] => {
							super::push(out, 10376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wara (3 syllables).
pub(crate) fn n4352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4050);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6984);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9146);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 9147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bra (3 syllables).
pub(crate) fn n1550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1428);
	match s {
		[] => true,
		['a', 'l', 'o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 4051);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6985);
					s.is_empty()
				}
				['n', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 6986);
					s.is_empty()
				}
				['p', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6987);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6988);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9148);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'u', 't', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4052);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dra (3 syllables).
pub(crate) fn n1551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1429);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 4053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -era (3 syllables).
pub(crate) fn n1552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1430);
	match s {
		[] => true,
		['a', 't', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 4054);
			s.is_empty()
		}
		['c', 'i', s @ ..] => {
			super::push(out, 4055);
			match s {
				[] => true,
				['l', 'e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4056);
			match s {
				[] => true,
				['i', 'n', 'i', 'm', 'a', 'r', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', s @ ..] => {
			super::push(out, 4057);
			match s {
				[] => true,
				['h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4058);
			match s {
				[] => true,
				['u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 6992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'o', s @ ..] => {
			super::push(out, 4059);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4060);
			match s {
				[] => true,
				['s', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 6993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4061);
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6994);
					s.is_empty()
				}
				['y', 'b', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 4062);
			match s {
				[] => true,
				['a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6996);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hra (3 syllables).
pub(crate) fn n1553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1431);
	match s {
		[] => true,
		['t', 'e', 'l', 'y', 't', 'c', 'a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 4063);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ira (3 syllables).
pub(crate) fn n1554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1432);
	match s {
		[] => true,
		['a', 't', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 4064);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4065);
			s.is_empty()
		}
		['f', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 4066);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4067);
			match s {
				[] => true,
				['p', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 6997);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6998);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9150);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10958);
											s.is_empty()
										}
										['e', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10959);
											s.is_empty()
										}
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

/// Words ending -ora (3 syllables).
pub(crate) fn n1555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1433);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4068);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6999);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 9151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', 'n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4069);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4070);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4071);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 7000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['p', 'o', 'n', 'i', 't', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7001);
					s.is_empty()
				}
				['t', 'e', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4072);
			match s {
				[] => true,
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 7003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4073);
			match s {
				[] => true,
				['f', 'o', s @ ..] => {
					super::push(out, 7004);
					match s {
						[] => true,
						['c', 'y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9152);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9153);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10377);
									match s {
										[] => true,
										['a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10960);
											s.is_empty()
										}
										['i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10961);
											s.is_empty()
										}
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
			super::push(out, 4074);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4075);
			match s {
				[] => true,
				['s', 'o', 'i', 'o', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'a', s @ ..] => {
			super::push(out, 4076);
			s.is_empty()
		}
		['v', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 4077);
			s.is_empty()
		}
		['w', 'i', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 4078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rra (3 syllables).
pub(crate) fn n1556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1434);
	s.is_empty()
}

/// Words ending -tra (3 syllables).
pub(crate) fn n1557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1435);
	match s {
		[] => true,
		['s', 'i', 'a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ura (3 syllables).
pub(crate) fn n1558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1436);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4080);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4081);
			match s {
				[] => true,
				['a', 'y', '\u{1}', s @ ..] => {
					super::push(out, 7007);
					s.is_empty()
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 4082);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 4083);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7009);
					match s {
						[] => true,
						['b', 'a', 'y', 'k', '\u{1}', s @ ..] => {
							super::push(out, 9154);
							s.is_empty()
						}
						['m', 'i', 'k', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4084);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4085);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4086);
			match s {
				[] => true,
				['e', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7010);
					s.is_empty()
				}
				['i', 'c', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4087);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7012);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7013);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9156);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 't', '\u{1}', s @ ..] => {
									super::push(out, 10378);
									s.is_empty()
								}
								['u', 'j', '\u{1}', s @ ..] => {
									super::push(out, 10379);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'd', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7014);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 9158);
							s.is_empty()
						}
						['r', 'o', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9159);
							s.is_empty()
						}
						['s', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7015);
					s.is_empty()
				}
				['r', 'e', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7016);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ephyra (3 syllables).
pub(crate) fn n1559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1437);
	s.is_empty()
}

/// Words ending -vibrissa (3 syllables).
pub(crate) fn n12091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10962);
	s.is_empty()
}

/// Words ending -aeta (3 syllables).
pub(crate) fn n4431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4126);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 7035);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10381);
							s.is_empty()
						}
						['y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 10382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -seta (3 syllables).
pub(crate) fn n4433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4128);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 7037);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9169);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -weta (3 syllables).
pub(crate) fn n4434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4129);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 7038);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9170);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 10383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ita (3 syllables).
pub(crate) fn n1586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1462);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4130);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4131);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7039);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10384);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10385);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'u', 'e', 'z', '\u{1}', s @ ..] => {
			super::push(out, 4132);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4133);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4134);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 7040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4135);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4136);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					super::push(out, 7041);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 9171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4137);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7042);
					match s {
						[] => true,
						['p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ulta (3 syllables).
pub(crate) fn n1587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1463);
	match s {
		[] => true,
		['p', 'a', 't', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4138);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nta (3 syllables).
pub(crate) fn n1588(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'c', 'a', 'l', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4139);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4140);
			match s {
				[] => true,
				['s', 'c', 'a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ota (3 syllables).
pub(crate) fn n1589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1464);
	match s {
		[] => true,
		['c', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 4141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sta (3 syllables).
pub(crate) fn n1590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1465);
	match s {
		[] => true,
		['a', 't', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4142);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4143);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 7044);
					match s {
						[] => true,
						['o', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 9173);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4144);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 9174);
							s.is_empty()
						}
						['d', 'a', 'u', 'q', 's', '\u{1}', s @ ..] => {
							super::push(out, 9175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4145);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7045);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9176);
							s.is_empty()
						}
						['i', 's', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9177);
							s.is_empty()
						}
						['o', 't', 'p', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 9178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'c', 'o', 'l', '\u{1}', s @ ..] => {
			super::push(out, 4146);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tta (3 syllables).
pub(crate) fn n1591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1466);
	match s {
		[] => true,
		['e', 'z', s @ ..] => {
			super::push(out, 4147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uta (3 syllables).
pub(crate) fn n1592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1467);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 4148);
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 4149);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word pileorhiza (3 syllables).
pub(crate) fn n12092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10963);
	s.is_empty()
}

/// The word coleorhiza (3 syllables).
pub(crate) fn n12093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10964);
	s.is_empty()
}

/// The word ichomage (3 syllables).
pub(crate) fn n12094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10965);
	s.is_empty()
}

/// The word oichomage (3 syllables).
pub(crate) fn n12095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10966);
	s.is_empty()
}

/// Words ending -ashinage (3 syllables).
pub(crate) fn n12096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10967);
	s.is_empty()
}

/// Words ending -kurinage (3 syllables).
pub(crate) fn n12097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10968);
	s.is_empty()
}

/// Words ending -nge (3 syllables).
pub(crate) fn n1806(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4313);
			s.is_empty()
		}
		['e', 'l', 'u', 'm', 'a', 'y', 'n', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 4314);
			s.is_empty()
		}
		['o', 'n', 'i', 'k', 's', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4315);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oge (3 syllables).
pub(crate) fn n1807(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4316);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4317);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 7134);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tole (3 syllables).
pub(crate) fn n4701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4386);
	s.is_empty()
}

/// Words ending -ule (3 syllables).
pub(crate) fn n1853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1720);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 4387);
			s.is_empty()
		}
		['k', 'a', '\u{1}', s @ ..] => {
			super::push(out, 4388);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yle (3 syllables).
pub(crate) fn n1854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1721);
	match s {
		[] => true,
		['d', 'o', s @ ..] => {
			super::push(out, 4389);
			s.is_empty()
		}
		['t', 'o', s @ ..] => {
			super::push(out, 4390);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -atone (3 syllables).
pub(crate) fn n7808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7204);
	match s {
		[] => true,
		['g', 'i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 9243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ytone (3 syllables).
pub(crate) fn n7809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7205);
	s.is_empty()
}

/// Words ending -zone (3 syllables).
pub(crate) fn n4749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4434);
	s.is_empty()
}

/// Words ending -rne (3 syllables).
pub(crate) fn n1878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1745);
	s.is_empty()
}

/// Words ending -une (3 syllables).
pub(crate) fn n1879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1746);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 4435);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7206);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4436);
			match s {
				[] => true,
				['t', 'i', s @ ..] => {
					super::push(out, 7208);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10398);
									s.is_empty()
								}
								['a', 't', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 10399);
									s.is_empty()
								}
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

/// Words ending -ese (3 syllables).
pub(crate) fn n1911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1777);
	match s {
		[] => true,
		['b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 4471);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 4472);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7236);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4473);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7237);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'n', 'i', 'k', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4474);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7238);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9262);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4475);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 9264);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 9265);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 9266);
							match s {
								[] => true,
								['g', 'n', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 10400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'c', 'i', 'f', 'f', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9267);
							s.is_empty()
						}
						['n', 'r', 'u', 'o', 'j', '\u{1}', s @ ..] => {
							super::push(out, 9268);
							s.is_empty()
						}
						['t', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'n', 'i', 't', 'n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 7239);
					s.is_empty()
				}
				['l', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7240);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 9270);
							s.is_empty()
						}
						['r', 'y', 't', '\u{1}', s @ ..] => {
							super::push(out, 9271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 9272);
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 9273);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9274);
							s.is_empty()
						}
						['r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9275);
							s.is_empty()
						}
						['s', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'c', 's', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4476);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9277);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 9278);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10401);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 10969);
											match s {
												[] => true,
												['m', s @ ..] => {
													super::push(out, 11260);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', 'z', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'o', 'l', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7243);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['k', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9280);
							s.is_empty()
						}
						['s', 's', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', 'y', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9282);
							s.is_empty()
						}
						['s', 'r', 'e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7244);
					s.is_empty()
				}
				['u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4477);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4478);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7246);
					match s {
						[] => true,
						['r', 'r', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9284);
							s.is_empty()
						}
						['s', 's', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 9285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', 'l', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7247);
					s.is_empty()
				}
				['d', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7248);
					s.is_empty()
				}
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7249);
					s.is_empty()
				}
				['o', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7250);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7251);
					s.is_empty()
				}
				['r', 'a', 'v', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7252);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', 'l', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9286);
							s.is_empty()
						}
						['p', 'i', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9287);
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
				['k', 'r', 'o', 'y', '\u{1}', s @ ..] => {
					super::push(out, 7253);
					s.is_empty()
				}
				['n', 'o', 'm', 'd', 'e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4479);
			s.is_empty()
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7255);
					s.is_empty()
				}
				['e', 'n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7256);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oose (3 syllables).
pub(crate) fn n4803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4483);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 7258);
			match s {
				[] => true,
				['a', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 9289);
					s.is_empty()
				}
				['d', 'l', 'e', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 9290);
					s.is_empty()
				}
				['h', 'c', 't', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 9291);
					s.is_empty()
				}
				['i', 'h', 'c', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9292);
					s.is_empty()
				}
				['l', 'e', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 9293);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9294);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 10970);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 10971);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10972);
									s.is_empty()
								}
								['t', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9295);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sse (3 syllables).
pub(crate) fn n1917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1783);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 4484);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 7259);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9296);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10402);
									match s {
										[] => true,
										['e', 'g', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10974);
											s.is_empty()
										}
										['i', 'e', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 10975);
											s.is_empty()
										}
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
			super::push(out, 4485);
			s.is_empty()
		}
		_ => false,
	}
}

