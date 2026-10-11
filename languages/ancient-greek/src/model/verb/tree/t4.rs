use flexion::Path;

/// Words ending -ρευω.
pub(crate) fn n973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 972);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2258);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3830);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5311);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6523);
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
		['γ', s @ ..] => {
			super::push(out, 2259);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2260);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3832);
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
		['ο', s @ ..] => {
			super::push(out, 2261);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3833);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5313);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6524);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6525);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7357);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7831);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9702(s, out),
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
									super::push(out, 6526);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7358);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7832);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9703(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3834);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5314);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6527);
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
				['χ', s @ ..] => {
					super::push(out, 3835);
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
		['τ', s @ ..] => {
			super::push(out, 2262);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3836);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5316);
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
		['ω', s @ ..] => {
			super::push(out, 2263);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3837);
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

/// Words ending -σευω.
pub(crate) fn n974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 973);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2264);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 2265);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3838);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5317);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6529);
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
		['ι', s @ ..] => {
			super::push(out, 2266);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3839);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5318);
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
		['σ', s @ ..] => {
			super::push(out, 2267);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3840);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5319);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6530);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7359);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5320);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6531);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7360);
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
		_ => false,
	}
}

/// Words ending -τευω.
pub(crate) fn n975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 974);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2268);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3841);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5321);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6532);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7361);
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
				['ρ', s @ ..] => {
					super::push(out, 3842);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6533);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5323);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6534);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 7362);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7833);
													match s {
														[] => true,
														['ε', s @ ..] => super::t4::n9704(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 7363);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7834);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9705(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 7364);
											match s {
												[] => true,
												['ξ', s @ ..] => {
													super::push(out, 7835);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9706(s, out),
														_ => false,
													}
												}
												['σ', s @ ..] => {
													super::push(out, 7836);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9707(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2269);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3843);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6535);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6536);
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
				['π', s @ ..] => {
					super::push(out, 3844);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5325);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6537);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7365);
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
		['η', s @ ..] => {
			super::push(out, 2270);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3845);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5326);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6538);
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
				['ο', s @ ..] => {
					super::push(out, 3846);
					match s {
						[] => true,
						['γ', s @ ..] => {
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
				['φ', s @ ..] => {
					super::push(out, 3847);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5328);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6539);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7366);
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
		['ι', s @ ..] => {
			super::push(out, 2271);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3848);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5329);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6540);
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
		['κ', s @ ..] => {
			super::push(out, 2272);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3849);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5330);
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
		['ο', s @ ..] => {
			super::push(out, 2273);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3850);
					match s {
						[] => true,
						['β', s @ ..] => {
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
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2274);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3851);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5332);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6541);
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
				['υ', s @ ..] => {
					super::push(out, 3852);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5333);
							match s {
								[] => true,
								['κ', s @ ..] => super::t4::n7339(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => super::t4::n2316(s, out),
		['υ', s @ ..] => super::t4::n2317(s, out),
		['ω', s @ ..] => super::t4::n2318(s, out),
		_ => false,
	}
}

/// Words ending -φευω.
pub(crate) fn n976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 975);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 2278);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3857);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5341);
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

/// Words ending -χευω.
pub(crate) fn n977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 976);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 2279);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3858);
					match s {
						[] => true,
						['λ', s @ ..] => {
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
		['ι', s @ ..] => {
			super::push(out, 2280);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3859);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5343);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6551);
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

/// The word επιδιαλυω.
pub(crate) fn n9288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7837);
	s.is_empty()
}

/// The word αποδιαλυω.
pub(crate) fn n9289(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word προδιαλυω.
pub(crate) fn n9290(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word προσδιαλυω.
pub(crate) fn n9291(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ανταναλυω.
pub(crate) fn n9292(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επικαταλυω.
pub(crate) fn n9293(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word προκαταλυω.
pub(crate) fn n9294(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ιλυω.
pub(crate) fn n986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 984);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2290);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2291);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3865);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5354);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6564);
									match s {
										[] => true,
										['π', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2292);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3866);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5355);
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

/// Words ending -λλυω.
pub(crate) fn n988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 986);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 2295);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3867);
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

/// Words ending -ολυω.
pub(crate) fn n989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 987);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 2296);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3868);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5356);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5357);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6565);
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
						['ο', s @ ..] => {
							super::push(out, 5358);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6566);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7377);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7838);
													s.is_empty()
												}
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
							super::push(out, 5359);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6567);
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
		['ρ', s @ ..] => {
			super::push(out, 2297);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3869);
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

/// Words ending -ωλυω.
pub(crate) fn n991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 989);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2298);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3870);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5360);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6568);
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

/// The word προσκρουω.
pub(crate) fn n9296(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αναστρεφω.
pub(crate) fn n9301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7839);
	s.is_empty()
}

/// Words ending -ναναστρεφω.
pub(crate) fn n9302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7840);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 8075);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 8182);
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

/// Words ending -καταστρεφω.
pub(crate) fn n9303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7841);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επιστρεφω.
pub(crate) fn n9304(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αποστρεφω.
pub(crate) fn n9305(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word υποστρεφω.
pub(crate) fn n9306(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -προκαταρχω.
pub(crate) fn n9311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7843);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αντιπασχω.
pub(crate) fn n9312(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word μηχαναομαι.
pub(crate) fn n9317(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -πιτεχναομαι.
pub(crate) fn n9318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7845);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 8076);
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

/// Words ending -ταχραομαι.
pub(crate) fn n7462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6637);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 7407);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 7846);
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

/// Words ending -ιχραομαι.
pub(crate) fn n5944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5459);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 6638);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 7408);
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

/// Words ending -σαομαι.
pub(crate) fn n2466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2414);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3992);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5460);
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
		['σ', s @ ..] => {
			super::push(out, 3993);
			match s {
				[] => true,
				['η', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -κταομαι.
pub(crate) fn n4223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3994);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χαομαι.
pub(crate) fn n2468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2416);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 3995);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5463);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6639);
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

/// Words ending -βομαι.
pub(crate) fn n1086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1082);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2417);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3996);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3997);
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
		['ι', s @ ..] => {
			super::push(out, 2418);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3998);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5464);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6640);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7409);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7847);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9715(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -γομαι.
pub(crate) fn n1087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1083);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 2419);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3999);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5465);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6641);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 7410);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7848);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9716(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2420);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4000);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5466);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6642);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7411);
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
		['υ', s @ ..] => {
			super::push(out, 2421);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4001);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5467);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6643);
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

/// Words ending -δομαι.
pub(crate) fn n1088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1084);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 2422);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4002);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2423);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5470);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 5471);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6644);
									s.is_empty()
								}
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
			super::push(out, 2424);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5472);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 5473);
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
		['ν', s @ ..] => {
			super::push(out, 2425);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4007);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2426);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4008);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5475);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6645);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7412);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7849);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9717(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εομαι.
pub(crate) fn n1089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1085);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 2427);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4009);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5476);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6646);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7413);
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
		['γ', s @ ..] => {
			super::push(out, 2428);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 4010);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5477);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6647);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7414);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 5478);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6648);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 7415);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7850);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9718(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5479);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6649);
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
				['ο', s @ ..] => {
					super::push(out, 4011);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5480);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6650);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7416);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7851);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9719(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2429);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4012);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5481);
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
		['θ', s @ ..] => {
			super::push(out, 2430);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 4013);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5482);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6651);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7417);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7852);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9720(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2431);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4014);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5483);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6652);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7418);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7853);
													match s {
														[] => true,
														['χ', s @ ..] => super::t4::n9721(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2432);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4015);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 4016);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5484);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6653);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7419);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7854);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9722(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4017);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5485);
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
				['ω', s @ ..] => {
					super::push(out, 4018);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5486);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6654);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7420);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7855);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9723(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => super::t4::n2485(s, out),
		['ν', s @ ..] => super::t4::n2486(s, out),
		['ο', s @ ..] => {
			super::push(out, 2435);
			match s {
				[] => true,
				['ν', s @ ..] => super::t4::n4256(s, out),
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2436);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => super::t4::n2489(s, out),
		['χ', s @ ..] => super::t4::n2490(s, out),
		_ => false,
	}
}

/// Words ending -ζομαι.
pub(crate) fn n1090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1086);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2439);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4031);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 4032);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5500);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6669);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7431);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 7432);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7861);
													match s {
														[] => true,
														['ξ', '\u{1}', s @ ..] => super::t4::n9729(s, out),
														['σ', s @ ..] => super::t4::n9730(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ξ', s @ ..] => {
											super::push(out, 7433);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7862);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9731(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 7434);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7863);
													match s {
														[] => true,
														['κ', s @ ..] => super::t4::n9732(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4033);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5501);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6670);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 5502);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6671);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7435);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7864);
													match s {
														[] => true,
														['ο', s @ ..] => super::t4::n9733(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6672);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5504);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5505);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6673);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7436);
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
				['π', s @ ..] => {
					super::push(out, 4035);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5506);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 5507);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6675);
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
				['υ', s @ ..] => {
					super::push(out, 4036);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5508);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 6676);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7437);
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
				['χ', s @ ..] => {
					super::push(out, 4037);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5509);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5510);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6677);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7438);
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
						['ο', s @ ..] => {
							super::push(out, 5511);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6678);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7439);
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
		['ε', s @ ..] => {
			super::push(out, 2440);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4038);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5512);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t4::n2493(s, out),
		['ι', s @ ..] => super::t4::n2494(s, out),
		['υ', s @ ..] => super::t4::n2495(s, out),
		_ => false,
	}
}

/// Words ending -θομαι.
pub(crate) fn n1091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1087);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2444);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4049);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5525);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6690);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7452);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7878);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9746(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2445);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4050);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5526);
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
		['χ', s @ ..] => {
			super::push(out, 2446);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4051);
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

/// Words ending -ιομαι.
pub(crate) fn n1092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1088);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2447);
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

/// Words ending -κομαι.
pub(crate) fn n1093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1089);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 2448);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4052);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5527);
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
		['σ', s @ ..] => {
			super::push(out, 2449);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4053);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5528);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6691);
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
				['η', s @ ..] => {
					super::push(out, 4054);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5529);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6692);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7453);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7879);
													match s {
														[] => true,
														['ι', s @ ..] => super::t4::n9747(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4055);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5530);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6693);
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

/// Words ending -λομαι.
pub(crate) fn n1094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1090);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2450);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4057);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5531);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6694);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7454);
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
		['ι', s @ ..] => {
			super::push(out, 2451);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4058);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6695);
									s.is_empty()
								}
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
			super::push(out, 2452);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5533);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 5534);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6696);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7455);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 5535);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6697);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7456);
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
						['φ', s @ ..] => {
							super::push(out, 5536);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6698);
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
		['υ', s @ ..] => {
			super::push(out, 2453);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4060);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5537);
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

/// Words ending -μομαι.
pub(crate) fn n1095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1091);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2454);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4061);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5538);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6699);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7457);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7880);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νομαι.
pub(crate) fn n1096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1092);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2455);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4062);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5539);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6700);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7458);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7881);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9748(s, out),
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
									super::push(out, 6701);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7459);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 7882);
													match s {
														[] => true,
														['ι', s @ ..] => super::t4::n9749(s, out),
														_ => false,
													}
												}
												['ο', s @ ..] => {
													super::push(out, 7883);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t4::n9750(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5540);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6702);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7460);
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
						['χ', s @ ..] => {
							super::push(out, 5541);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6703);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7461);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7884);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9751(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2456);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4063);
					match s {
						[] => true,
						['γ', s @ ..] => super::t4::n6033(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2457);
			match s {
				[] => true,
				['π', s @ ..] => super::t4::n4295(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t4::n2510(s, out),
		_ => false,
	}
}

/// Words ending -οομαι.
pub(crate) fn n1097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1093);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2459);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4069);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5551);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6718);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7478);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7902);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 7903);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9768(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4070);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5552);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6719);
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
				['σ', s @ ..] => {
					super::push(out, 4071);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5553);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6720);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7479);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7904);
													match s {
														[] => true,
														['δ', s @ ..] => super::t4::n9769(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4072);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5554);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6721);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7480);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7905);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9770(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2460);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4073);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5555);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6722);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7481);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7906);
													match s {
														[] => true,
														['τ', s @ ..] => super::t4::n9771(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2461);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 4074);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5556);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6723);
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
		['ρ', s @ ..] => {
			super::push(out, 2462);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4075);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5557);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6724);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7482);
											match s {
												[] => true,
												['β', s @ ..] => {
													super::push(out, 7907);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9772(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4076);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5558);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 6725);
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
				['τ', s @ ..] => {
					super::push(out, 4077);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5559);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6726);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7483);
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
				['υ', s @ ..] => {
					super::push(out, 4078);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5560);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6727);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7484);
											match s {
												[] => true,
												['π', s @ ..] => super::t4::n9400(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t4::n2515(s, out),
		_ => false,
	}
}

/// Words ending -πομαι.
pub(crate) fn n1098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1094);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4080);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5562);
							match s {
								[] => true,
								['ξ', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 6729);
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

/// Words ending -ρομαι.
pub(crate) fn n1099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1095);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2465);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4081);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 4082);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5563);
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
				['π', s @ ..] => {
					super::push(out, 4083);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5564);
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
				['τ', s @ ..] => {
					super::push(out, 4084);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2466);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4085);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5566);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6730);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7486);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7910);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9774(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4086);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5567);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 5568);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6731);
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
		['υ', s @ ..] => {
			super::push(out, 2467);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4087);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5569);
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
				['ν', s @ ..] => {
					super::push(out, 4088);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5570);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6732);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 4089);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5571);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6733);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7487);
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
		_ => false,
	}
}

/// Words ending -σομαι.
pub(crate) fn n1100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1096);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 2468);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4090);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5572);
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
		['σ', s @ ..] => {
			super::push(out, 2469);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4091);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5573);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6734);
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
						['τ', s @ ..] => {
							super::push(out, 5574);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6735);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7488);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7911);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9775(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4092);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5575);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6736);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 6737);
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
						['ν', s @ ..] => {
							super::push(out, 5576);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4093);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5577);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6739);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7489);
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
								['ο', s @ ..] => {
									super::push(out, 6740);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7490);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 5578);
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

/// Words ending -τομαι.
pub(crate) fn n1101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1097);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2470);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5579);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5580);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6741);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7491);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7912);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9776(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2471);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4095);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5581);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6742);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 7492);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7913);
													match s {
														[] => true,
														['δ', s @ ..] => super::t4::n9777(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 7493);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7914);
													match s {
														[] => true,
														['ε', s @ ..] => super::t4::n9778(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5582);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6743);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υομαι.
pub(crate) fn n1102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1098);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2472);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4096);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 5583);
							match s {
								[] => true,
								['μ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
									super::push(out, 6744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4097);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5584);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6745);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7494);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7915);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9779(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4098);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5585);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6746);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7495);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7916);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9780(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4099);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5586);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6747);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7496);
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
				['ν', s @ ..] => {
					super::push(out, 4100);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 5587);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6748);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7497);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7917);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9781(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4101);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5588);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6749);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7498);
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
				['σ', s @ ..] => {
					super::push(out, 4102);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5589);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6750);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7499);
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
						['κ', s @ ..] => {
							super::push(out, 5590);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6751);
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
				['τ', s @ ..] => {
					super::push(out, 4103);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5591);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6752);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7500);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 7918);
													match s {
														[] => true,
														['ε', s @ ..] => super::t4::n9782(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5592);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6753);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7501);
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
						['τ', s @ ..] => {
							super::push(out, 5593);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 4104);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5594);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6755);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7502);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7919);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9783(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t4::n2525(s, out),
		_ => false,
	}
}

/// Words ending -φομαι.
pub(crate) fn n1103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1099);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 2474);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4106);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5596);
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

/// Words ending -χομαι.
pub(crate) fn n1104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1100);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2475);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 5597);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6756);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7503);
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
						['ο', s @ ..] => {
							super::push(out, 5598);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6757);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7504);
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
		['ε', s @ ..] => {
			super::push(out, 2476);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5599);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5600);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6758);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7505);
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
						['ο', s @ ..] => {
							super::push(out, 5601);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6759);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7506);
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
						['σ', s @ ..] => {
							super::push(out, 5602);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6760);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7507);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7920);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9784(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4109);
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
		['ι', s @ ..] => {
			super::push(out, 2477);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5603);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5604);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6761);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 6762);
									s.is_empty()
								}
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
			super::push(out, 2478);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4111);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5605);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6763);
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
				['ε', s @ ..] => super::t4::n4344(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2479);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5615);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5616);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6776);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 6777);
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
						['σ', s @ ..] => {
							super::push(out, 5617);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6778);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7522);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7929);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9790(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωομαι.
pub(crate) fn n1105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1101);
	match s {
		[] => true,
		['ῥ', s @ ..] => {
			super::push(out, 2480);
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

/// The word προσμειγνυμι.
pub(crate) fn n9973(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -εγκαταμιγνυμι.
pub(crate) fn n9974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8205);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -παραδεικνυμι.
pub(crate) fn n9804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8132);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επιδεικνυμι.
pub(crate) fn n9805(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αποδεικνυμι.
pub(crate) fn n9806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8133);
	s.is_empty()
}

/// The word υποδεικνυμι.
pub(crate) fn n9807(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ιασκεδαννυμι.
pub(crate) fn n9808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8134);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 8206);
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

/// Words ending -πικρεμαννυμι.
pub(crate) fn n9809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8135);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 8207);
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

/// The word αποσβεννυμι.
pub(crate) fn n9810(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αναρρωννυμι.
pub(crate) fn n9811(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word γενεαλογεω.
pub(crate) fn n9487(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word βατταλογεω.
pub(crate) fn n9488(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word εξομολογεω.
pub(crate) fn n9489(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word τεχνολογεω.
pub(crate) fn n9490(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -τερατολογεω.
pub(crate) fn n9491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7962);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word βαττολογεω.
pub(crate) fn n9492(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ιχθυολογεω.
pub(crate) fn n9493(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word κουφολογεω.
pub(crate) fn n9494(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -υλογεω.
pub(crate) fn n2864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2766);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4380);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5818);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word μογεω.
pub(crate) fn n2865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2767);
	s.is_empty()
}

/// The word δημιουργεω.
pub(crate) fn n9495(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -υγκακουργεω.
pub(crate) fn n9496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7963);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 8139);
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

/// The word χειρουργεω.
pub(crate) fn n9497(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αισχρουργεω.
pub(crate) fn n9498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7964);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τερατουργεω.
pub(crate) fn n9499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7965);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εκλητουργεω.
pub(crate) fn n9500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7966);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αρρητουργεω.
pub(crate) fn n9501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7967);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ειτουργεω.
pub(crate) fn n7834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6940);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 7626);
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

/// The word ετεροζυγεω.
pub(crate) fn n9503(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word νεκραγωγεω.
pub(crate) fn n9504(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ακριτομυθεω.
pub(crate) fn n9823(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επικατοικεω.
pub(crate) fn n9829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8143);
	s.is_empty()
}

/// The word τερατοτοκεω.
pub(crate) fn n9830(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -εκπολιορκεω.
pub(crate) fn n9519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7977);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word λογχοβολεω.
pub(crate) fn n9521(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αντεπιπλεω.
pub(crate) fn n9522(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -διοικοδομεω.
pub(crate) fn n9527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7981);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ανοικοδομεω.
pub(crate) fn n9528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7982);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υνοικοδομεω.
pub(crate) fn n9529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7983);
	match s {
		[] => true,
		['ξ', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 8146);
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

/// Words ending -ατοικοδομεω.
pub(crate) fn n9530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7984);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 8147);
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

/// The word αγορανομεω.
pub(crate) fn n9531(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ατοικονομεω.
pub(crate) fn n9532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7985);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 8148);
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

/// The word κληρονομεω.
pub(crate) fn n9533(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -δρομεω.
pub(crate) fn n2987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2878);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 4480);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 5901);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6993);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7661);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7986);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 8149);
													match s {
														[] => true,
														['λ', s @ ..] => super::t4::n9989(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 5902);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6994);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7662);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7987);
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
		_ => false,
	}
}

/// Words ending -τομεω.
pub(crate) fn n1499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1481);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2880);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4482);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5903);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6995);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7663);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7988);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9843(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2881);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4483);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5904);
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

/// The word εναδημονεω.
pub(crate) fn n9538(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ροκαταπονεω.
pub(crate) fn n9539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7990);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 8150);
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

/// The word καταφρονεω.
pub(crate) fn n9540(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word περιφρονεω.
pub(crate) fn n9541(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -οκτονεω.
pub(crate) fn n4802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4509);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 5925);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 7005);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 7669);
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
		['ν', s @ ..] => {
			super::push(out, 5926);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 7006);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 7670);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7991);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 7007);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7671);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 7992);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 8151);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 8211);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n10047(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5927);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 7008);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7672);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7993);
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
		['φ', s @ ..] => {
			super::push(out, 5928);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 7009);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 7673);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7994);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 8152);
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
		_ => false,
	}
}

/// Words ending -φονεω.
pub(crate) fn n1526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1508);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2915);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4510);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5929);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7010);
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

/// Words ending -χονεω.
pub(crate) fn n1527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1509);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 2916);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4511);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5930);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7011);
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

/// The word λαβρηγορεω.
pub(crate) fn n9552(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word μακρηγορεω.
pub(crate) fn n9553(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word οικοφθορεω.
pub(crate) fn n9554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7995);
	s.is_empty()
}

/// Words ending -τοικοφθορεω.
pub(crate) fn n9555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7996);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 8153);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 8212);
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

/// Words ending -παδηφορεω.
pub(crate) fn n7971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7034);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 7681);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 7997);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 8154);
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

/// Words ending -οφορεω.
pub(crate) fn n3088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2975);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 4566);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5971);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7035);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 7682);
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
		['ρ', s @ ..] => {
			super::push(out, 4567);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 5972);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 7036);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 7683);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7998);
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
						['λ', s @ ..] => {
							super::push(out, 7037);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7684);
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
				['τ', s @ ..] => {
					super::push(out, 5973);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 7038);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 7685);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7999);
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
		_ => false,
	}
}

/// The word προτιμωρεω.
pub(crate) fn n9561(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ταλαιπωρεω.
pub(crate) fn n9562(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επαναχωρεω.
pub(crate) fn n9563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8000);
	s.is_empty()
}

/// The word υπαναχωρεω.
pub(crate) fn n9564(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word στενοχωρεω.
pub(crate) fn n9565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8001);
	s.is_empty()
}

/// The word πλεονεκτεω.
pub(crate) fn n9567(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αναισχυντεω.
pub(crate) fn n9568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8002);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ικοδεσποτεω.
pub(crate) fn n9569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8003);
	match s {
		[] => true,
		['ο', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -παιδεραστεω.
pub(crate) fn n9570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8004);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 8155);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 8213);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 8227);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 8237);
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

/// The word ευχαριστεω.
pub(crate) fn n9571(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ανηκουστεω.
pub(crate) fn n9572(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -τρωγλοδυτεω.
pub(crate) fn n9573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8005);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ορυζοτροφεω.
pub(crate) fn n9581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8013);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κουροτροφεω.
pub(crate) fn n9582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8014);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word λαμπαδαρχεω.
pub(crate) fn n9869(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ανηγυριαρχεω.
pub(crate) fn n9870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8160);
	match s {
		[] => true,
		['π', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λαμπαδουχεω.
pub(crate) fn n9587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8019);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ολιγοψυχεω.
pub(crate) fn n9588(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αποσχεδιαζω.
pub(crate) fn n9592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8021);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λειστηριαζω.
pub(crate) fn n9593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8022);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 8161);
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

/// The word παρουσιαζω.
pub(crate) fn n9594(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αντιαζω.
pub(crate) fn n6666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6059);
	s.is_empty()
}

/// Words ending -χιαζω.
pub(crate) fn n1658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1636);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3078);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 3079);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4666);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6060);
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

/// The word καταναγκαζω.
pub(crate) fn n9875(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επιχειμαζω.
pub(crate) fn n9597(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -προετοιμαζω.
pub(crate) fn n9598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8024);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word διασκευαζω.
pub(crate) fn n9599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8025);
	s.is_empty()
}

/// The word ανασκευαζω.
pub(crate) fn n9600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8026);
	s.is_empty()
}

/// Words ending -παρασκευαζω.
pub(crate) fn n9601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8027);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 8162);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 8215);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 8228);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 8238);
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
		['ο', s @ ..] => {
			super::push(out, 8163);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 8216);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 8229);
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

/// Words ending -κατασκευαζω.
pub(crate) fn n9602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8028);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 8164);
			match s {
				[] => true,
				['υ', s @ ..] => {
					match s {
						[] => true,
						['ξ', '\u{1}', s @ ..] => {
							super::push(out, 8230);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word επισκευαζω.
pub(crate) fn n9603(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ανδραποδιζω.
pub(crate) fn n9882(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word εναττικιζω.
pub(crate) fn n9605(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ξαττικιζω.
pub(crate) fn n8111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7131);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 7733);
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

/// Words ending -ραττικιζω.
pub(crate) fn n8112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7132);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 7734);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 8030);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 8165);
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

/// Words ending -ρκιζω.
pub(crate) fn n1733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1710);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 3174);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 4746);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6107);
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

/// Words ending -υκιζω.
pub(crate) fn n1734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1711);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 3175);
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

/// The word σκανδαλιζω.
pub(crate) fn n9608(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αποκεφαλιζω.
pub(crate) fn n9609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8031);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ευαγγελιζω.
pub(crate) fn n9610(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word μετακομιζω.
pub(crate) fn n9611(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word προσκομιζω.
pub(crate) fn n9612(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word συγχρονιζω.
pub(crate) fn n9613(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word απαγχονιζω.
pub(crate) fn n9614(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ωνιζω.
pub(crate) fn n1755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1732);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 3211);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4780);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6135);
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

/// Words ending -χαρακτηριζω.
pub(crate) fn n9615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8032);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word υποστηριζω.
pub(crate) fn n9616(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -μεταχειριζω.
pub(crate) fn n9617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8033);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ποριζω.
pub(crate) fn n3355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3231);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 4800);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 6152);
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

/// Words ending -φοριζω.
pub(crate) fn n3356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3232);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4801);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6153);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word τριζω.
pub(crate) fn n3357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3233);
	s.is_empty()
}

/// Words ending -υριζω.
pub(crate) fn n1770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1747);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 3234);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 3235);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 3236);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4803);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6154);
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
		['σ', s @ ..] => {
			super::push(out, 3237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4804);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φριζω.
pub(crate) fn n1771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1748);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3238);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4805);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωριζω.
pub(crate) fn n1772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1749);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 3239);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 6155);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 7154);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7746);
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
		['χ', s @ ..] => {
			super::push(out, 3240);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 4807);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6156);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7155);
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

/// The word δειγματιζω.
pub(crate) fn n9619(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αδειγματιζω.
pub(crate) fn n9620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8034);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 8166);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 8217);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 8231);
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

/// The word στιγματιζω.
pub(crate) fn n9621(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -γεννηματιζω.
pub(crate) fn n9622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8035);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τιλημματιζω.
pub(crate) fn n9623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8036);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 8167);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 8218);
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

/// Words ending -ητιζω.
pub(crate) fn n1776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1753);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 3246);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 6166);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7165);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7751);
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
		_ => false,
	}
}

/// Words ending -αιτιζω.
pub(crate) fn n3371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3247);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κτιζω.
pub(crate) fn n1778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1755);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3248);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3249);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4817);
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

/// Words ending -ντιζω.
pub(crate) fn n1779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1756);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3250);
			match s {
				[] => true,
				['ῥ', s @ ..] => {
					super::push(out, 4818);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3251);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4819);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6168);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4820);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 6169);
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

/// Words ending -οτιζω.
pub(crate) fn n1780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1757);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3252);
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

/// Words ending -πτιζω.
pub(crate) fn n1781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1758);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3253);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4821);
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

/// Words ending -ρτιζω.
pub(crate) fn n1782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1759);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 3254);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 4822);
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

/// Words ending -στιζω.
pub(crate) fn n1783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1760);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3255);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3256);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4823);
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

/// Words ending -ωτιζω.
pub(crate) fn n1784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1761);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 3257);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4824);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6170);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 7167);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7752);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 8037);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9890(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ανακουφιζω.
pub(crate) fn n9626(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -περιτειχιζω.
pub(crate) fn n9627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8038);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word συντειχιζω.
pub(crate) fn n9628(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αποτειχιζω.
pub(crate) fn n9629(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αναμιμνησκω.
pub(crate) fn n9894(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -διαγιγνωσκω.
pub(crate) fn n9635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8042);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 8168);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 8219);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 8232);
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

/// Words ending -αναγιγνωσκω.
pub(crate) fn n9636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8043);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αταγιγνωσκω.
pub(crate) fn n9637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8044);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 8170);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 8220);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 8233);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 8239);
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

/// Words ending -εταγιγνωσκω.
pub(crate) fn n9638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8045);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 8171);
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

/// Words ending -επιγιγνωσκω.
pub(crate) fn n9639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8046);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επιγινωσκω.
pub(crate) fn n9640(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word παραγγελλω.
pub(crate) fn n9641(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word εισαναβαινω.
pub(crate) fn n9912(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επιλευκαινω.
pub(crate) fn n9913(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word κατακτεινω.
pub(crate) fn n9665(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word καταπλησσω.
pub(crate) fn n9680(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word καταπτησσω.
pub(crate) fn n9681(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ανακαλυπτω.
pub(crate) fn n9683(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αμφικαλυπτω.
pub(crate) fn n9684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8063);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αποκαλυπτω.
pub(crate) fn n9685(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word παρακρυπτω.
pub(crate) fn n9686(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word κατακρυπτω.
pub(crate) fn n9687(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ικρυπτω.
pub(crate) fn n5684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5249);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 6477);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 7329);
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
		['ρ', s @ ..] => {
			super::push(out, 6478);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 7330);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 7816);
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
		['φ', s @ ..] => {
			super::push(out, 6479);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 7331);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7817);
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

/// Words ending -εκκρυπτω.
pub(crate) fn n7260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6480);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οκρυπτω.
pub(crate) fn n5686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5251);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 6481);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 7332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 7333);
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
		['ρ', s @ ..] => {
			super::push(out, 6482);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 7334);
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

/// Words ending -τυπτω.
pub(crate) fn n2233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2192);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3760);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αντιστρατοπεδευω.
pub(crate) fn n10078(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επιβουλευω.
pub(crate) fn n9692(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -οεπιβουλευω.
pub(crate) fn n9693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8065);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 8176);
			match s {
				[] => true,
				['π', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τεπιβουλευω.
pub(crate) fn n9694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8066);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 8177);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ξυμβουλευω.
pub(crate) fn n9695(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word συμβουλευω.
pub(crate) fn n9696(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -βυσσοδομευω.
pub(crate) fn n9697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8067);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιαμνημονευω.
pub(crate) fn n9698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8068);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 8178);
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

/// Words ending -ινδυνευω.
pub(crate) fn n7312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6521);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 7356);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 7828);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 8069);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 8179);
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
				['γ', s @ ..] => {
					super::push(out, 7829);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 8070);
							match s {
								[] => true,
								['ξ', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 8180);
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
				['ο', s @ ..] => {
					super::push(out, 7830);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 8071);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 8181);
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

/// The word ιχνευω.
pub(crate) fn n5752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5307);
	s.is_empty()
}

/// The word προαγορευω.
pub(crate) fn n9702(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -προσαγορευω.
pub(crate) fn n9703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8072);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -επιστρατευω.
pub(crate) fn n9704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8073);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word εκστρατευω.
pub(crate) fn n9705(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ξυστρατευω.
pub(crate) fn n9706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8074);
	s.is_empty()
}

/// The word συστρατευω.
pub(crate) fn n9707(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -κρυπτευω.
pub(crate) fn n7339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6542);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -στευω.
pub(crate) fn n2316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2275);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 3853);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6543);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 6544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7367);
									s.is_empty()
								}
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
			super::push(out, 3854);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5335);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5336);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6546);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 6547);
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
				['ρ', s @ ..] => {
					super::push(out, 5337);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6548);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 5338);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6549);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 7369);
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

/// Words ending -υτευω.
pub(crate) fn n2317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2276);
	match s {
		[] => true,
		['φ', s @ ..] => {
			super::push(out, 3855);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 5339);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6550);
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

/// Words ending -ωτευω.
pub(crate) fn n2318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2277);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 3856);
			match s {
				[] => true,
				['α', s @ ..] => {
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
		_ => false,
	}
}

/// The word απαμειβομαι.
pub(crate) fn n9715(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αποφθεγγομαι.
pub(crate) fn n9716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8077);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αποπερδομαι.
pub(crate) fn n9717(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επεξηγεομαι.
pub(crate) fn n9718(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word απολογεομαι.
pub(crate) fn n9719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8078);
	s.is_empty()
}

/// The word προμηθεομαι.
pub(crate) fn n9720(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -χανοποιεομαι.
pub(crate) fn n9721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8079);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 8183);
			match s {
				[] => true,
				['μ', 'α', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word επιμελεομαι.
pub(crate) fn n9722(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επιπωλεομαι.
pub(crate) fn n9723(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -μεομαι.
pub(crate) fn n2485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2433);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 4019);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5487);
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
		['ο', s @ ..] => {
			super::push(out, 4020);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5488);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6655);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7421);
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
		['υ', s @ ..] => {
			super::push(out, 4021);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 5489);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6656);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7422);
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
						['ο', s @ ..] => {
							super::push(out, 6657);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 7423);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7856);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8080);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νεομαι.
pub(crate) fn n2486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2434);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 4022);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5490);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6658);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 6659);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7424);
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
						['φ', s @ ..] => {
							super::push(out, 6660);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7425);
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
		['ρ', s @ ..] => {
			super::push(out, 4023);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6661);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 6662);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7426);
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
		['χ', s @ ..] => {
			super::push(out, 4024);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5492);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6663);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7427);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7857);
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
		['ω', s @ ..] => {
			super::push(out, 4025);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5493);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νοεομαι.
pub(crate) fn n4256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4026);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5494);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 6664);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 7428);
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

/// Words ending -τεομαι.
pub(crate) fn n2489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2437);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4028);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 5495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5497);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6667);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 7429);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7858);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 8081);
													match s {
														[] => true,
														['δ', s @ ..] => super::t4::n9933(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 7859);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 8082);
													match s {
														[] => true,
														['ο', s @ ..] => super::t4::n9934(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4029);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5498);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6668);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7430);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7860);
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
		_ => false,
	}
}

/// Words ending -χεομαι.
pub(crate) fn n2490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2438);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 4030);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5499);
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

/// The word ξυνεργαζομαι.
pub(crate) fn n9729(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -συνεργαζομαι.
pub(crate) fn n9730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8083);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word εξεργαζομαι.
pub(crate) fn n9731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8084);
	s.is_empty()
}

/// Words ending -κατεργαζομαι.
pub(crate) fn n9732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8085);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ορινθιαζομαι.
pub(crate) fn n9733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8086);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 8186);
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

/// Words ending -ηζομαι.
pub(crate) fn n2493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2441);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 4039);
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

/// Words ending -ιζομαι.
pub(crate) fn n2494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2442);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 4040);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5513);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6680);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 7440);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7865);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 8087);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9938(s, out),
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
									super::push(out, 7441);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7866);
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
								['λ', s @ ..] => {
									super::push(out, 7442);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7867);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 8088);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9939(s, out),
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
									super::push(out, 7443);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7868);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 8089);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9940(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4041);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 5514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 4042);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5515);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6681);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 7444);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7869);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 8090);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9941(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4043);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 5516);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6682);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7445);
									s.is_empty()
								}
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
			super::push(out, 4044);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5517);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6683);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 7446);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7870);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 8091);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9942(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 5518);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6684);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7447);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7871);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 7872);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 8092);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9943(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 7873);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 8093);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9944(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4045);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5519);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6685);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7448);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7874);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 8094);
													match s {
														[] => true,
														['ε', s @ ..] => super::t4::n9945(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t4::n4276(s, out),
		['σ', s @ ..] => super::t4::n4277(s, out),
		_ => false,
	}
}

/// Words ending -υζομαι.
pub(crate) fn n2495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2443);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 4048);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5524);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word επιπειθομαι.
pub(crate) fn n9746(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ιμιμνησκομαι.
pub(crate) fn n9747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8098);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 8193);
			match s {
				[] => true,
				['ε', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πιλανθανομαι.
pub(crate) fn n9748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8099);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 8194);
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

/// Words ending -ιαπυνθανομαι.
pub(crate) fn n9749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8100);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 8195);
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

/// Words ending -ροπυνθανομαι.
pub(crate) fn n9750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8101);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 8196);
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

/// The word απεχθανομαι.
pub(crate) fn n9751(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -γιγνομαι.
pub(crate) fn n6033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5542);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6704);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 6705);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 7462);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 7885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 7463);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7886);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 8103);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['μ', 'υ', 'σ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7464);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7887);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 8104);
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
		['γ', s @ ..] => {
			super::push(out, 6706);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 7465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 7466);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 7889);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 8105);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 8197);
											match s {
												[] => true,
												['π', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6707);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 7467);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 7890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 7468);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 7891);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 8107);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['σ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6708);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 7469);
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
		['ο', s @ ..] => {
			super::push(out, 6709);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 7470);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7892);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 7893);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 7471);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 7894);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8109);
									s.is_empty()
								}
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
			super::push(out, 6710);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 7472);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 7895);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 8110);
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

/// Words ending -πενομαι.
pub(crate) fn n4295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4064);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 5543);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 6711);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 7473);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ινομαι.
pub(crate) fn n2510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2458);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4065);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 5544);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6712);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7474);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7897);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 8111);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9960(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 5545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 5546);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6714);
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
				['ρ', s @ ..] => {
					super::push(out, 5547);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 6715);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 7475);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7898);
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
				['τ', s @ ..] => {
					super::push(out, 5548);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6716);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7476);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7899);
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
		['ε', s @ ..] => {
			super::push(out, 4066);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5549);
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
		['ρ', s @ ..] => {
			super::push(out, 4067);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5550);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6717);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7477);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7900);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 8112);
													match s {
														[] => true,
														['ν', s @ ..] => super::t4::n9961(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 7901);
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
		['σ', s @ ..] => {
			super::push(out, 4068);
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

/// Words ending -ατοικειοομαι.
pub(crate) fn n9768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8113);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 8199);
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

/// Words ending -διπλασιοομαι.
pub(crate) fn n9769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8114);
	s.is_empty()
}

/// The word εναντιοομαι.
pub(crate) fn n9770(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -τραγγαλοομαι.
pub(crate) fn n9771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8115);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 8200);
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

/// The word βαρβαροομαι.
pub(crate) fn n9772(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -πονευροομαι.
pub(crate) fn n9400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7908);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 8116);
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

/// Words ending -υοομαι.
pub(crate) fn n2515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2463);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 4079);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 5561);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6728);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7485);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7909);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word τεκμαιρομαι.
pub(crate) fn n9774(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -παρατασσομαι.
pub(crate) fn n9775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8117);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 8201);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 8226);
					match s {
						[] => true,
						['ν', 'α', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word υπερπετομαι.
pub(crate) fn n9776(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -διασκεπτομαι.
pub(crate) fn n9777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8118);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8202);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -επισκεπτομαι.
pub(crate) fn n9778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8119);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αστειευομαι.
pub(crate) fn n9779(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αρακελευομαι.
pub(crate) fn n9780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8120);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 8203);
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

/// The word ειρωνευομαι.
pub(crate) fn n9781(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -εγκρατευομαι.
pub(crate) fn n9782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8121);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word λαλαχευομαι.
pub(crate) fn n9783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8122);
	s.is_empty()
}

/// Words ending -ρυομαι.
pub(crate) fn n2525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2473);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 4105);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5595);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word προσδεχομαι.
pub(crate) fn n9784(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ερχομαι.
pub(crate) fn n4344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4112);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5606);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 5607);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 6764);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 5608);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6765);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7509);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 7510);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7921);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8123);
											s.is_empty()
										}
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
					super::push(out, 6766);
					match s {
						[] => true,
						['ξ', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 7511);
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
		['ξ', s @ ..] => {
			super::push(out, 5609);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 6767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7512);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 7513);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7922);
									match s {
										[] => true,
										['ξ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 8124);
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
						['π', s @ ..] => {
							super::push(out, 7514);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7923);
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
		['ο', s @ ..] => {
			super::push(out, 5610);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 6768);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 7515);
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
		['π', s @ ..] => {
			super::push(out, 5611);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6769);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 6770);
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
		['ρ', s @ ..] => {
			super::push(out, 5612);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6771);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 7517);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7924);
									s.is_empty()
								}
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
			super::push(out, 5613);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 6772);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 7518);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 6773);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 7519);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7926);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8125);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5614);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6774);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 7520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7927);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 6775);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 7521);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word προσευχομαι.
pub(crate) fn n9790(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -λαμπαδοδρομεω.
pub(crate) fn n9989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8210);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word κεφαλοτομεω.
pub(crate) fn n9843(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word τυραννοκτονεω.
pub(crate) fn n10047(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αιχμαλωτιζω.
pub(crate) fn n9890(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -δημοκρατεομαι.
pub(crate) fn n9933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8184);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ουτοκρατεομαι.
pub(crate) fn n9934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8185);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 8223);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 8236);
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

/// The word διαλογιζομαι.
pub(crate) fn n9938(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word συλλογιζομαι.
pub(crate) fn n9939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8187);
	s.is_empty()
}

/// The word υπολογιζομαι.
pub(crate) fn n9940(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αντικαθιζομαι.
pub(crate) fn n9941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8188);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αναχρονιζομαι.
pub(crate) fn n9942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8189);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word διαγωνιζομαι.
pub(crate) fn n9943(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ανταγωνιζομαι.
pub(crate) fn n9944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8190);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -επιδορπιζομαι.
pub(crate) fn n9945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8191);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ριζομαι.
pub(crate) fn n4276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4046);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5520);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 6686);
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
		['η', s @ ..] => {
			super::push(out, 5521);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 6687);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 7449);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 7875);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 8095);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 8192);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n10029(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5522);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 6688);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 7450);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7876);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 8096);
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
		_ => false,
	}
}

/// Words ending -σιζομαι.
pub(crate) fn n4277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4047);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5523);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 6689);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 7451);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 7877);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 8097);
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
		_ => false,
	}
}

/// The word απαλθαινομαι.
pub(crate) fn n9960(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -νταποκρινομαι.
pub(crate) fn n9961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8198);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 8225);
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

/// Words ending -πλειστηριζομαι.
pub(crate) fn n10029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8224);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

