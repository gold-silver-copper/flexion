use flexion::Path;

/// Words ending -χοσ.
pub(crate) fn n663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2186);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4416);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 4417);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4418);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6114);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 6115);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 6116);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4420);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6117);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6964);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7287);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 6118);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6965);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7288);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7379);
													s.is_empty()
												}
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
				['ν', s @ ..] => {
					super::push(out, 4421);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6119);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4423);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6121);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 6122);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6967);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4424);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4425);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 4426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2187);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4427);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 4428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6123);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 6124);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4429);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4430);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6125);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ῥ', s @ ..] => {
							super::push(out, 6126);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2188);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4431);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6127);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2189);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4432);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2190);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4433);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4434);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4435);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6131);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4436);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6133);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6972);
									s.is_empty()
								}
								['ϝ', s @ ..] => {
									super::push(out, 6973);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 6134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4437);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2191);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4438);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2192);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4439);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2193);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4440);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4441);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4442);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6138);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4443);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6140);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6142);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 6143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2194);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4444);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2195);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4446);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4447);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 4448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2196);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4449);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t2::n6184(s, out),
						['τ', s @ ..] => super::t2::n6185(s, out),
						_ => false,
					}
				}
				['μ', s @ ..] => super::t2::n4480(s, out),
				['ο', s @ ..] => super::t2::n4481(s, out),
				['ρ', s @ ..] => super::t2::n4482(s, out),
				['ψ', s @ ..] => super::t2::n4483(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t2::n2211(s, out),
		_ => false,
	}
}

/// Words ending -ψοσ.
pub(crate) fn n664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2198);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4455);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 4456);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 4457);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4458);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2199);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4459);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6154);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2200);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2201);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4460);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 4461);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρηνυσ.
pub(crate) fn n4520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4490);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 6168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κνυσ.
pub(crate) fn n2248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2234);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 4491);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξυσ.
pub(crate) fn n679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 674);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2235);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4492);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 4493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2236);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ουσ.
pub(crate) fn n680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 675);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2237);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 2238);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 2239);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4494);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6169);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4495);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6170);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 6171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2240);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4496);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6172);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2241);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4497);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6173);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6988);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 6174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2242);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4498);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6175);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2243);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4499);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 6176);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2244);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4500);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 4501);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6177);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6991);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7291);
											s.is_empty()
										}
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
		['τ', s @ ..] => {
			super::push(out, 2245);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4502);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6178);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 2246);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4503);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 4504);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4505);
					match s {
						[] => true,
						['ρ', 'τ', 'υ', 'ο', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 6180);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6181);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ῥ', s @ ..] => {
			super::push(out, 2247);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πυσ.
pub(crate) fn n681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 676);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2248);
			match s {
				[] => true,
				['ῥ', s @ ..] => {
					super::push(out, 4506);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρυσ.
pub(crate) fn n682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 677);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 2249);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4507);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 2250);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4508);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6183);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6994);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2251);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2252);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2253);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4511);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 2254);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4512);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6185);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 2255);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4513);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -συσ.
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 678);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2256);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4514);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 6187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4515);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τυσ.
pub(crate) fn n684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 679);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 2257);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2258);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4516);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6188);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6996);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2259);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4517);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2260);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4518);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6190);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6997);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7292);
											s.is_empty()
										}
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
		['τ', s @ ..] => {
			super::push(out, 2261);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4519);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φυσ.
pub(crate) fn n685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 680);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2262);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2263);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4520);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2264);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4521);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χυσ.
pub(crate) fn n686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 681);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2265);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 4522);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6193);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6998);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7293);
											s.is_empty()
										}
										['φ', s @ ..] => {
											super::push(out, 7294);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4523);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6194);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2266);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2267);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word αγορανομια.
pub(crate) fn n7422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7380);
	s.is_empty()
}

/// Words ending -ριστοκρατια.
pub(crate) fn n7423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7381);
	s.is_empty()
}

/// Words ending -ννηιον.
pub(crate) fn n4959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4928);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 6367);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7067);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θιον.
pub(crate) fn n1395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1386);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2918);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4929);
					match s {
						[] => true,
						['ρ', s @ ..] => {
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

/// Words ending -κιον.
pub(crate) fn n1396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1387);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2919);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4930);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6369);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7068);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 7313);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4931);
					match s {
						[] => true,
						['σ', 'υ', s @ ..] => {
							super::push(out, 6370);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 7069);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7314);
											s.is_empty()
										}
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
		['ι', s @ ..] => {
			super::push(out, 2920);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4932);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4933);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6371);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7070);
									match s {
										[] => true,
										['ζ', s @ ..] => {
											super::push(out, 7315);
											s.is_empty()
										}
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
		['ρ', s @ ..] => {
			super::push(out, 2921);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4934);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4935);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6373);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 7071);
									s.is_empty()
								}
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

/// Words ending -λιον.
pub(crate) fn n1397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1388);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2922);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 2923);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4936);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'β', s @ ..] => {
					super::push(out, 4937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2924);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4938);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6375);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 7072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2925);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4939);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6376);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7073);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7316);
											s.is_empty()
										}
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
		['ι', s @ ..] => {
			super::push(out, 2926);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2927);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4941);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6377);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 6378);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νιον.
pub(crate) fn n1399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1390);
	match s {
		[] => true,
		['α', s @ ..] => {
			match s {
				[] => true,
				['λ', 'κ', s @ ..] => {
					super::push(out, 4942);
					s.is_empty()
				}
				['ρ', 'θ', s @ ..] => {
					super::push(out, 4943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2929);
			s.is_empty()
		}
		['η', s @ ..] => {
			match s {
				[] => true,
				['λ', 'π', s @ ..] => {
					super::push(out, 4944);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'ο', s @ ..] => {
			super::push(out, 2930);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 4946);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2931);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2932);
			s.is_empty()
		}
		['π', 'ι', s @ ..] => {
			super::push(out, 2933);
			s.is_empty()
		}
		['ρ', 'α', s @ ..] => {
			super::push(out, 2934);
			s.is_empty()
		}
		['ω', 'φ', s @ ..] => {
			super::push(out, 2935);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξιον.
pub(crate) fn n1400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1391);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 2936);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οιον.
pub(crate) fn n1401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1392);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 2937);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2938);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πιον.
pub(crate) fn n1402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1393);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 2940);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4951);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6381);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 7075);
									s.is_empty()
								}
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

/// Words ending -ριον.
pub(crate) fn n1403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1394);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2941);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4952);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 6382);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 7076);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7317);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7382);
													s.is_empty()
												}
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
				['σ', s @ ..] => {
					super::push(out, 4953);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6383);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7077);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', 'ν', s @ ..] => {
			super::push(out, 2942);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4954);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 6384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2943);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4955);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4956);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4957);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 4958);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4959);
					match s {
						[] => true,
						['σ', 'η', s @ ..] => {
							super::push(out, 6385);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 7078);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7318);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7383);
													s.is_empty()
												}
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
		['θ', s @ ..] => {
			super::push(out, 2944);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4960);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2945);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2946);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4961);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6387);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2947);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 2948);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σιον.
pub(crate) fn n1404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1395);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2949);
			match s {
				[] => true,
				['ρ', 'κ', s @ ..] => {
					super::push(out, 4962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2950);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4963);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6388);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2951);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4964);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6389);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2952);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 4965);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 6390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τιον.
pub(crate) fn n1405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1396);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2953);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2954);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 4966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2955);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4967);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2956);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4968);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 6392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2957);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 4969);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6393);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 7082);
									s.is_empty()
								}
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

/// Words ending -φιον.
pub(crate) fn n1406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1397);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 2958);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 4970);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χιον.
pub(crate) fn n1407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1398);
	match s {
		[] => true,
		['γ', 'ο', s @ ..] => {
			super::push(out, 2959);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'α', s @ ..] => {
			super::push(out, 2960);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υρον.
pub(crate) fn n1453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1444);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3052);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 5025);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 5026);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 3053);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 3054);
			s.is_empty()
		}
		['ξ', s @ ..] => {
			super::push(out, 3055);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 3056);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5028);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3057);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5030);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6425);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 7097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3058);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 3059);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5031);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word αρποκρατησ.
pub(crate) fn n7426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7384);
	s.is_empty()
}

/// The word αδριανουπολιτησ.
pub(crate) fn n7427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7385);
	s.is_empty()
}

/// Words ending -νιτησ.
pub(crate) fn n3505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3478);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5285);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 6521);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 7124);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 6522);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7125);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 7332);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7386);
											s.is_empty()
										}
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
		['χ', s @ ..] => {
			super::push(out, 5286);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 6523);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 7126);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 5287);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 6524);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7127);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7334);
									match s {
										[] => true,
										['σ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 7387);
											s.is_empty()
										}
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

/// Words ending -ριτησ.
pub(crate) fn n3507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3480);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 5289);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6526);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 6527);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 7128);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7335);
									s.is_empty()
								}
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

/// Words ending -κτησ.
pub(crate) fn n1848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1836);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3481);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5290);
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 6528);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 6529);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3482);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 3483);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3484);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 3485);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 3486);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ντησ.
pub(crate) fn n1850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1838);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3489);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3490);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 3491);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 5293);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οτησ.
pub(crate) fn n1851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1839);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 3492);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5294);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6531);
							match s {
								[] => true,
								['π', '\u{1}', s @ ..] => {
									super::push(out, 7130);
									s.is_empty()
								}
								['τ', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
									super::push(out, 7131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3493);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5295);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6532);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7132);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7336);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 5296);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5297);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 6533);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 7133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3494);
			s.is_empty()
		}
		['μ', 'η', s @ ..] => {
			super::push(out, 3495);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 5298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 3496);
			s.is_empty()
		}
		['ξ', 'ο', 'τ', s @ ..] => {
			super::push(out, 3497);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5299);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5300);
					match s {
						[] => true,
						['π', 'π', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 6535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3498);
			match s {
				[] => true,
				['ε', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 5301);
					s.is_empty()
				}
				['μ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 5302);
					s.is_empty()
				}
				['ο', 'λ', 'ι', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 5303);
					s.is_empty()
				}
				['π', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 5304);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 5305);
					match s {
						[] => true,
						['ε', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 6536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', 'γ', s @ ..] => {
			super::push(out, 3499);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πτησ.
pub(crate) fn n1852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1840);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 3501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρτησ.
pub(crate) fn n1853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1841);
	s.is_empty()
}

/// Words ending -στησ.
pub(crate) fn n1854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1842);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3502);
			match s {
				[] => true,
				['λ', s @ ..] => {
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6538);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 7134);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 6539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'υ', s @ ..] => {
					super::push(out, 5307);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'ε', 'κ', s @ ..] => {
					super::push(out, 5308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'γ', s @ ..] => {
			super::push(out, 3503);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5309);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3504);
			match s {
				[] => true,
				['ρ', 'π', s @ ..] => {
					super::push(out, 5310);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6542);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 7135);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7338);
											s.is_empty()
										}
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
		['υ', s @ ..] => {
			super::push(out, 3505);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5311);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 5312);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 5313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 3506);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5314);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 5315);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υτησ.
pub(crate) fn n1855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1843);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3507);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 3508);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5317);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6547);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 7136);
									match s {
										[] => true,
										['π', '\u{1}', s @ ..] => {
											super::push(out, 7339);
											s.is_empty()
										}
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
		['δ', s @ ..] => {
			super::push(out, 3509);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5318);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6548);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 7137);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 5319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3510);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5320);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5321);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5322);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5323);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6549);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 7138);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7340);
											s.is_empty()
										}
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
		['λ', s @ ..] => {
			super::push(out, 3511);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5324);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6550);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5325);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6551);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7140);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 5326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 3512);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3513);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5328);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 3514);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5329);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 3515);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5330);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6555);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωτησ.
pub(crate) fn n1856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1844);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3516);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5331);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 6557);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 7141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', 'η', s @ ..] => {
			super::push(out, 3517);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 5332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 3518);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3519);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5334);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5335);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5336);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word λιμεναρχησ.
pub(crate) fn n7430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7388);
	s.is_empty()
}

/// Words ending -ουρισ.
pub(crate) fn n3671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3644);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 5426);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 6604);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 7156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πυρισ.
pub(crate) fn n3672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3645);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 5427);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 6605);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 7157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υσισ.
pub(crate) fn n1953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1941);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3678);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 3679);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 3680);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5452);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5453);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 5454);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 5455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3681);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5456);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 5457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3682);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5458);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5459);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3683);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5461);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 3684);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 3685);
			s.is_empty()
		}
		['ῥ', s @ ..] => {
			super::push(out, 3686);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωσισ.
pub(crate) fn n1954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1942);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 3687);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5464);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3688);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 5465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3689);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5466);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6619);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 7165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3690);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 3691);
			s.is_empty()
		}
		['ξ', s @ ..] => {
			super::push(out, 3692);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5467);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3693);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5468);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 5469);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6621);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', 'υ', 'λ', 'α', s @ ..] => {
					super::push(out, 5470);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6622);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', 'π', s @ ..] => {
					super::push(out, 5471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3694);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 3695);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word πλευριτισ.
pub(crate) fn n7431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7389);
	s.is_empty()
}

/// Words ending -μυστισ.
pub(crate) fn n5530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5496);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6637);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξυστισ.
pub(crate) fn n5531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5497);
	s.is_empty()
}

/// Words ending -πυστισ.
pub(crate) fn n5532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5498);
	s.is_empty()
}

/// Words ending -ωστισ.
pub(crate) fn n3746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3719);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 5499);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 6638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 5500);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 6639);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7347);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 7348);
									s.is_empty()
								}
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

/// Words ending -ττισ.
pub(crate) fn n1962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1950);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3720);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5501);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6640);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 7176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3721);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3722);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5502);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υτισ.
pub(crate) fn n1963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1951);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 3723);
			s.is_empty()
		}
		['ῥ', s @ ..] => {
			super::push(out, 3724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωτισ.
pub(crate) fn n1964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1952);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3725);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 3726);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3727);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 3728);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 3729);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μοργοσ.
pub(crate) fn n5590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5555);
	s.is_empty()
}

/// Words ending -τοργοσ.
pub(crate) fn n5591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5556);
	s.is_empty()
}

/// Words ending -υργοσ.
pub(crate) fn n3831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3804);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 5557);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 6671);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7182);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 5558);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σγοσ.
pub(crate) fn n2014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2002);
	s.is_empty()
}

/// Words ending -υγοσ.
pub(crate) fn n2015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2003);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3805);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 5559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωγοσ.
pub(crate) fn n2016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2004);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 3807);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5560);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6672);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7183);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7351);
											s.is_empty()
										}
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

/// Words ending -σθοσ.
pub(crate) fn n2046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2034);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3885);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5628);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 5629);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5630);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 3886);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τθοσ.
pub(crate) fn n2047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2035);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3887);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 5631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3888);
			match s {
				[] => true,
				['γ', 'ι', s @ ..] => {
					super::push(out, 5632);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υθοσ.
pub(crate) fn n2048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2036);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 3889);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 3890);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5633);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6707);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 3891);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 3892);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 5634);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 3893);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5635);
					match s {
						[] => true,
						['ψ', s @ ..] => {
							super::push(out, 6709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 3894);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3895);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5637);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5638);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 3896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χθοσ.
pub(crate) fn n2049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2037);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3897);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3898);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3899);
			match s {
				[] => true,
				['ρ', 'ο', s @ ..] => {
					super::push(out, 5641);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χαλκοσ.
pub(crate) fn n5727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5691);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 6738);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 7200);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ελκοσ.
pub(crate) fn n3994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3967);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υλκοσ.
pub(crate) fn n3996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3969);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 5693);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 6739);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οκοσ.
pub(crate) fn n2072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2060);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 3970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρκοσ.
pub(crate) fn n2073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2061);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3971);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5694);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3972);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3973);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5696);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σκοσ.
pub(crate) fn n2074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2062);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3974);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3975);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3976);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 3977);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5699);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 3978);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υκοσ.
pub(crate) fn n2075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2063);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3979);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 5700);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3980);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 5702);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3981);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5703);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3982);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5704);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3983);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 5706);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3984);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λλοσ.
pub(crate) fn n2084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2072);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4025);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5735);
					match s {
						[] => true,
						['ι', 'ρ', 'τ', s @ ..] => {
							super::push(out, 6762);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 6763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 5736);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 5737);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5738);
					s.is_empty()
				}
				['μ', 'υ', s @ ..] => {
					super::push(out, 5739);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 4026);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 5741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4027);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5742);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6764);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 4028);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5743);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6765);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 6766);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 7206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 5744);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6767);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 7207);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 5745);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ολοσ.
pub(crate) fn n2085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2073);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 4029);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5746);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5747);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 5748);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 5749);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 4030);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 4031);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 4032);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 4033);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 4034);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 5751);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6771);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7208);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7354);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 5752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 4035);
			s.is_empty()
		}
		['τ', 'σ', 'ο', s @ ..] => {
			super::push(out, 4036);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 4037);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 4038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -επλοσ.
pub(crate) fn n4067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4039);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 5754);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τλοσ.
pub(crate) fn n2087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2075);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 4040);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 4041);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υλοσ.
pub(crate) fn n2088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2076);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4042);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5755);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5756);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 5757);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 4043);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 5758);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 4044);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5759);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5760);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 4045);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 4046);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 4047);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 4048);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 5763);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 5764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 4049);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5765);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 5766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 4050);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 4051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χλοσ.
pub(crate) fn n2089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2077);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 4052);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 4053);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5768);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωλοσ.
pub(crate) fn n2090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2078);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 4054);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 4055);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 4056);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 4057);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 4058);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 4059);
			s.is_empty()
		}
		['ψ', s @ ..] => {
			super::push(out, 4060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ευκοφθαλμοσ.
pub(crate) fn n7432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7390);
	s.is_empty()
}

/// Words ending -υμοσ.
pub(crate) fn n2103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2091);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 4090);
			s.is_empty()
		}
		['θ', 'υ', s @ ..] => {
			super::push(out, 4091);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 4092);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5793);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5794);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6789);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 7215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 4093);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 5795);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6790);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4094);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 5796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 4095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εμενοσ.
pub(crate) fn n5867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5830);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6808);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμενοσ.
pub(crate) fn n5868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5831);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 6809);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξενοσ.
pub(crate) fn n4151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4122);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 5832);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 6810);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 7220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τενοσ.
pub(crate) fn n4152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4123);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 5833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φενοσ.
pub(crate) fn n4153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4124);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5834);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηνοσ.
pub(crate) fn n2110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2098);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 4125);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 5835);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6812);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 4126);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 4127);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 4128);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 4129);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 4130);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 5838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 4131);
			s.is_empty()
		}
		['τ', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 4132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θνοσ.
pub(crate) fn n2111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2099);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ινοσ.
pub(crate) fn n2112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2100);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4134);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 4135);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 4136);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 5840);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 6813);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 7222);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 4137);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5841);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 5842);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5843);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6815);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 4138);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5844);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 6816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 4139);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5845);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6817);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 6818);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7225);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7360);
											s.is_empty()
										}
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
		['ξ', s @ ..] => {
			super::push(out, 4140);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 4141);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5846);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 5847);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 4142);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 4143);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5849);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6820);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7226);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7361);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5850);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6821);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 7227);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 7362);
											s.is_empty()
										}
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
		['τ', s @ ..] => {
			super::push(out, 4144);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5851);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 5852);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5853);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 5854);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6824);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7228);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7363);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 5855);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 4145);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5856);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 4146);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ῥ', s @ ..] => {
			super::push(out, 4147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κνοσ.
pub(crate) fn n2113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2101);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 4148);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 4149);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μνοσ.
pub(crate) fn n2114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2102);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4150);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5858);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 5859);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6827);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7229);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 7364);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 6828);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 7230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 5860);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5861);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 5862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 4151);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5863);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4152);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 5864);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6831);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 7231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5865);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 4153);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 4154);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5867);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ννοσ.
pub(crate) fn n2115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2103);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4155);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5868);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6834);
							match s {
								[] => true,
								['τ', 'ο', s @ ..] => {
									super::push(out, 7232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 4156);
			match s {
				[] => true,
				['ξ', '\u{1}', s @ ..] => {
					super::push(out, 5869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4157);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5870);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4158);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 4159);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5871);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ονοσ.
pub(crate) fn n2116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2104);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 4160);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5872);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5873);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4161);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5874);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 6838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 4162);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5875);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6839);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 4163);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5876);
					match s {
						[] => true,
						['ο', 'ν', s @ ..] => {
							super::push(out, 6840);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 7234);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7365);
											s.is_empty()
										}
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
		['φ', s @ ..] => {
			super::push(out, 4164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πνοσ.
pub(crate) fn n2117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2105);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4165);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 4166);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 4167);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρνοσ.
pub(crate) fn n2118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2106);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4168);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 4169);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5877);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 5878);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 5879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4170);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 5880);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 4171);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5881);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5882);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τνοσ.
pub(crate) fn n2119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2107);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υνοσ.
pub(crate) fn n2120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2108);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4173);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5883);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5884);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6844);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 4174);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5885);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 5886);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 4175);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 4176);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φνοσ.
pub(crate) fn n2121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2109);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 4177);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5887);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χνοσ.
pub(crate) fn n2122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2110);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 4178);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 4179);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5889);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6847);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6848);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 4180);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 5890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωνοσ.
pub(crate) fn n2123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2111);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 4181);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5891);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4182);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 4183);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 4184);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5892);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6850);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 7237);
									s.is_empty()
								}
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

/// Words ending -φειροσ.
pub(crate) fn n5980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5942);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 6874);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κιροσ.
pub(crate) fn n4282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4253);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 5943);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οιροσ.
pub(crate) fn n4283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4254);
	match s {
		[] => true,
		['χ', s @ ..] => {
			super::push(out, 5944);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κροσ.
pub(crate) fn n2151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2137);
	s.is_empty()
}

/// Words ending -οροσ.
pub(crate) fn n2152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2138);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4256);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 4257);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5945);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 4258);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5947);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 4259);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5949);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6875);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 7251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 4260);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 4261);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5950);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 6876);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 7252);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 7368);
											s.is_empty()
										}
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
		['μ', s @ ..] => {
			super::push(out, 4262);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5951);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6877);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 4263);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 4264);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 4265);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5952);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6878);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7254);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 4266);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5953);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 5954);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5955);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 5956);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6879);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 7255);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7369);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 5957);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6880);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 7256);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 4267);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5958);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -προσ.
pub(crate) fn n2153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2139);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4268);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4269);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5960);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4270);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 4271);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5962);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρροσ.
pub(crate) fn n2154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2140);
	match s {
		[] => true,
		['α', 'θ', '\u{1}', s @ ..] => {
			super::push(out, 4272);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 4273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τροσ.
pub(crate) fn n2155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2141);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4274);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5963);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 5964);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 4275);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5965);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4276);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 4277);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5967);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 5968);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5969);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υροσ.
pub(crate) fn n2156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2142);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4278);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5970);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 6888);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 7257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 5971);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6889);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7258);
									match s {
										[] => true,
										['κ', 'ο', 'υ', s @ ..] => {
											super::push(out, 7370);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 7391);
													match s {
														[] => true,
														['χ', s @ ..] => super::t2::n7437(s, out),
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
						['σ', s @ ..] => {
							super::push(out, 6890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 4279);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5972);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5973);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 4280);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5974);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5975);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6893);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7259);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 4281);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5976);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 4282);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5977);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 5978);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6895);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 4283);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5979);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6896);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 7261);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 5980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4284);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5981);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6897);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7262);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5982);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6898);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 6899);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7263);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7371);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7392);
													s.is_empty()
												}
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
				['κ', s @ ..] => {
					super::push(out, 5983);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5984);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6900);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6901);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 6902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5985);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 6903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 5986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 4285);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5987);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 4286);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 4287);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5988);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φροσ.
pub(crate) fn n2157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2143);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4288);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 5989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 4289);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4290);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 5991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χροσ.
pub(crate) fn n2158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2144);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 4291);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 4292);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωροσ.
pub(crate) fn n2159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2145);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4293);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 4294);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 4295);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 4296);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 4297);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 5993);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 6906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 4298);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 4299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ναυσοσ.
pub(crate) fn n6056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6018);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 6922);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 7270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ουσοσ.
pub(crate) fn n4358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4329);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 6019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρυσοσ.
pub(crate) fn n4359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4330);
	match s {
		[] => true,
		['χ', s @ ..] => {
			super::push(out, 6020);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 6923);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 7271);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7375);
									s.is_empty()
								}
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

/// Words ending -οντοσ.
pub(crate) fn n4389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4360);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 6053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οτοσ.
pub(crate) fn n2176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2162);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 4361);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 6054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 4362);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 6055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 4363);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 6056);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6938);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 4364);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 4365);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 6057);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 6058);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πτοσ.
pub(crate) fn n2177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2163);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4366);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 6059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 4367);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρτοσ.
pub(crate) fn n2178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2164);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4368);
			match s {
				[] => true,
				['κ', '\u{1}', s @ ..] => {
					super::push(out, 6060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4369);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 6061);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 6062);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 6063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 4370);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 6064);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6940);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 7279);
									s.is_empty()
								}
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

/// Words ending -στοσ.
pub(crate) fn n2179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2165);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4371);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 6065);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 6066);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 4372);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 6067);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6943);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4373);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 6068);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 6944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4374);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 6069);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 6070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 4375);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 6071);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6945);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 7281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 6072);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ττοσ.
pub(crate) fn n2180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2166);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 4376);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 6073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 4377);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 6074);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υτοσ.
pub(crate) fn n2181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2167);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 4378);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 4379);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 6075);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 6076);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6948);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 4380);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 6077);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4381);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 6078);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 4382);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 6079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 4383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωτοσ.
pub(crate) fn n2182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2168);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 4384);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 6080);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 4385);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ορθογραφοσ.
pub(crate) fn n7435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7393);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7396);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νακογραφοσ.
pub(crate) fn n7436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7394);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 7397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κελυφοσ.
pub(crate) fn n7003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6962);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7286);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ουφοσ.
pub(crate) fn n4441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4412);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 6110);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρυφοσ.
pub(crate) fn n4442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4413);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 6111);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 6112);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συφοσ.
pub(crate) fn n4443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4414);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 6113);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ευχοσ.
pub(crate) fn n6184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6146);
	s.is_empty()
}

/// Words ending -τευχοσ.
pub(crate) fn n6185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6147);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6974);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μυχοσ.
pub(crate) fn n4480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4450);
	s.is_empty()
}

/// Words ending -ουχοσ.
pub(crate) fn n4481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4451);
	s.is_empty()
}

/// Words ending -ρυχοσ.
pub(crate) fn n4482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4452);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6975);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 6976);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 7289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 6149);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 6977);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 7290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψυχοσ.
pub(crate) fn n4483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4453);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6150);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωχοσ.
pub(crate) fn n2211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2197);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 4454);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 6151);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χθυοκενταυροσ.
pub(crate) fn n7437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7395);
	s.is_empty()
}

