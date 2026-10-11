use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['α', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 9);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 63);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 307);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 1052);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 2376);
													match s {
														[] => true,
														['ξ', s @ ..] => super::t0::n4173(s, out),
														['σ', s @ ..] => super::t0::n4174(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 1053);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 2377);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n4175(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 10);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 64);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 308);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 1054);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 2378);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n4176(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 65);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 309);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1055);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 2379);
													match s {
														[] => true,
														['ι', s @ ..] => super::t0::n4177(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 66);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 310);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1056);
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
					super::push(out, 13);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 67);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 311);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1057);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 2380);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n4178(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 68);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 312);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1058);
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
		['ε', s @ ..] => super::t0::n2(s, out),
		['η', s @ ..] => super::t0::n3(s, out),
		['ι', s @ ..] => super::t0::n4(s, out),
		['ν', s @ ..] => super::t0::n5(s, out),
		['ο', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 35);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 115);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 386);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1185);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 2583);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n4483(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => super::t0::n7(s, out),
		['ω', s @ ..] => super::t0::n8(s, out),
		_ => false,
	}
}

/// Words ending -ξυνοιδα.
pub(crate) fn n4173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3949);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5422);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συνοιδα.
pub(crate) fn n4174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3950);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προοιδα.
pub(crate) fn n4175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3951);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ενημεθα.
pub(crate) fn n4176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3952);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 5423);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 6610);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιδειδια.
pub(crate) fn n4177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3953);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 5424);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 6611);
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

/// The word τεθηπα.
pub(crate) fn n4178(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ε.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 15);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 69);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 313);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1059);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2381);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 3954);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n5905(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 70);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 314);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1060);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 2382);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 71);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 315);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1061);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2383);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 3955);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n5906(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 72);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 316);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1062);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2384);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 3956);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n5907(s, out),
														_ => false,
													}
												}
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
							super::push(out, 317);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1063);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 2385);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3957);
													match s {
														[] => true,
														['γ', s @ ..] => super::t0::n5908(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 73);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 318);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1064);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2386);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 3958);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n5909(s, out),
														_ => false,
													}
												}
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
							super::push(out, 319);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1065);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 2387);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3959);
													match s {
														[] => true,
														['γ', s @ ..] => super::t0::n5910(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 74);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 320);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1066);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2388);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3960);
													match s {
														[] => true,
														['θ', s @ ..] => super::t0::n5911(s, out),
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
									super::push(out, 1067);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2389);
											match s {
												[] => true,
												['γ', s @ ..] => super::t0::n4186(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => super::t0::n18(s, out),
		_ => false,
	}
}

/// Words ending -η.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 19);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 76);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 322);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1069);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2391);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3962);
													match s {
														[] => true,
														['ο', s @ ..] => super::t0::n5913(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 20);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 77);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 323);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1070);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2392);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3963);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n5914(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 78);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 324);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1071);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2393);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 3964);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n5915(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 79);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 325);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1072);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2394);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3965);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 80);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 326);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1073);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 2395);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 81);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ι.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['α', s @ ..] => super::t0::n24(s, out),
		['ε', s @ ..] => {
			super::push(out, 25);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 86);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => super::t0::n26(s, out),
		['ο', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 91);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 350);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1141);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 2534);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4200);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n6204(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 92);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 351);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1142);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2535);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 4201);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 28);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 93);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 352);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1143);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2536);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 4202);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n6205(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 94);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 353);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1144);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2537);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 4203);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n6206(s, out),
														_ => false,
													}
												}
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
							super::push(out, 354);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1145);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 2538);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4204);
													match s {
														[] => true,
														['γ', s @ ..] => super::t0::n6207(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 95);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 355);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1146);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2539);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4205);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n6208(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 96);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 356);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 1147);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2540);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ν.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 30);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 97);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 357);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1148);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2541);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 4206);
													match s {
														[] => true,
														['θ', s @ ..] => super::t0::n6209(s, out),
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
									super::push(out, 1149);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2542);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 4207);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n6210(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 98);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 358);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1150);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 2543);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4208);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n6211(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 99);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 359);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1151);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2544);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 4209);
													match s {
														[] => true,
														['θ', s @ ..] => super::t0::n6212(s, out),
														_ => false,
													}
												}
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
							super::push(out, 360);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1152);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2545);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 4210);
													match s {
														[] => true,
														['ν', s @ ..] => super::t0::n6213(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2546);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 4211);
													match s {
														[] => true,
														['γ', s @ ..] => super::t0::n6214(s, out),
														_ => false,
													}
												}
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
							super::push(out, 361);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1153);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2547);
											match s {
												[] => true,
												['ν', s @ ..] => super::t0::n4446(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => super::t0::n1159(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => super::t0::n100(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n31(s, out),
		['ι', s @ ..] => super::t0::n32(s, out),
		['ο', s @ ..] => super::t0::n33(s, out),
		['ω', s @ ..] => super::t0::n34(s, out),
		_ => false,
	}
}

/// Words ending -εγενησο.
pub(crate) fn n4483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4247);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 5738);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σ.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 36);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 116);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 387);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1186);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 2584);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4248);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n6250(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 117);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 388);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1187);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2585);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 4249);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 118);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 389);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1188);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2586);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 4250);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n6251(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 119);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 390);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1189);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2587);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 4251);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n6252(s, out),
														_ => false,
													}
												}
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
							super::push(out, 391);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1190);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2588);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 4252);
													match s {
														[] => true,
														['θ', s @ ..] => super::t0::n6253(s, out),
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
									super::push(out, 1191);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2589);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 4253);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n6254(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 120);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 392);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1192);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 2590);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4254);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n6255(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 38);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 121);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 393);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1193);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2591);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 4255);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n6256(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ω.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['α', s @ ..] => super::t0::n39(s, out),
		['β', s @ ..] => {
			super::push(out, 40);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 143);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 486);
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
					super::push(out, 144);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 487);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1310);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 1311);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2690);
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
									super::push(out, 1312);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2691);
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
							super::push(out, 488);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 1313);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 2692);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 4326);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n6318(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2693);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 4327);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n6319(s, out),
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
									super::push(out, 1314);
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
							super::push(out, 489);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1315);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2694);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2695);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 4328);
													match s {
														[] => true,
														['δ', s @ ..] => super::t0::n6320(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2696);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 4329);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n6321(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2697);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 4330);
													match s {
														[] => true,
														['υ', s @ ..] => super::t0::n6322(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 145);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 490);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1316);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2698);
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
				['μ', s @ ..] => super::t0::n146(s, out),
				['ρ', s @ ..] => {
					super::push(out, 147);
					match s {
						[] => true,
						['ε', s @ ..] => super::t0::n492(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => super::t0::n41(s, out),
		['δ', s @ ..] => super::t0::n42(s, out),
		['ε', s @ ..] => super::t0::n43(s, out),
		['ζ', s @ ..] => super::t0::n44(s, out),
		['η', s @ ..] => super::t0::n45(s, out),
		['θ', s @ ..] => super::t0::n46(s, out),
		['ι', s @ ..] => super::t0::n47(s, out),
		['κ', s @ ..] => super::t0::n48(s, out),
		['λ', s @ ..] => super::t0::n49(s, out),
		['μ', s @ ..] => super::t0::n50(s, out),
		['ν', s @ ..] => super::t0::n51(s, out),
		['ξ', s @ ..] => super::t0::n52(s, out),
		['ο', s @ ..] => super::t0::n53(s, out),
		['π', s @ ..] => super::t0::n54(s, out),
		['ρ', s @ ..] => super::t0::n55(s, out),
		['σ', s @ ..] => super::t0::n56(s, out),
		['τ', s @ ..] => super::t0::n57(s, out),
		['υ', s @ ..] => super::t0::n58(s, out),
		['φ', s @ ..] => super::t0::n59(s, out),
		['χ', s @ ..] => super::t0::n60(s, out),
		['ψ', s @ ..] => super::t0::n61(s, out),
		['ω', s @ ..] => super::t0::n62(s, out),
		_ => false,
	}
}

/// Words ending -εγενησθε.
pub(crate) fn n5905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5425);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 6612);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εθνηκατε.
pub(crate) fn n5906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5426);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6613);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εθνηκετε.
pub(crate) fn n5907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5427);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γεγονετε.
pub(crate) fn n5908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5428);
	s.is_empty()
}

/// Words ending -εθνηκητε.
pub(crate) fn n5909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5429);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6615);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γεγονητε.
pub(crate) fn n5910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5430);
	s.is_empty()
}

/// Words ending -θνηκοιτε.
pub(crate) fn n5911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5431);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 6616);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 7399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -γονοιτε.
pub(crate) fn n4186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3961);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 5432);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 6617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χε.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 321);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1068);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2390);
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

/// Words ending -οστερηθη.
pub(crate) fn n5913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5433);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 6618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τερηθειη.
pub(crate) fn n5914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5434);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 6619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εθνηκοιη.
pub(crate) fn n5915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5435);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αι.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['μ', s @ ..] => super::t0::n82(s, out),
		['ν', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 333);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1103);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2485);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 4118);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 5622);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n7627(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 334);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1104);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2486);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 4119);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 5623);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n7628(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 84);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 335);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1105);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2487);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 4120);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 5624);
													match s {
														[] => true,
														['γ', s @ ..] => super::t0::n7629(s, out),
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
									super::push(out, 2488);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4121);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 5625);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 85);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 336);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1106);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2489);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 4122);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 5626);
													match s {
														[] => true,
														['γ', s @ ..] => super::t0::n7630(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 337);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1107);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2490);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4123);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 5627);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n7631(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μι.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['θ', s @ ..] => super::t0::n338(s, out),
				['ι', s @ ..] => {
					super::push(out, 339);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1109);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1110);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2492);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4131);
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
						['ν', s @ ..] => super::t0::n1115(s, out),
						['ξ', s @ ..] => {
							super::push(out, 1112);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2496);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t0::n4370(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => super::t0::n1117(s, out),
						['φ', s @ ..] => super::t0::n1118(s, out),
						_ => false,
					}
				}
				['λ', s @ ..] => super::t0::n340(s, out),
				['ν', s @ ..] => super::t0::n341(s, out),
				['ρ', s @ ..] => super::t0::n342(s, out),
				['τ', s @ ..] => super::t0::n343(s, out),
				['φ', s @ ..] => super::t0::n344(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n88(s, out),
		['υ', s @ ..] => super::t1::n89(s, out),
		['ω', s @ ..] => super::t1::n90(s, out),
		_ => false,
	}
}

/// Words ending -τεθνηκοι.
pub(crate) fn n6204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5698);
	s.is_empty()
}

/// Words ending -εθνηκασι.
pub(crate) fn n6205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5699);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6855);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εθνηκωσι.
pub(crate) fn n6206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5700);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6856);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γεγονωσι.
pub(crate) fn n6207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5701);
	s.is_empty()
}

/// Words ending -τερηθητι.
pub(crate) fn n6208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5702);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 6857);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 7577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θνηκοιεν.
pub(crate) fn n6209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5703);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 6858);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 7578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εγονοιεν.
pub(crate) fn n6210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5704);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 6859);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεθνηκεν.
pub(crate) fn n6211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5705);
	s.is_empty()
}

/// Words ending -θνηκαμεν.
pub(crate) fn n6212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5706);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 6860);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 7579);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νηκοιμεν.
pub(crate) fn n6213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5707);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 6861);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 7580);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 7954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -γονοιμεν.
pub(crate) fn n6214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5708);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 6862);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 7581);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νηκωμεν.
pub(crate) fn n4446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4212);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 5709);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 6863);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 7582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νωμεν.
pub(crate) fn n1159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1154);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2548);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4213);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5710);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νεν.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2549);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ην.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 363);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1156);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2550);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 4215);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 5711);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n7724(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 364);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1157);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2551);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 4216);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 5712);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n7725(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 102);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1158);
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
			super::push(out, 103);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 366);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1159);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2552);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 4217);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 5713);
													match s {
														[] => true,
														['θ', s @ ..] => super::t1::n7726(s, out),
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
									super::push(out, 2553);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 4218);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 5714);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n7727(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιν.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1160);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2554);
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
			super::push(out, 105);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 368);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1161);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2555);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 4219);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 5715);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n7728(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 369);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1162);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2556);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 4220);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 5716);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n7729(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1163);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2557);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 4221);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 5717);
													match s {
														[] => true,
														['γ', s @ ..] => super::t1::n7730(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ον.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 370);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1164);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2558);
									s.is_empty()
								}
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
			super::push(out, 107);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 371);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1165);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2559);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4222);
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
					super::push(out, 372);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1166);
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
			super::push(out, 108);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 373);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1167);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2560);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4223);
											match s {
												[] => true,
												['τ', 'ε', 'μ', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 374);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 375);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1169);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2561);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4224);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 5718);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n7731(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 109);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 376);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1170);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2562);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4225);
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
		['π', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 377);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 2563);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4226);
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
									super::push(out, 2564);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 4227);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 5719);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7732(s, out),
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
									super::push(out, 2565);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4228);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5720);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ε', s @ ..] => {
											super::push(out, 4229);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 4230);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5721);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2566);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4231);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 5722);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7733(s, out),
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
									super::push(out, 2567);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 4232);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 5723);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n7734(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 111);
			match s {
				[] => true,
				['ο', s @ ..] => super::t1::n378(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => super::t1::n112(s, out),
		_ => false,
	}
}

/// Words ending -ων.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 113);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 383);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1181);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2578);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4242);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 5733);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n7744(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 114);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 384);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1182);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2579);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 4243);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 5734);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n7745(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1183);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2580);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 4244);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 5735);
													match s {
														[] => true,
														['γ', s @ ..] => super::t1::n7746(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 385);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1184);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2581);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 4245);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 5736);
													match s {
														[] => true,
														['θ', s @ ..] => super::t1::n7747(s, out),
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
									super::push(out, 2582);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 4246);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 5737);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n7748(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τεθνηκασ.
pub(crate) fn n6250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5739);
	s.is_empty()
}

/// Words ending -στερηθησ.
pub(crate) fn n6251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5740);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 6888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ερηθειησ.
pub(crate) fn n6252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5741);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6889);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 7601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θνηκοιησ.
pub(crate) fn n6253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5742);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 6890);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 7602);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εγονοιησ.
pub(crate) fn n6254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5743);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 6891);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεθνηκησ.
pub(crate) fn n6255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5744);
	s.is_empty()
}

/// Words ending -εθνηκοισ.
pub(crate) fn n6256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5745);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 6892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αω.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 395);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1194);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2592);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 4256);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 5746);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7754(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 124);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 397);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1195);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2593);
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
					super::push(out, 398);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1196);
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
					super::push(out, 399);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1197);
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
				['υ', s @ ..] => {
					super::push(out, 400);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1198);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2594);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4257);
											s.is_empty()
										}
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
					super::push(out, 401);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1199);
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
		['δ', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 402);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1201);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2595);
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
				['ε', s @ ..] => {
					super::push(out, 403);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1202);
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
					super::push(out, 404);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1203);
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
					super::push(out, 405);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1204);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2596);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4258);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 4259);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 5747);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n7755(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1205);
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
				['υ', s @ ..] => {
					super::push(out, 406);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2597);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2598);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 4260);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 5748);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n7756(s, out),
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
									super::push(out, 2599);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4261);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 5749);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7757(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 407);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 408);
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
		['ζ', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 409);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1207);
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
		['ι', s @ ..] => super::t1::n128(s, out),
		['κ', s @ ..] => super::t1::n129(s, out),
		['λ', s @ ..] => super::t1::n130(s, out),
		['μ', s @ ..] => super::t1::n131(s, out),
		['ν', s @ ..] => super::t1::n132(s, out),
		['ο', s @ ..] => super::t1::n133(s, out),
		['π', s @ ..] => super::t1::n134(s, out),
		['ρ', s @ ..] => super::t1::n135(s, out),
		['σ', s @ ..] => super::t1::n136(s, out),
		['τ', s @ ..] => super::t1::n137(s, out),
		['υ', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['θ', s @ ..] => super::t1::n477(s, out),
				_ => false,
			}
		}
		['φ', s @ ..] => super::t1::n139(s, out),
		['χ', s @ ..] => super::t1::n140(s, out),
		['ψ', s @ ..] => super::t1::n141(s, out),
		['ω', s @ ..] => super::t1::n142(s, out),
		_ => false,
	}
}

/// Words ending -συνθλιβω.
pub(crate) fn n6318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5780);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αποθλιβω.
pub(crate) fn n6319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5781);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -διατριβω.
pub(crate) fn n6320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5782);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6909);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 6910);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 7606);
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

/// Words ending -συντριβω.
pub(crate) fn n6321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5783);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υποτριβω.
pub(crate) fn n6322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5784);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μβω.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1317);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2699);
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
				['ῥ', s @ ..] => {
					super::push(out, 1318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ερβω.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 492);
	match s {
		[] => true,
		['φ', s @ ..] => {
			super::push(out, 1319);
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

/// Words ending -γω.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 493);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 494);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1320);
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
					super::push(out, 495);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1321);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2701);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2702);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 4331);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 5785);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7792(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1323);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2704);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4332);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 2705);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4333);
											s.is_empty()
										}
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
					super::push(out, 496);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 497);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1325);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2707);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4334);
											s.is_empty()
										}
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
					super::push(out, 498);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2708);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1328);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2709);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 499);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1329);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2710);
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
					super::push(out, 500);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1330);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2711);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4335);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1331);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2712);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 4336);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5786);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 501);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1332);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2713);
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
			super::push(out, 149);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 502);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1333);
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
					super::push(out, 503);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1334);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2714);
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
		['ε', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['λ', s @ ..] => super::t1::n504(s, out),
				['ρ', s @ ..] => {
					super::push(out, 505);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1342);
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
				['τ', s @ ..] => super::t1::n506(s, out),
				['ψ', s @ ..] => super::t1::n507(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t1::n151(s, out),
		['ι', s @ ..] => super::t1::n152(s, out),
		['λ', s @ ..] => super::t1::n153(s, out),
		['ρ', s @ ..] => super::t1::n154(s, out),
		['σ', s @ ..] => super::t1::n155(s, out),
		['υ', s @ ..] => super::t1::n156(s, out),
		['ω', s @ ..] => super::t1::n157(s, out),
		_ => false,
	}
}

/// Words ending -δω.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1362);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2740);
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
							super::push(out, 1363);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2741);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4356);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 160);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 524);
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
			super::push(out, 161);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 525);
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
			super::push(out, 162);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1364);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 1365);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2742);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 4357);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 5799);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7806(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1366);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2743);
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
			super::push(out, 163);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1367);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2744);
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
							super::push(out, 1368);
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
					super::push(out, 529);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1369);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2745);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4358);
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
		['ρ', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1370);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 1371);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2746);
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
			super::push(out, 165);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1372);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1373);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2747);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4359);
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
							super::push(out, 1374);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2748);
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
						['ψ', s @ ..] => {
							super::push(out, 1375);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εω.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 1377);
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
					super::push(out, 534);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1378);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2750);
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
							super::push(out, 1379);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2751);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4360);
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
		['γ', s @ ..] => super::t1::n167(s, out),
		['δ', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 545);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1400);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2776);
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
							super::push(out, 1401);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2777);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4391);
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
						['μ', s @ ..] => {
							super::push(out, 1402);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2778);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4392);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1403);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2779);
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
							super::push(out, 1404);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2780);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4393);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5831);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 547);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1406);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2781);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2782);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 4394);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 5832);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7838(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 548);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1407);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 1408);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2783);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4395);
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
					super::push(out, 549);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1409);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1410);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2785);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4396);
											s.is_empty()
										}
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
					super::push(out, 550);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1411);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2786);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 4397);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5833);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 551);
					match s {
						[] => true,
						['γ', s @ ..] => super::t1::n1429(s, out),
						['ρ', s @ ..] => super::t1::n1430(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t1::n169(s, out),
		['ζ', s @ ..] => super::t1::n170(s, out),
		['θ', s @ ..] => super::t1::n171(s, out),
		['ι', s @ ..] => super::t1::n172(s, out),
		['κ', s @ ..] => super::t1::n173(s, out),
		['λ', s @ ..] => super::t1::n174(s, out),
		['μ', s @ ..] => super::t1::n175(s, out),
		['ν', s @ ..] => super::t1::n176(s, out),
		['ξ', s @ ..] => super::t1::n177(s, out),
		['ο', s @ ..] => super::t1::n178(s, out),
		['π', s @ ..] => super::t1::n179(s, out),
		['ρ', s @ ..] => super::t1::n180(s, out),
		['σ', s @ ..] => super::t1::n181(s, out),
		['τ', s @ ..] => super::t1::n182(s, out),
		['υ', s @ ..] => super::t1::n183(s, out),
		['φ', s @ ..] => super::t1::n184(s, out),
		['χ', s @ ..] => super::t1::n185(s, out),
		['ψ', s @ ..] => super::t1::n186(s, out),
		['ω', s @ ..] => super::t1::n187(s, out),
		['ῥ', s @ ..] => super::t1::n188(s, out),
		_ => false,
	}
}

/// Words ending -ζω.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 189);
			s.is_empty()
		}
		['α', s @ ..] => super::t1::n190(s, out),
		['ε', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1684);
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
					super::push(out, 680);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1685);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3141);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 4716);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6084);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n8086(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 681);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 682);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1687);
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
		['ι', s @ ..] => super::t1::n193(s, out),
		['ο', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1769);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3266);
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
					super::push(out, 704);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1770);
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
			super::push(out, 195);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1771);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3267);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 4830);
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
				['λ', s @ ..] => {
					super::push(out, 706);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1772);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 1773);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1774);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3268);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3269);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 4831);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6180);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n8179(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1775);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3270);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 4832);
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
					super::push(out, 707);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1776);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3271);
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
				['ρ', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1777);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3272);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4833);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 6181);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n8180(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 709);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1778);
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
			super::push(out, 196);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1779);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3273);
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
				['ρ', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['κ', s @ ..] => super::t1::n1803(s, out),
						_ => false,
					}
				}
				['σ', s @ ..] => super::t1::n712(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηω.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θω.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1784);
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
					super::push(out, 715);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1785);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3276);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4836);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 200);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1786);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1787);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3277);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4837);
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
							super::push(out, 1788);
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
					super::push(out, 717);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1789);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1790);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3279);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 4838);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 6182);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n8181(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1791);
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
					super::push(out, 719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 721);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1794);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3280);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 4839);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6183);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n8182(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 202);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 722);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1795);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3281);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 4840);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 6184);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n8183(s, out),
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
									super::push(out, 3282);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4841);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6185);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 203);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1796);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3283);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 4842);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6186);
													match s {
														[] => true,
														['γ', s @ ..] => super::t1::n8184(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 204);
			match s {
				[] => true,
				['ε', s @ ..] => super::t1::n724(s, out),
				['ν', s @ ..] => {
					super::push(out, 725);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1800);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3288);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t1::n5193(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => super::t1::n726(s, out),
				_ => false,
			}
		}
		['χ', s @ ..] => super::t1::n205(s, out),
		['ω', s @ ..] => super::t1::n206(s, out),
		_ => false,
	}
}

/// Words ending -ιω.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 729);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1804);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3291);
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
				['δ', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1806);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3292);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4845);
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
				['λ', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1807);
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
					super::push(out, 733);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1810);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3293);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4846);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
									super::push(out, 3294);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4847);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6190);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n8187(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1811);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 735);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1812);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3296);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 4848);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 6191);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n8188(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 736);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1813);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3297);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 737);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1814);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 741);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1817);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3299);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 4849);
											match s {
												[] => true,
												['φ', '\u{1}', s @ ..] => {
													super::push(out, 6192);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 742);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1818);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3300);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3301);
									match s {
										[] => true,
										['ρ', s @ ..] => super::t1::n5202(s, out),
										_ => false,
									}
								}
								['ο', s @ ..] => super::t1::n3437(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => super::t1::n743(s, out),
				['ξ', s @ ..] => super::t1::n744(s, out),
				['ρ', s @ ..] => super::t1::n745(s, out),
				['σ', s @ ..] => super::t1::n746(s, out),
				_ => false,
			}
		}
		['θ', s @ ..] => super::t1::n211(s, out),
		['κ', s @ ..] => super::t1::n212(s, out),
		['λ', s @ ..] => super::t1::n213(s, out),
		['ν', s @ ..] => super::t1::n214(s, out),
		['ρ', s @ ..] => super::t1::n215(s, out),
		['τ', s @ ..] => super::t1::n216(s, out),
		_ => false,
	}
}

/// Words ending -κω.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 756);
					match s {
						[] => true,
						['ῥ', s @ ..] => {
							super::push(out, 1836);
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
		['ε', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 757);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1837);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1838);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3313);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 4859);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6200);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n8195(s, out),
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
									super::push(out, 3314);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4860);
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
					super::push(out, 758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 759);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1840);
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
			super::push(out, 219);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1841);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3315);
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
					super::push(out, 762);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1842);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3316);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1843);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 1844);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3317);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 4861);
											s.is_empty()
										}
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
					super::push(out, 764);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1845);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3318);
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
					super::push(out, 765);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1846);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3319);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 4862);
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
					super::push(out, 766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1848);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3320);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4863);
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
					super::push(out, 767);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1849);
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
			super::push(out, 220);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 768);
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
			super::push(out, 221);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 769);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1851);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3321);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4864);
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
							super::push(out, 1852);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3322);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 3323);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4865);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => super::t1::n222(s, out),
		['υ', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['ρ', s @ ..] => super::t1::n778(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t1::n224(s, out),
		_ => false,
	}
}

/// Words ending -λω.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1876);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1877);
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
				['μ', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 781);
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
			super::push(out, 226);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1878);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3352);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4890);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 4891);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 6223);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n8220(s, out),
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
									super::push(out, 3353);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 4892);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 6224);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n8221(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t1::n227(s, out),
		_ => false,
	}
}

/// Words ending -μω.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1899);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1900);
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

/// Words ending -νω.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['μ', s @ ..] => super::t1::n1934(s, out),
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 792);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1902);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3387);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 4923);
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
				['δ', s @ ..] => {
					super::push(out, 793);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1903);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3388);
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
						['ν', s @ ..] => super::t1::n1937(s, out),
						_ => false,
					}
				}
				['ζ', s @ ..] => super::t1::n795(s, out),
				['θ', s @ ..] => super::t1::n796(s, out),
				['κ', s @ ..] => super::t1::n797(s, out),
				['ξ', s @ ..] => super::t1::n798(s, out),
				['π', s @ ..] => super::t1::n799(s, out),
				['τ', s @ ..] => super::t1::n800(s, out),
				['φ', s @ ..] => super::t1::n801(s, out),
				['χ', s @ ..] => super::t1::n802(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t1::n230(s, out),
		['ι', s @ ..] => super::t2::n231(s, out),
		['κ', s @ ..] => super::t2::n232(s, out),
		['μ', s @ ..] => super::t2::n233(s, out),
		['υ', s @ ..] => super::t2::n234(s, out),
		_ => false,
	}
}

/// Words ending -ξω.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 827);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1986);
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
			super::push(out, 236);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οω.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 830);
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
		['β', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 831);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1989);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3513);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5050);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6345);
													s.is_empty()
												}
												['π', s @ ..] => {
													super::push(out, 6346);
													match s {
														[] => true,
														['α', s @ ..] => super::t2::n8339(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 832);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1990);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3514);
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
					super::push(out, 833);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1991);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3515);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5051);
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
			super::push(out, 239);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 834);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1992);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3516);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5052);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 6347);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8340(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 835);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 1993);
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
		['δ', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 836);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1994);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3517);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5053);
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
							super::push(out, 1995);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3518);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 5054);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6348);
													match s {
														[] => true,
														['ῥ', s @ ..] => super::t2::n8341(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 837);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1996);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3519);
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
		['ε', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 838);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1997);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3520);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5055);
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
									super::push(out, 3521);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 5056);
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
			super::push(out, 242);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 840);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1999);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 841);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2000);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3523);
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
		['ι', s @ ..] => super::t2::n244(s, out),
		['κ', s @ ..] => super::t2::n245(s, out),
		['λ', s @ ..] => super::t2::n246(s, out),
		['μ', s @ ..] => super::t2::n247(s, out),
		['ν', s @ ..] => super::t2::n248(s, out),
		['ξ', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['ο', s @ ..] => super::t2::n878(s, out),
				_ => false,
			}
		}
		['π', s @ ..] => super::t2::n250(s, out),
		['ρ', s @ ..] => super::t2::n251(s, out),
		['σ', s @ ..] => super::t2::n252(s, out),
		['τ', s @ ..] => super::t2::n253(s, out),
		['φ', s @ ..] => super::t2::n254(s, out),
		['χ', s @ ..] => super::t2::n255(s, out),
		['ψ', s @ ..] => super::t2::n256(s, out),
		_ => false,
	}
}

/// Words ending -πω.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 898);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2081);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3597);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5104);
											match s {
												[] => true,
												['ε', '\u{1}', s @ ..] => {
													super::push(out, 6370);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 258);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 900);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3598);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2083);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3599);
									match s {
										[] => true,
										['π', s @ ..] => {
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
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 2084);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3600);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5106);
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
					super::push(out, 901);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2085);
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
					super::push(out, 902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 2086);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3601);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3602);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 5107);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6371);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8358(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 5108);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6372);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8359(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 903);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2087);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2088);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3603);
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
				['ρ', s @ ..] => {
					super::push(out, 904);
					match s {
						[] => true,
						['δ', s @ ..] => super::t2::n2125(s, out),
						['π', s @ ..] => super::t2::n2126(s, out),
						['ρ', s @ ..] => super::t2::n2127(s, out),
						['τ', s @ ..] => super::t2::n2128(s, out),
						_ => false,
					}
				}
				['φ', s @ ..] => super::t2::n906(s, out),
				['ῥ', s @ ..] => super::t2::n907(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t2::n259(s, out),
		['ι', s @ ..] => super::t2::n260(s, out),
		['λ', s @ ..] => super::t2::n261(s, out),
		['μ', s @ ..] => super::t2::n262(s, out),
		['ρ', s @ ..] => super::t2::n263(s, out),
		_ => false,
	}
}

/// Words ending -ρω.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2105);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 265);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2106);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2107);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3626);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 5133);
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
									super::push(out, 3627);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5134);
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
									super::push(out, 3628);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5135);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 6393);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8381(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2108);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3629);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5136);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6394);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 3630);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5137);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6395);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8382(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2109);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3631);
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
						['μ', s @ ..] => {
							super::push(out, 2110);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3632);
									match s {
										[] => true,
										['ξ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 5138);
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
							super::push(out, 2111);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3633);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 5139);
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
									super::push(out, 3634);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5140);
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
						['ρ', s @ ..] => {
							super::push(out, 2112);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3635);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5141);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 6396);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8383(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2113);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3636);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 5142);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6397);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8384(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t2::n266(s, out),
		['ρ', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ϝ', s @ ..] => {
							super::push(out, 2132);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t2::n268(s, out),
		_ => false,
	}
}

/// Words ending -σω.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['σ', s @ ..] => super::t2::n269(s, out),
		_ => false,
	}
}

/// Words ending -τω.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2165);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3715);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 5211);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6441);
													match s {
														[] => true,
														['τ', s @ ..] => super::t2::n8424(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 271);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2166);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3716);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5212);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 6442);
													match s {
														[] => true,
														['σ', s @ ..] => super::t2::n8425(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 272);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2167);
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
		['π', s @ ..] => super::t2::n273(s, out),
		['τ', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2195);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3763);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5252);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6483);
													s.is_empty()
												}
												['ν', s @ ..] => {
													super::push(out, 6484);
													match s {
														[] => true,
														['υ', s @ ..] => super::t2::n8465(s, out),
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
									super::push(out, 3764);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 5253);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6485);
													s.is_empty()
												}
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
							super::push(out, 2196);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3765);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 5254);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6486);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 3766);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5255);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6487);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8466(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2197);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 3767);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5256);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 3768);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5257);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2198);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2199);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3770);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => super::t2::n2242(s, out),
						_ => false,
					}
				}
				['η', s @ ..] => super::t2::n946(s, out),
				['ι', s @ ..] => super::t2::n947(s, out),
				['υ', s @ ..] => super::t2::n948(s, out),
				['ω', s @ ..] => super::t2::n949(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υω.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 275);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 951);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2210);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3779);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5266);
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
					super::push(out, 952);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2211);
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
					super::push(out, 953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2212);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2213);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3780);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5267);
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
					super::push(out, 954);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2214);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 3781);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 5268);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 6491);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8469(s, out),
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
									super::push(out, 3782);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5269);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6492);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8470(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2215);
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
		['δ', s @ ..] => {
			super::push(out, 277);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 955);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2216);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3783);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5270);
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
					super::push(out, 957);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2217);
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
					super::push(out, 958);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2218);
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
					super::push(out, 959);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2219);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3784);
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
		['ε', s @ ..] => super::t2::n278(s, out),
		['θ', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2281);
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
			super::push(out, 280);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 978);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 979);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2282);
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
					super::push(out, 980);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2283);
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
		['λ', s @ ..] => super::t2::n281(s, out),
		['μ', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 991);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2299);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3871);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5361);
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
				['μ', s @ ..] => {
					super::push(out, 992);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2300);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3872);
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
		['ν', s @ ..] => super::t2::n283(s, out),
		['ξ', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t2::n285(s, out),
		['ρ', s @ ..] => super::t2::n286(s, out),
		['σ', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['σ', s @ ..] => super::t2::n1010(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => super::t2::n288(s, out),
		['φ', s @ ..] => super::t2::n289(s, out),
		['χ', s @ ..] => super::t2::n290(s, out),
		_ => false,
	}
}

/// Words ending -φω.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1016);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2328);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3904);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3905);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 5384);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6582);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8554(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 5385);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6583);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8555(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 5386);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6584);
													match s {
														[] => true,
														['κ', s @ ..] => super::t2::n8556(s, out),
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
									super::push(out, 3906);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5387);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 5388);
											match s {
												[] => true,
												['ξ', s @ ..] => {
													super::push(out, 6585);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8557(s, out),
														_ => false,
													}
												}
												['σ', s @ ..] => {
													super::push(out, 6586);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8558(s, out),
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
									super::push(out, 3907);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5389);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6587);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8559(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 5390);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6588);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8560(s, out),
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
									super::push(out, 3908);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5391);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 6589);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8561(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 5392);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6590);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8562(s, out),
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
									super::push(out, 3909);
									match s {
										[] => true,
										['ε', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 5393);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6591);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8563(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 5394);
											match s {
												[] => true,
												['ρ', s @ ..] => super::t2::n7406(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t2::n292(s, out),
		['η', s @ ..] => super::t2::n293(s, out),
		['ι', s @ ..] => super::t2::n294(s, out),
		['ρ', s @ ..] => super::t2::n295(s, out),
		['υ', s @ ..] => super::t2::n296(s, out),
		_ => false,
	}
}

/// Words ending -χω.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1028);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2338);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3916);
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
					super::push(out, 1029);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2339);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3917);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 5402);
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
			super::push(out, 298);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2340);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2341);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3918);
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
				['ε', s @ ..] => {
					super::push(out, 1031);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2342);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3919);
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
		['ε', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1032);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1033);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2343);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3920);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5403);
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
					super::push(out, 1034);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2344);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3921);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2345);
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
					super::push(out, 1035);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2346);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3922);
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
					super::push(out, 1036);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2347);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2348);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1037);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2349);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3924);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 5404);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 6600);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t2::n8573(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['β', s @ ..] => {
							super::push(out, 2350);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => super::t2::n2398(s, out),
						['τ', s @ ..] => super::t2::n2399(s, out),
						_ => false,
					}
				}
				['σ', s @ ..] => super::t2::n1040(s, out),
				['τ', s @ ..] => super::t2::n1041(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t2::n300(s, out),
		['ι', s @ ..] => super::t2::n301(s, out),
		['ρ', s @ ..] => super::t2::n302(s, out),
		['σ', s @ ..] => super::t2::n303(s, out),
		['υ', s @ ..] => super::t2::n304(s, out),
		_ => false,
	}
}

/// Words ending -ψω.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 305);
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

/// Words ending -ωω.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1051);
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

/// Words ending -μαι.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2396);
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
					super::push(out, 1075);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2397);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3966);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5436);
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
							super::push(out, 2398);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3967);
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
							super::push(out, 2399);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3968);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 5437);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2400);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3969);
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
				['ρ', s @ ..] => {
					super::push(out, 1076);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2401);
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
					super::push(out, 1077);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2402);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3970);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5438);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6621);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8584(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 328);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1078);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2403);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3971);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5439);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6622);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8585(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 329);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2404);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3972);
									s.is_empty()
								}
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
			super::push(out, 330);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2405);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => super::t2::n4202(s, out),
								['γ', s @ ..] => super::t2::n4203(s, out),
								['ι', s @ ..] => super::t2::n4204(s, out),
								['ο', s @ ..] => super::t2::n4205(s, out),
								['σ', s @ ..] => super::t2::n4206(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t2::n331(s, out),
		['υ', s @ ..] => super::t2::n332(s, out),
		_ => false,
	}
}

/// Words ending -εθνηκεναι.
pub(crate) fn n7627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6780);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 7523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εννηθηναι.
pub(crate) fn n7628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6781);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 7524);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γεγενησαι.
pub(crate) fn n7629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6782);
	s.is_empty()
}

/// Words ending -γεγενηται.
pub(crate) fn n7630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6783);
	s.is_empty()
}

/// Words ending -εγενηνται.
pub(crate) fn n7631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6784);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 7525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θημι.
pub(crate) fn n338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1108);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4124);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 4125);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5628);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6785);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7526);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 5629);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6786);
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
								['ρ', s @ ..] => {
									super::push(out, 5630);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6787);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7528);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n9434(s, out),
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
									super::push(out, 5631);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6788);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7529);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n9435(s, out),
														_ => false,
													}
												}
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
							super::push(out, 4126);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5632);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6789);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['ν', s @ ..] => {
													super::push(out, 7530);
													match s {
														[] => true,
														['υ', s @ ..] => super::t2::n9436(s, out),
														_ => false,
													}
												}
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
							super::push(out, 4127);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5633);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6790);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 4128);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5634);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6791);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 6792);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7531);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n9437(s, out),
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
									super::push(out, 5635);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 6793);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7532);
													s.is_empty()
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 6794);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7533);
													s.is_empty()
												}
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
							super::push(out, 4129);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5636);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6795);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7534);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 6796);
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
									super::push(out, 5637);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6797);
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
							super::push(out, 4130);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5638);
									match s {
										[] => true,
										['ρ', s @ ..] => super::t2::n7645(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νιημι.
pub(crate) fn n1115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1111);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2493);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4132);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4133);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5639);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6799);
									s.is_empty()
								}
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
			super::push(out, 2494);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2495);
			match s {
				[] => true,
				['ξ', '\u{1}', s @ ..] => {
					super::push(out, 4135);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 4136);
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

/// The word εξιημι.
pub(crate) fn n4370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4137);
	s.is_empty()
}

/// Words ending -οιημι.
pub(crate) fn n1117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1113);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 2497);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4138);
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

/// Words ending -φιημι.
pub(crate) fn n1118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1114);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2498);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2499);
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

/// Words ending -λημι.
pub(crate) fn n340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2500);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4140);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5640);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 6800);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 7536);
													match s {
														[] => true,
														['α', s @ ..] => super::t2::n9439(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['μ', s @ ..] => {
											super::push(out, 6801);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7537);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n9440(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νημι.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1116);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2501);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4141);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5641);
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
			super::push(out, 1117);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2502);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4142);
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
		['μ', s @ ..] => {
			super::push(out, 1118);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2503);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4143);
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
		['ρ', s @ ..] => {
			super::push(out, 1119);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2504);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4144);
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

/// Words ending -ρημι.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 342);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2505);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4145);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5642);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['μ', s @ ..] => {
											super::push(out, 6802);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7538);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n9441(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τημι.
pub(crate) fn n343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 343);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 1121);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4146);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 4147);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5643);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6803);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7539);
													s.is_empty()
												}
												['γ', s @ ..] => {
													super::push(out, 7540);
													match s {
														[] => true,
														['ε', s @ ..] => super::t2::n9442(s, out),
														_ => false,
													}
												}
												['ι', s @ ..] => {
													super::push(out, 7541);
													match s {
														[] => true,
														['τ', s @ ..] => super::t2::n9443(s, out),
														_ => false,
													}
												}
												['ο', s @ ..] => {
													super::push(out, 7542);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n9444(s, out),
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
									super::push(out, 5644);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6804);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7543);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 5645);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6805);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7544);
													s.is_empty()
												}
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
							super::push(out, 4148);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 5646);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6806);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 5647);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6807);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7545);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n9445(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => super::t2::n4383(s, out),
						['ξ', s @ ..] => {
							super::push(out, 4150);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5651);
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
						['ο', s @ ..] => super::t2::n4385(s, out),
						['ρ', s @ ..] => super::t2::n4386(s, out),
						['φ', s @ ..] => super::t2::n4387(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φημι.
pub(crate) fn n344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 344);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1122);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2507);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4154);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιμι.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2508);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4155);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5659);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6821);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1125);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2509);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2510);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4157);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5660);
											s.is_empty()
										}
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
					super::push(out, 1126);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2511);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4158);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5661);
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
					super::push(out, 1127);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2512);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4159);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5662);
											s.is_empty()
										}
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
					super::push(out, 1128);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2513);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4161);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1129);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2515);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4162);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5663);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 5664);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6822);
													match s {
														[] => true,
														['ε', s @ ..] => super::t2::n8775(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1130);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2516);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4163);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5665);
											s.is_empty()
										}
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
					super::push(out, 1131);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2517);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2518);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4165);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5666);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 346);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2519);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4166);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 5667);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6823);
													match s {
														[] => true,
														['τ', s @ ..] => super::t2::n8776(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

