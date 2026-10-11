use flexion::Path;

/// Words ending -uitas.
pub(crate) fn n3666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3626);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5803);
			match s {
				[] => true,
				['s', 'i', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5804);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5805);
			s.is_empty()
		}
		['l', 'f', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 5806);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5807);
			match s {
				[] => true,
				['e', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 7613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 5808);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7614);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7615);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5809);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7617);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 8759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vitas.
pub(crate) fn n3667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3627);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5810);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7618);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5811);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7620);
					s.is_empty()
				}
				['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5812);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7622);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7623);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9285);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9286);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7624);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8760);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 8761);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8762);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8763);
							match s {
								[] => true,
								['e', 'p', s @ ..] => {
									super::push(out, 9287);
									s.is_empty()
								}
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
			super::push(out, 5813);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xitas.
pub(crate) fn n3668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3628);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5814);
			s.is_empty()
		}
		['i', 'l', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5815);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iarius.
pub(crate) fn n6137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6044);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7730);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 9290);
							s.is_empty()
						}
						['l', 'a', 'n', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 9291);
							s.is_empty()
						}
						['t', 's', 'u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 9292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 7731);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7732);
			match s {
				[] => true,
				['e', 'p', 'u', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 8782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7733);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8783);
					s.is_empty()
				}
				['o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7734);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 7735);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7736);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7737);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7738);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8787);
					match s {
						[] => true,
						['r', 'a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 's', 'u', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8788);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8789);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8790);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -larius.
pub(crate) fn n6138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6045);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7739);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8791);
					s.is_empty()
				}
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 8792);
					s.is_empty()
				}
				['d', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 7740);
			s.is_empty()
		}
		['e', 'd', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 7741);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7742);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 7743);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8794);
					match s {
						[] => true,
						['c', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 9294);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9295);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 9460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'x', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7744);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7745);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8796);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9296);
							match s {
								[] => true,
								['n', 'e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9297);
							match s {
								[] => true,
								['r', 't', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9462);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9463);
									match s {
										[] => true,
										['u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 9492);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 9464);
									match s {
										[] => true,
										['a', 'l', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9493);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9298);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9299);
							match s {
								[] => true,
								['a', 'v', '\u{1}', s @ ..] => {
									super::push(out, 9465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8797);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8798);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 9300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8799);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8800);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -narius.
pub(crate) fn n6140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6047);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7750);
			match s {
				[] => true,
				['d', 'r', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8802);
					s.is_empty()
				}
				['g', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 8803);
					s.is_empty()
				}
				['t', 's', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7751);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8805);
					s.is_empty()
				}
				['g', 'a', 'u', 't', 'p', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8806);
					s.is_empty()
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7752);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7753);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8808);
					s.is_empty()
				}
				['g', 'i', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 8809);
					s.is_empty()
				}
				['h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8810);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8811);
					s.is_empty()
				}
				['m', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8812);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8813);
					match s {
						[] => true,
						['c', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9301);
							s.is_empty()
						}
						['u', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 8814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7754);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7755);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7756);
			s.is_empty()
		}
		['u', 'c', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 7757);
			s.is_empty()
		}
		['y', 's', 'o', 'm', 'e', 'e', 'l', 'e', '\u{1}', s @ ..] => {
			super::push(out, 7758);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rarius.
pub(crate) fn n6142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6049);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7760);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8816);
					match s {
						[] => true,
						['l', 'e', 'd', 'n', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7761);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8817);
					match s {
						[] => true,
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7762);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 8818);
					s.is_empty()
				}
				['t', 'c', 'e', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7763);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 8820);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 8821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7764);
			match s {
				[] => true,
				['t', 'c', 'e', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8822);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sarius.
pub(crate) fn n6143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6050);
	match s {
		[] => true,
		['n', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 7765);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7766);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7767);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8823);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tarius.
pub(crate) fn n6144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6051);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7768);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8824);
					match s {
						[] => true,
						['o', 'l', 'p', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8825);
					s.is_empty()
				}
				['r', 'e', 'b', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 8826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 7769);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7770);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 8828);
					s.is_empty()
				}
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8829);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8830);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'c', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8831);
					s.is_empty()
				}
				['u', 's', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8832);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 8833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7771);
			match s {
				[] => true,
				['m', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8834);
					s.is_empty()
				}
				['s', 'o', 'p', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7772);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					super::push(out, 8836);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9307);
							match s {
								[] => true,
								['l', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 9308);
							s.is_empty()
						}
						['g', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9309);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9310);
							match s {
								[] => true,
								['v', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7773);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7774);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8837);
					match s {
						[] => true,
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 9312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'v', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7775);
			s.is_empty()
		}
		['u', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 7776);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uarius.
pub(crate) fn n6145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6052);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7777);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7778);
			match s {
				[] => true,
				['a', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8839);
					s.is_empty()
				}
				['e', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 7779);
			match s {
				[] => true,
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'u', '\u{1}', s @ ..] => {
			super::push(out, 7780);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7781);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8842);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 9313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -varius.
pub(crate) fn n6146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6053);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7782);
			match s {
				[] => true,
				['s', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 8843);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -erius.
pub(crate) fn n4078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4032);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6054);
			match s {
				[] => true,
				['i', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', 'n', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6055);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orius.
pub(crate) fn n4079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4033);
	match s {
		[] => true,
		['t', 'a', s @ ..] => {
			super::push(out, 6056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urius.
pub(crate) fn n4080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4034);
	match s {
		[] => true,
		['d', 'l', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 6057);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yrius.
pub(crate) fn n4081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4035);
	s.is_empty()
}

/// Words ending -sculus.
pub(crate) fn n6208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6114);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7826);
			s.is_empty()
		}
		['o', 'l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 7827);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7828);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8878);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 9326);
							s.is_empty()
						}
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 9327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uculus.
pub(crate) fn n6209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6115);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 7829);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 7830);
			s.is_empty()
		}
		['d', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 7831);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dulus.
pub(crate) fn n4128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4082);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6116);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6117);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6118);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 7832);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gulus.
pub(crate) fn n4129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4083);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6119);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7833);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 7834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iulus.
pub(crate) fn n4130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4084);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6120);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mulus.
pub(crate) fn n4132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4086);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6121);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 6122);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 7835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6123);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6124);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nulus.
pub(crate) fn n4133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4087);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 6125);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6126);
			s.is_empty()
		}
		['n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pulus.
pub(crate) fn n4134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4088);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6128);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6129);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6130);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 7837);
					s.is_empty()
				}
				['r', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 7838);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rulus.
pub(crate) fn n4135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4089);
	match s {
		[] => true,
		['e', 'g', s @ ..] => {
			super::push(out, 6131);
			match s {
				[] => true,
				['i', 'g', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7839);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sulus.
pub(crate) fn n4136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4090);
	s.is_empty()
}

/// Words ending -tulus.
pub(crate) fn n4137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4091);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6132);
			match s {
				[] => true,
				['n', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6133);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 7841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6134);
			match s {
				[] => true,
				['a', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7842);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7843);
					match s {
						[] => true,
						['c', 's', 'e', 'l', 'u', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6135);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 7844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vulus.
pub(crate) fn n4138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4092);
	match s {
		[] => true,
		['i', 'r', '\u{1}', s @ ..] => {
			super::push(out, 6136);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dimentum.
pub(crate) fn n8471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8332);
	match s {
		[] => true,
		['e', 's', '\u{1}', s @ ..] => {
			super::push(out, 9102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gimentum.
pub(crate) fn n8472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8333);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9103);
			s.is_empty()
		}
		['r', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 9104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -limentum.
pub(crate) fn n8473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8334);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 9105);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9106);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9107);
			match s {
				[] => true,
				['m', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9401);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nimentum.
pub(crate) fn n8474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8335);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9108);
			match s {
				[] => true,
				['l', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 9109);
			s.is_empty()
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 9110);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rimentum.
pub(crate) fn n8475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8336);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9111);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9112);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9403);
					s.is_empty()
				}
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 9404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -timentum.
pub(crate) fn n8476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8337);
	match s {
		[] => true,
		['s', 'e', 'v', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 9113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vimentum.
pub(crate) fn n8477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8338);
	match s {
		[] => true,
		['o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 9114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mmentum.
pub(crate) fn n7048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6939);
	s.is_empty()
}

/// Words ending -omentum.
pub(crate) fn n7049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6940);
	s.is_empty()
}

/// Words ending -rmentum.
pub(crate) fn n7050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6941);
	s.is_empty()
}

/// Words ending -umentum.
pub(crate) fn n7051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6942);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8339);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 8340);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9115);
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 9116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8341);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8342);
			match s {
				[] => true,
				['o', 'm', 'e', '\u{1}', s @ ..] => {
					super::push(out, 9117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 8343);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8344);
			match s {
				[] => true,
				['t', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 9118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -guratio.
pub(crate) fn n7306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7192);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 8522);
			match s {
				[] => true,
				['f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 9190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iuratio.
pub(crate) fn n7307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7193);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8524);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 9191);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -turatio.
pub(crate) fn n7309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7195);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8525);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 9192);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yratio.
pub(crate) fn n5508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5436);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 7196);
			s.is_empty()
		}
		['t', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 7197);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utatio.
pub(crate) fn n5526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5454);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 7240);
			match s {
				[] => true,
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7241);
			match s {
				[] => true,
				['a', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 7242);
			match s {
				[] => true,
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 8566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7243);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 7244);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sibilitas.
pub(crate) fn n9427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9265);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 9452);
			match s {
				[] => true,
				['v', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 9491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 9453);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9454);
			s.is_empty()
		}
		['u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 9455);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tibilitas.
pub(crate) fn n9428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9266);
	s.is_empty()
}

/// Words ending -obilitas.
pub(crate) fn n8869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8719);
	s.is_empty()
}

/// Words ending -ubilitas.
pub(crate) fn n8870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8720);
	s.is_empty()
}

/// Words ending -cilitas.
pub(crate) fn n7673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7549);
	s.is_empty()
}

/// The word aedilitas.
pub(crate) fn n7674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7550);
	s.is_empty()
}

/// Words ending -gilitas.
pub(crate) fn n7675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7551);
	s.is_empty()
}

/// Words ending -militas.
pub(crate) fn n7676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7552);
	s.is_empty()
}

/// The word anilitas.
pub(crate) fn n7677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7553);
	s.is_empty()
}

/// Words ending -rilitas.
pub(crate) fn n7678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7554);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8721);
			s.is_empty()
		}
		['e', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 8722);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tilitas.
pub(crate) fn n7679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7555);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 8723);
			match s {
				[] => true,
				['e', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 9267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 8724);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8725);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 9268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vilitas.
pub(crate) fn n7680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7556);
	match s {
		[] => true,
		['i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 8726);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xilitas.
pub(crate) fn n7681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7557);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			super::push(out, 8727);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -llitas.
pub(crate) fn n5852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5768);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7558);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 7559);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word solitas.
pub(crate) fn n5853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5769);
	s.is_empty()
}

/// Words ending -ulitas.
pub(crate) fn n5854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5770);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7560);
			s.is_empty()
		}
		_ => false,
	}
}

