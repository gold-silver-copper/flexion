use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ω', s @ ..] => super::t0::n1(s, out),
		_ => false,
	}
}

/// Words ending -ω.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 17);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 88);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 18);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 89);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 419);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 1114);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 1960);
													match s {
														[] => true,
														['κ', s @ ..] => super::t0::n2655(s, out),
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
									super::push(out, 420);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1115);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 1961);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 19);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 90);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 421);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1116);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 1117);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 1962);
													match s {
														[] => true,
														['δ', s @ ..] => super::t0::n2656(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 20);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 91);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 92);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 423);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 1118);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 1963);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n2657(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 21);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 93);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 424);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 94);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 425);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1119);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 1964);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n2658(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 1120);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 1965);
													match s {
														[] => true,
														['υ', s @ ..] => super::t0::n2659(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 22);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 95);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 426);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 1121);
											s.is_empty()
										}
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
					super::push(out, 23);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 97);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 427);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 1122);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 98);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 99);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 429);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1123);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 100);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 430);
									s.is_empty()
								}
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
			super::push(out, 6);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 24);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 101);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 431);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 1124);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 1966);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n7(s, out),
		['λ', s @ ..] => super::t0::n8(s, out),
		['μ', s @ ..] => super::t0::n9(s, out),
		['ν', s @ ..] => super::t0::n10(s, out),
		['ξ', s @ ..] => super::t0::n11(s, out),
		['ρ', s @ ..] => super::t0::n12(s, out),
		['σ', s @ ..] => super::t0::n13(s, out),
		['υ', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['ε', s @ ..] => super::t0::n76(s, out),
				_ => false,
			}
		}
		['ψ', s @ ..] => super::t0::n15(s, out),
		['ω', s @ ..] => super::t0::n16(s, out),
		_ => false,
	}
}

/// Words ending -καταβιβω.
pub(crate) fn n2655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2653);
	s.is_empty()
}

/// Words ending -διασκεδω.
pub(crate) fn n2656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2654);
	s.is_empty()
}

/// Words ending -αποτελεω.
pub(crate) fn n2657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2655);
	s.is_empty()
}

/// Words ending -εριμενεω.
pub(crate) fn n2658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2656);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υπομενεω.
pub(crate) fn n2659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2657);
	s.is_empty()
}

/// Words ending -ιω.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 25);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 102);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 432);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1125);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1967);
											s.is_empty()
										}
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
					super::push(out, 103);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 433);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1126);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1968);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 104);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 434);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1127);
									s.is_empty()
								}
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
			super::push(out, 27);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 105);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 28);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 106);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 436);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1128);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 1969);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2658);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3119(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 107);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 437);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 108);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 438);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1130);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1970);
											s.is_empty()
										}
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
					super::push(out, 109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 30);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 110);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 111);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 439);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1131);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 1971);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 2659);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3120(s, out),
														_ => false,
													}
												}
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
							super::push(out, 440);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1132);
									match s {
										[] => true,
										['α', 'ν', s @ ..] => {
											super::push(out, 1972);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2660);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3121(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 31);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 113);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 442);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1133);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1973);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 2661);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3122(s, out),
														_ => false,
													}
												}
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
							super::push(out, 443);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1134);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 1974);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2662);
													s.is_empty()
												}
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
							super::push(out, 444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 114);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 445);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1135);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1975);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 2663);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3123(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 115);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 446);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1136);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1976);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 32);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => super::t0::n33(s, out),
		['π', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 123);
					match s {
						[] => true,
						['ε', s @ ..] => super::t0::n451(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t0::n35(s, out),
		['τ', s @ ..] => super::t0::n36(s, out),
		['φ', s @ ..] => {
			super::push(out, 37);
			s.is_empty()
		}
		['χ', s @ ..] => super::t0::n38(s, out),
		_ => false,
	}
}

/// Words ending -λω.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 39);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 133);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 462);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 463);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 1149);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 1986);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2669);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 1150);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1987);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 1151);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1988);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 1152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 464);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1153);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1989);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2670);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 1154);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1990);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2671);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['φ', s @ ..] => {
									super::push(out, 1155);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 1991);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2672);
													s.is_empty()
												}
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
							super::push(out, 465);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 466);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1157);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 1158);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 1992);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 467);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1159);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1993);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 1160);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1994);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2673);
													s.is_empty()
												}
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
							super::push(out, 468);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1161);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1995);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 2674);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 135);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 470);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1162);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1996);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2675);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 1163);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1997);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 2676);
													s.is_empty()
												}
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
							super::push(out, 471);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1164);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 1998);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 472);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1165);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1999);
											s.is_empty()
										}
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
					super::push(out, 136);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 473);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 137);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n40(s, out),
		['ι', s @ ..] => {
			super::push(out, 41);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 42);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 146);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μω.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 148);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 149);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 489);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1179);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2009);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 151);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 491);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1180);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2010);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 492);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1181);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2011);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 1182);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2012);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2682);
													s.is_empty()
												}
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
							super::push(out, 493);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1183);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 494);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1184);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2013);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 495);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1185);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2014);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 45);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 152);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 496);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1186);
									s.is_empty()
								}
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
			super::push(out, 46);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 153);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 154);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 497);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 1187);
									s.is_empty()
								}
								_ => false,
							}
						}
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
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 47);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 155);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 498);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1188);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2015);
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
								['ε', s @ ..] => {
									super::push(out, 1189);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2016);
											s.is_empty()
										}
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
					super::push(out, 156);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 500);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1190);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2017);
											s.is_empty()
										}
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
					super::push(out, 157);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 501);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1191);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2018);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 502);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 1192);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2019);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 503);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 505);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 506);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1193);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2020);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2683);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 159);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 507);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1194);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2021);
											s.is_empty()
										}
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
					super::push(out, 160);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 508);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1195);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2022);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 1196);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2023);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2684);
													match s {
														[] => true,
														['υ', s @ ..] => super::t0::n3129(s, out),
														_ => false,
													}
												}
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
							super::push(out, 509);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1197);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2024);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 510);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1198);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2025);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2685);
													s.is_empty()
												}
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
							super::push(out, 511);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1199);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 161);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 162);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 512);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1200);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2026);
											match s {
												[] => true,
												['χ', s @ ..] => {
													super::push(out, 2686);
													s.is_empty()
												}
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
							super::push(out, 513);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 1201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n163(s, out),
				['φ', s @ ..] => super::t0::n164(s, out),
				['χ', s @ ..] => {
					super::push(out, 165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => super::t0::n166(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n48(s, out),
		['η', s @ ..] => super::t0::n49(s, out),
		['ι', s @ ..] => super::t0::n50(s, out),
		['υ', s @ ..] => super::t0::n51(s, out),
		_ => false,
	}
}

/// Words ending -ξω.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 186);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 187);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 560);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 188);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 561);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 189);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 562);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1254);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1255);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2067);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2708);
													s.is_empty()
												}
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
							super::push(out, 563);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1256);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2068);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 2709);
													s.is_empty()
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 2069);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2710);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 2711);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2070);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2712);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2071);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 2713);
													s.is_empty()
												}
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
							super::push(out, 564);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 1257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 190);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 565);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1258);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 566);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 567);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1259);
									s.is_empty()
								}
								['τ', s @ ..] => super::t0::n1260(s, out),
								_ => false,
							}
						}
						['ε', s @ ..] => super::t0::n568(s, out),
						['ι', s @ ..] => super::t0::n569(s, out),
						['υ', s @ ..] => super::t0::n570(s, out),
						_ => false,
					}
				}
				['ξ', s @ ..] => super::t0::n192(s, out),
				['ο', s @ ..] => super::t0::n193(s, out),
				['π', s @ ..] => super::t0::n194(s, out),
				['ρ', s @ ..] => super::t0::n195(s, out),
				['σ', s @ ..] => super::t0::n196(s, out),
				['τ', s @ ..] => super::t0::n197(s, out),
				['φ', s @ ..] => super::t0::n198(s, out),
				['ῥ', s @ ..] => super::t0::n199(s, out),
				_ => false,
			}
		}
		['γ', s @ ..] => super::t0::n53(s, out),
		['ε', s @ ..] => super::t0::n54(s, out),
		['η', s @ ..] => super::t0::n55(s, out),
		['ι', s @ ..] => super::t0::n56(s, out),
		['λ', s @ ..] => super::t0::n57(s, out),
		['ο', s @ ..] => super::t0::n58(s, out),
		['ρ', s @ ..] => super::t0::n59(s, out),
		['υ', s @ ..] => super::t0::n60(s, out),
		['ω', s @ ..] => super::t0::n61(s, out),
		_ => false,
	}
}

/// Words ending -ρω.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 253);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 254);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 687);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1385);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2145);
											s.is_empty()
										}
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
					super::push(out, 255);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 256);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 257);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 258);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 691);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 692);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1387);
									s.is_empty()
								}
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
			super::push(out, 63);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 259);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 260);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 694);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1389);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2146);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 1390);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2147);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2753);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 261);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 262);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 695);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1391);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2148);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 2754);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 263);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 264);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 696);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 265);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 697);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1393);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2149);
											s.is_empty()
										}
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
					super::push(out, 266);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 698);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 699);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1395);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 1396);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2150);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 2755);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2151);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2756);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3151(s, out),
														_ => false,
													}
												}
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
							super::push(out, 700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 267);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 701);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1398);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2152);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2757);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 268);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 702);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 703);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1400);
									s.is_empty()
								}
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
			super::push(out, 64);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 269);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 271);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 706);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1401);
									s.is_empty()
								}
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
			super::push(out, 65);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 272);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σω.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 66);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 273);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 274);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 707);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 275);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 276);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 277);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 708);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 709);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 278);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 710);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 711);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 712);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1406);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2153);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2758);
													s.is_empty()
												}
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
							super::push(out, 713);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1407);
									match s {
										[] => true,
										['τ', s @ ..] => super::t0::n2155(s, out),
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 1408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 714);
							s.is_empty()
						}
						['τ', s @ ..] => super::t0::n715(s, out),
						['χ', s @ ..] => {
							super::push(out, 716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => super::t0::n279(s, out),
				['λ', s @ ..] => super::t0::n280(s, out),
				['μ', s @ ..] => super::t0::n281(s, out),
				['ν', s @ ..] => super::t0::n282(s, out),
				['ξ', s @ ..] => super::t0::n283(s, out),
				['ο', s @ ..] => {
					super::push(out, 284);
					s.is_empty()
				}
				['π', s @ ..] => super::t0::n285(s, out),
				['ρ', s @ ..] => super::t0::n286(s, out),
				['τ', s @ ..] => super::t0::n287(s, out),
				['υ', s @ ..] => super::t0::n288(s, out),
				['χ', s @ ..] => super::t0::n289(s, out),
				['ω', s @ ..] => super::t0::n290(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n67(s, out),
		['η', s @ ..] => super::t0::n68(s, out),
		['ι', s @ ..] => super::t0::n69(s, out),
		['λ', s @ ..] => super::t0::n70(s, out),
		['ο', s @ ..] => super::t0::n71(s, out),
		['ρ', s @ ..] => super::t0::n72(s, out),
		['σ', s @ ..] => super::t0::n73(s, out),
		['υ', s @ ..] => super::t0::n74(s, out),
		['ω', s @ ..] => super::t0::n75(s, out),
		_ => false,
	}
}

/// Words ending -ευω.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1045);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1890);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2584);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3062);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψω.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 377);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1047);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1048);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1891);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2585);
											s.is_empty()
										}
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
					super::push(out, 380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1049);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1050);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1892);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2586);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1051);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1893);
									match s {
										[] => true,
										['ξ', '\u{1}', s @ ..] => {
											super::push(out, 2587);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 2588);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3063);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 382);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1053);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1054);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1894);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2589);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3064);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 383);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1055);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 1056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1057);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 384);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1058);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1897);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1059);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1060);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 1899);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2590);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 2591);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3065);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3339(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 385);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1061);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 386);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1062);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1901);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 1902);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 2592);
											match s {
												[] => true,
												['ξ', s @ ..] => {
													super::push(out, 3066);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3340(s, out),
														_ => false,
													}
												}
												['σ', s @ ..] => {
													super::push(out, 3067);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3341(s, out),
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
									super::push(out, 1903);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 1904);
									match s {
										[] => true,
										['π', s @ ..] => super::t0::n2594(s, out),
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 1905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => super::t0::n1063(s, out),
						['τ', s @ ..] => super::t0::n1064(s, out),
						_ => false,
					}
				}
				['ῥ', s @ ..] => super::t0::n387(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n78(s, out),
		['η', s @ ..] => super::t0::n79(s, out),
		['ι', s @ ..] => super::t0::n80(s, out),
		['λ', s @ ..] => super::t0::n81(s, out),
		['μ', s @ ..] => super::t0::n82(s, out),
		['ο', s @ ..] => super::t0::n83(s, out),
		['ρ', s @ ..] => super::t0::n84(s, out),
		['υ', s @ ..] => super::t0::n85(s, out),
		['ω', s @ ..] => super::t0::n86(s, out),
		_ => false,
	}
}

/// Words ending -ωω.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 87);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αναποδιω.
pub(crate) fn n3119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3115);
	s.is_empty()
}

/// The word σολοικιω.
pub(crate) fn n3120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3116);
	s.is_empty()
}

/// The word εναττικιω.
pub(crate) fn n3121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3117);
	s.is_empty()
}

/// The word γαργαλιω.
pub(crate) fn n3122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3118);
	s.is_empty()
}

/// Words ending -ευαγγελιω.
pub(crate) fn n3123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3119);
	s.is_empty()
}

/// Words ending -νιω.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 117);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 118);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 447);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 119);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 448);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1138);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1977);
									s.is_empty()
								}
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
			super::push(out, 121);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 449);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1139);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1978);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2664);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3120);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 122);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 450);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1140);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ελπιω.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ριω.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 452);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1142);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1980);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2665);
											s.is_empty()
										}
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
					super::push(out, 453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 125);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 126);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 454);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1144);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1981);
									s.is_empty()
								}
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
			super::push(out, 128);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 456);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 457);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1145);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1982);
									s.is_empty()
								}
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
			super::push(out, 129);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 458);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1146);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1983);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2666);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3121);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3366(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 459);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τιω.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['μ', 'η', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1147);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1984);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2667);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 3122);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3367(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χιω.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['ε', 'τ', 'ι', s @ ..] => {
					super::push(out, 461);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1148);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1985);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2668);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3123);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ελω.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 475);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1167);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2000);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2001);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2677);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3124);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 139);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 476);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1168);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2002);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 2678);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 140);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 478);
					match s {
						[] => true,
						['ξ', '\u{1}', s @ ..] => {
							super::push(out, 1170);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 481);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 482);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1173);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2003);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2679);
											s.is_empty()
										}
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
					super::push(out, 483);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1174);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 484);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1176);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2006);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2680);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1177);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2007);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2681);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1178);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υποσημανω.
pub(crate) fn n3129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3125);
	s.is_empty()
}

/// Words ending -ρανω.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 514);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1207);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2029);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2687);
									s.is_empty()
								}
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
			super::push(out, 520);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1208);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 521);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φανω.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥανω.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 528);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ενω.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 167);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1215);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2034);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2688);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 169);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 532);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1216);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1217);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2036);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2689);
											s.is_empty()
										}
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
					super::push(out, 533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1218);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1219);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2037);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2690);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3126);
													s.is_empty()
												}
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
							super::push(out, 1220);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1221);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2038);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2691);
											s.is_empty()
										}
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
					super::push(out, 534);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1223);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2039);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2692);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1224);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηνω.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ινω.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 536);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1225);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2041);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2693);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 172);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 537);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1226);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2042);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2694);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3127);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 173);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 538);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1227);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2043);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2695);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3128);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 2044);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2696);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2045);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2697);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1228);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2046);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2698);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υνω.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 539);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1229);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2047);
									s.is_empty()
								}
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
			super::push(out, 175);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 540);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1230);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 541);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1231);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2049);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2699);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 176);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 543);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1233);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 544);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1234);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2051);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2700);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2052);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2701);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3129);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 177);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 545);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 546);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1236);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 547);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1238);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2054);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2702);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3130);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 179);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1239);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2055);
									s.is_empty()
								}
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
			super::push(out, 180);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 550);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1240);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2056);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2703);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 181);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 551);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 552);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1242);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2057);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2704);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2058);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2705);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3131);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 553);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 554);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1244);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2059);
									s.is_empty()
								}
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
			super::push(out, 183);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1245);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2060);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1246);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2061);
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
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1247);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2062);
									s.is_empty()
								}
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
			super::push(out, 184);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 557);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1248);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2063);
									s.is_empty()
								}
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
			super::push(out, 185);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 558);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1249);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => super::t0::n1250(s, out),
						_ => false,
					}
				}
				['σ', s @ ..] => super::t0::n559(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταναξω.
pub(crate) fn n1260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1260);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 2072);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2714);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εναξω.
pub(crate) fn n568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 568);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιναξω.
pub(crate) fn n569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 569);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 1262);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2073);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2715);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2074);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2716);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υναξω.
pub(crate) fn n570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 570);
	match s {
		[] => true,
		['ξ', s @ ..] => {
			super::push(out, 1263);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1264);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2076);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξαξω.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 571);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1265);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οαξω.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 572);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -παξω.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 574);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 575);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1270);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2078);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2717);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2718);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3134);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 2079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 576);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ραξω.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 577);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1273);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 578);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 579);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1276);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1277);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2083);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1278);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2084);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2720);
									s.is_empty()
								}
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
			super::push(out, 580);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σαξω.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 581);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1279);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταξω.
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1281);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1282);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1283);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2088);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2721);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1284);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2089);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2722);
									s.is_empty()
								}
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
			super::push(out, 583);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1285);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2723);
									s.is_empty()
								}
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
			super::push(out, 584);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1286);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1287);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2092);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2724);
									s.is_empty()
								}
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
			super::push(out, 586);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1288);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1289);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2093);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2725);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3135);
											s.is_empty()
										}
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
					super::push(out, 1290);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2094);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2726);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φαξω.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1291);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 588);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ῥαξω.
pub(crate) fn n199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 199);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 589);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γξω.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 591);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1294);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εξω.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 597);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 599);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1299);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2097);
									s.is_empty()
								}
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
			super::push(out, 206);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1300);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2098);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1301);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2099);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2727);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1302);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2100);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2728);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3136);
													s.is_empty()
												}
												['γ', s @ ..] => {
													super::push(out, 3137);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3368(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 601);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1304);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 603);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1305);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2102);
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
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1306);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1307);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2103);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2730);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3138);
													s.is_empty()
												}
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
							super::push(out, 1308);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 607);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 609);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1313);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2105);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2731);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3139);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3369(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 610);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 611);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1315);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 612);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1317);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2107);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2732);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3140);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 615);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1319);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2108);
									s.is_empty()
								}
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
			super::push(out, 211);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => super::t0::n620(s, out),
				_ => false,
			}
		}
		['ψ', s @ ..] => super::t0::n213(s, out),
		['ῥ', s @ ..] => super::t0::n214(s, out),
		_ => false,
	}
}

/// Words ending -ηξω.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 622);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 623);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1323);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1324);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1325);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2110);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2733);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3141);
													s.is_empty()
												}
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
							super::push(out, 1326);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2111);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1327);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2112);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2734);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 218);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1328);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2113);
									s.is_empty()
								}
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
			super::push(out, 220);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1329);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1331);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2114);
									s.is_empty()
								}
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
			super::push(out, 221);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 630);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1332);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2115);
									s.is_empty()
								}
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
			super::push(out, 222);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1333);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2116);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2735);
											s.is_empty()
										}
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
					super::push(out, 633);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1334);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2117);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2736);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3142);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 223);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		['ῥ', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιξω.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 636);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 637);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 638);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1335);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1336);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2118);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1337);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2119);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2737);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 228);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 640);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1338);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2120);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2738);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1339);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1340);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2122);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2739);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2740);
											s.is_empty()
										}
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
					super::push(out, 641);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1341);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1342);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2123);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2741);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1343);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2124);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2742);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1344);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2125);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2743);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3143);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3370(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 643);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1345);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2126);
									s.is_empty()
								}
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
			super::push(out, 229);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 644);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1347);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2127);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2744);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 230);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 646);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 647);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1348);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2128);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2745);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 3144);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3371(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 231);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1351);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 653);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λξω.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 654);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1354);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2130);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2746);
											s.is_empty()
										}
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
					super::push(out, 656);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 657);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1356);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οξω.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 658);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρξω.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 659);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1358);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1359);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1360);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 661);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1361);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2134);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2747);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3145);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3372(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 238);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 663);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1363);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1365);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1366);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2136);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2748);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 240);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υξω.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 668);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1369);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2137);
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
		['μ', s @ ..] => {
			super::push(out, 243);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 244);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 673);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 674);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 675);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 677);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1376);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2138);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2750);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1377);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2139);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2751);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 247);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 678);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1378);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2140);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 682);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1379);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωξω.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1380);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2142);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2752);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3146);
													s.is_empty()
												}
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
							super::push(out, 1381);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2143);
									s.is_empty()
								}
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
			super::push(out, 250);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 684);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1382);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1383);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2144);
									s.is_empty()
								}
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
			super::push(out, 251);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -παρασπερω.
pub(crate) fn n3151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3147);
	s.is_empty()
}

/// Words ending -τηριασω.
pub(crate) fn n2155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2154);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 2759);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3148);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3369);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3467);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3499);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τιασω.
pub(crate) fn n715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 715);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1409);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2155);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2760);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 3149);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3370);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1410);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2156);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κασω.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 717);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λασω.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 718);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1411);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 719);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1413);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 720);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 721);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1416);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2158);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2761);
									s.is_empty()
								}
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
			super::push(out, 722);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μασω.
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 724);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1418);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2159);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2762);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2763);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3150);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3371);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 725);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 726);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1420);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νασω.
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 727);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 728);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξασω.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 729);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1421);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πασω.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 730);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 731);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 732);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 733);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2162);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2163);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1425);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρασω.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 735);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 736);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1427);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1429);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2166);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2167);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2766);
									s.is_empty()
								}
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
			super::push(out, 737);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 738);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1432);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 740);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 741);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1435);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 742);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2172);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2767);
									s.is_empty()
								}
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
			super::push(out, 743);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2173);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2768);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3151);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3372);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τασω.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 744);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 1438);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2174);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 745);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 747);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2176);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2770);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υασω.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['σ', 'α', s @ ..] => {
							super::push(out, 2177);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2771);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3152);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 2772);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3153);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', 'α', 'π', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3154);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3373);
													match s {
														[] => true,
														['ν', s @ ..] => super::t0::n3472(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3155);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3374);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2773);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3156);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3375);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1443);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 749);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χασω.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 750);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1446);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2180);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωασω.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	s.is_empty()
}

/// Words ending -εσω.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1447);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1448);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2181);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2775);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 293);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 753);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 754);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1449);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2182);
									s.is_empty()
								}
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
			super::push(out, 294);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 756);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1450);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 757);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1451);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2184);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2776);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 296);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 758);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1452);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2185);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2777);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3157);
													match s {
														[] => true,
														['μ', s @ ..] => super::t0::n3380(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 759);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1453);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2186);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2778);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3158);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2187);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2779);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3159);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 297);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1454);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2188);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2780);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3160);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 299);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1455);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 764);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1456);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2190);
									s.is_empty()
								}
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
			super::push(out, 301);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 765);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1457);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2191);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2781);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3161);
													match s {
														[] => true,
														['ι', s @ ..] => super::t0::n3381(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ησω.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 303);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 766);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 767);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1458);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2192);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2782);
											s.is_empty()
										}
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
					super::push(out, 768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 769);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1460);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1461);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2194);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2783);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3162);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3382(s, out),
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
									super::push(out, 2195);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2784);
											s.is_empty()
										}
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
					super::push(out, 770);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1462);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1464);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2196);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2785);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3163);
													s.is_empty()
												}
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
							super::push(out, 1465);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2197);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2786);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3164);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3383(s, out),
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
									super::push(out, 2198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1466);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2199);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2787);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3165);
													s.is_empty()
												}
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
							super::push(out, 1467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 772);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1468);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1469);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2201);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2788);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3166);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n3384(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 773);
					match s {
						[] => true,
						['ῥ', s @ ..] => {
							super::push(out, 1470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2202);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1471);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2203);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1472);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2204);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2205);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2789);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3167);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 776);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1473);
							match s {
								[] => true,
								['ο', 'ρ', s @ ..] => {
									super::push(out, 2206);
									match s {
										[] => true,
										['ι', s @ ..] => super::t0::n2793(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => super::t0::n777(s, out),
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n306(s, out),
		['ε', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => super::t0::n308(s, out),
		['θ', s @ ..] => super::t0::n309(s, out),
		['ι', s @ ..] => super::t0::n310(s, out),
		['κ', s @ ..] => super::t0::n311(s, out),
		['λ', s @ ..] => super::t0::n312(s, out),
		['μ', s @ ..] => super::t0::n313(s, out),
		['ν', s @ ..] => super::t0::n314(s, out),
		['ξ', s @ ..] => super::t0::n315(s, out),
		['ο', s @ ..] => super::t0::n316(s, out),
		['π', s @ ..] => super::t0::n317(s, out),
		['ρ', s @ ..] => super::t1::n318(s, out),
		['σ', s @ ..] => super::t1::n319(s, out),
		['τ', s @ ..] => super::t1::n320(s, out),
		['υ', s @ ..] => super::t1::n321(s, out),
		['φ', s @ ..] => super::t1::n322(s, out),
		['χ', s @ ..] => super::t1::n323(s, out),
		['ψ', s @ ..] => super::t1::n324(s, out),
		_ => false,
	}
}

/// Words ending -ισω.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 898);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 899);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1691);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2415);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2949);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3264);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 900);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 901);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1693);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1694);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2416);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 2950);
											s.is_empty()
										}
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
					super::push(out, 903);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1695);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2417);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2951);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3265);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3434(s, out),
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
									super::push(out, 2418);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2952);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3266);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 904);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1696);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2419);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2953);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1697);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 905);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1698);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 908);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1700);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2422);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2954);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3267);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n3435(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 329);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 909);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 910);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1701);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 911);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1702);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2424);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2955);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 330);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1703);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1704);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2425);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2956);
											s.is_empty()
										}
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
					super::push(out, 914);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1705);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1706);
							match s {
								[] => true,
								['ξ', '\u{1}', s @ ..] => {
									super::push(out, 2426);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 916);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1708);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => super::t1::n917(s, out),
				['ρ', s @ ..] => super::t1::n918(s, out),
				['σ', s @ ..] => super::t1::n919(s, out),
				['τ', s @ ..] => super::t1::n920(s, out),
				_ => false,
			}
		}
		['π', s @ ..] => super::t1::n331(s, out),
		['ρ', s @ ..] => super::t1::n332(s, out),
		['τ', s @ ..] => super::t1::n333(s, out),
		['χ', s @ ..] => super::t1::n334(s, out),
		_ => false,
	}
}

/// Words ending -λσω.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οσω.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 934);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1728);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2443);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2968);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3274);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρσω.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1730);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1731);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1732);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2445);
									s.is_empty()
								}
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
			super::push(out, 339);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σσω.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1735);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υσω.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 945);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1737);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2447);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2969);
											s.is_empty()
										}
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
					super::push(out, 946);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1738);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2448);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2970);
											s.is_empty()
										}
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
					super::push(out, 947);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1739);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2449);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 2971);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3275);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2450);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2972);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3276);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3437(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 344);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 949);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1740);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2451);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2973);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 345);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 951);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 952);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1743);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t1::n346(s, out),
		['θ', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 970);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 971);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2519);
									s.is_empty()
								}
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
			super::push(out, 348);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 973);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 974);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 975);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1808);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2521);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3021);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3311);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3452(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1809);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2522);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2523);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1813);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => super::t1::n978(s, out),
				['λ', s @ ..] => {
					super::push(out, 979);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => super::t1::n980(s, out),
				['φ', s @ ..] => {
					super::push(out, 981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => super::t1::n982(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n350(s, out),
		['ν', s @ ..] => super::t1::n351(s, out),
		['ο', s @ ..] => super::t1::n352(s, out),
		['π', s @ ..] => super::t1::n353(s, out),
		['ρ', s @ ..] => super::t1::n354(s, out),
		['σ', s @ ..] => super::t1::n355(s, out),
		['τ', s @ ..] => super::t1::n356(s, out),
		['φ', s @ ..] => super::t1::n357(s, out),
		['χ', s @ ..] => super::t1::n358(s, out),
		_ => false,
	}
}

/// Words ending -ωσω.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1005);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1007);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1847);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2554);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1848);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2555);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3042);
											s.is_empty()
										}
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
					super::push(out, 1008);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1849);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2556);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 3043);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3321);
													match s {
														[] => true,
														['ῥ', s @ ..] => super::t1::n3456(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1850);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2557);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3044);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1851);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2558);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3045);
											s.is_empty()
										}
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
					super::push(out, 1009);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1010);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1853);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2559);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3046);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3322);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3457(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1854);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2560);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3047);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3323);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3458(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1011);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1855);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2561);
									s.is_empty()
								}
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
			super::push(out, 361);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1857);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2562);
									s.is_empty()
								}
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
			super::push(out, 363);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1015);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1016);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1859);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2563);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 3048);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3324);
													s.is_empty()
												}
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
							super::push(out, 1860);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2564);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3049);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 1863);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2565);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3050);
											match s {
												[] => true,
												['τ', s @ ..] => super::t1::n3329(s, out),
												_ => false,
											}
										}
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
					super::push(out, 1018);
					s.is_empty()
				}
				['ο', s @ ..] => super::t1::n1019(s, out),
				['ρ', s @ ..] => super::t1::n1020(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t1::n365(s, out),
		['λ', s @ ..] => super::t1::n366(s, out),
		['μ', s @ ..] => super::t1::n367(s, out),
		['ν', s @ ..] => super::t1::n368(s, out),
		['ρ', s @ ..] => super::t1::n369(s, out),
		['σ', s @ ..] => super::t1::n370(s, out),
		['τ', s @ ..] => super::t1::n371(s, out),
		['φ', s @ ..] => super::t1::n372(s, out),
		['χ', s @ ..] => super::t1::n373(s, out),
		['ψ', s @ ..] => super::t1::n374(s, out),
		['ῥ', s @ ..] => super::t1::n375(s, out),
		_ => false,
	}
}

/// Words ending -επισυναψω.
pub(crate) fn n3339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3335);
	s.is_empty()
}

/// The word ξυγγραψω.
pub(crate) fn n3340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3336);
	s.is_empty()
}

/// The word συγγραψω.
pub(crate) fn n3341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3337);
	s.is_empty()
}

/// Words ending -πογραψω.
pub(crate) fn n2594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2593);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 3068);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3338);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρραψω.
pub(crate) fn n1063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1063);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τραψω.
pub(crate) fn n1064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1064);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1907);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2595);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3069);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3339);
									s.is_empty()
								}
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
			super::push(out, 1908);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2596);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥαψω.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	s.is_empty()
}

/// Words ending -εψω.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 388);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1066);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 1067);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1910);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2597);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3071);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2598);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3072);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3340);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1068);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1911);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2599);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3073);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 390);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1069);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1071);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1914);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2600);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1915);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2601);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3075);
											s.is_empty()
										}
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
					super::push(out, 1072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1916);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1917);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2602);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3076);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3341);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1073);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1918);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2603);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3077);
											s.is_empty()
										}
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
					super::push(out, 1074);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1919);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1920);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2604);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3078);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2605);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3079);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3342);
													s.is_empty()
												}
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
							super::push(out, 1921);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2606);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3080);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1922);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2607);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3081);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1923);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1924);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2609);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3082);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2610);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3083);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => super::t1::n1925(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['σ', s @ ..] => super::t1::n1075(s, out),
				_ => false,
			}
		}
		['φ', s @ ..] => super::t1::n393(s, out),
		['ῥ', s @ ..] => super::t1::n394(s, out),
		_ => false,
	}
}

/// Words ending -ηψω.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1928);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2615);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3089);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 397);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιψω.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1929);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2616);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2617);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3090);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3348);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2618);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3091);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3349);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3468(s, out),
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
									super::push(out, 2619);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3092);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3350);
													s.is_empty()
												}
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
							super::push(out, 1930);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 1931);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1932);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2622);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3093);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3351);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 3094);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2623);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3095);
											s.is_empty()
										}
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
					super::push(out, 1083);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1933);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2624);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1084);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1934);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2625);
									s.is_empty()
								}
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
			super::push(out, 400);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1085);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1935);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2626);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3096);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1936);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2627);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3097);
											s.is_empty()
										}
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
					super::push(out, 1086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 403);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1089);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1938);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2628);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3098);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3352);
													s.is_empty()
												}
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
							super::push(out, 1939);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2629);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3099);
											s.is_empty()
										}
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
					super::push(out, 1090);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1940);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2630);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3100);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3353);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3469(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1941);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2631);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3101);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1942);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2632);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3102);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 404);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1091);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λψω.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1092);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1943);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2633);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3103);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3354);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 406);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μψω.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1945);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1946);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2634);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3104);
											s.is_empty()
										}
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
					super::push(out, 1095);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1947);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1948);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2635);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1096);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1949);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2636);
									s.is_empty()
								}
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
			super::push(out, 408);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1097);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1950);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1951);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οψω.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1098);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1099);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1100);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρψω.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1104);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υψω.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 414);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1107);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1953);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2638);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2639);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3105);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3355);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 2640);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 3106);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3356);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3470(s, out),
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
									super::push(out, 2641);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3107);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3357);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1111);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1956);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2642);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2643);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3108);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3358);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2644);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2645);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2646);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2647);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3109);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3359);
													s.is_empty()
												}
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
							super::push(out, 1957);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2648);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2649);
									s.is_empty()
								}
								['φ', s @ ..] => {
									super::push(out, 2650);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3110);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3360);
													s.is_empty()
												}
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
							super::push(out, 1958);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2651);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3111);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1959);
							match s {
								[] => true,
								['π', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3112);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3361);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 3113);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2652);
									s.is_empty()
								}
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
			super::push(out, 417);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωψω.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word αναγνωριω.
pub(crate) fn n3366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3362);
	s.is_empty()
}

/// The word γεννηματιω.
pub(crate) fn n3367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3363);
	s.is_empty()
}

/// Words ending -ραχυνω.
pub(crate) fn n1250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1250);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 2065);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2707);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σχυνω.
pub(crate) fn n559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 559);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1251);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2066);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εγκαταλεξω.
pub(crate) fn n3368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3364);
	s.is_empty()
}

/// Words ending -προσπαρεξω.
pub(crate) fn n3369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3365);
	s.is_empty()
}

/// Words ending -εφεξω.
pub(crate) fn n620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 620);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1321);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψεξω.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	s.is_empty()
}

/// Words ending -ῥεξω.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 621);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word προσμειξω.
pub(crate) fn n3370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3366);
	s.is_empty()
}

/// Words ending -εγκαταμιξω.
pub(crate) fn n3371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3367);
	s.is_empty()
}

/// Words ending -προκαταρξω.
pub(crate) fn n3372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3368);
	s.is_empty()
}

/// Words ending -ντιπαρασκευασω.
pub(crate) fn n3472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3468);
	s.is_empty()
}

/// Words ending -μετακαλεσω.
pub(crate) fn n3380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3376);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιδεραστεσω.
pub(crate) fn n3381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3377);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3470);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word κολυμβησω.
pub(crate) fn n3382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3378);
	s.is_empty()
}

/// The word σμαραγησω.
pub(crate) fn n3383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3379);
	s.is_empty()
}

/// Words ending -στρατηγησω.
pub(crate) fn n3384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3380);
	s.is_empty()
}

/// Words ending -ιρουργησω.
pub(crate) fn n2793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2790);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3168);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 3381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υγησω.
pub(crate) fn n777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 777);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2207);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2791);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3169);
									s.is_empty()
								}
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
			super::push(out, 1475);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2208);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δησω.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 778);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1477);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1478);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1479);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2210);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2793);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3170);
											s.is_empty()
										}
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
					super::push(out, 1480);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2211);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1482);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1483);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2214);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1484);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2215);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2796);
									s.is_empty()
								}
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
			super::push(out, 780);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2216);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1487);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1488);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2218);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2797);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3171);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3382);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 782);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1489);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1490);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2219);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2798);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3172);
											s.is_empty()
										}
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
					super::push(out, 1491);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1492);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2221);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2799);
									s.is_empty()
								}
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
			super::push(out, 784);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2222);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2223);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2800);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3173);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3383);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1494);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2224);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2801);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3174);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 785);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2225);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2802);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3175);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ζησω.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 787);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1496);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2226);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1497);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 789);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1498);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θησω.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 790);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 791);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1499);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2228);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1500);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2230);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2805);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3176);
											match s {
												[] => true,
												['φ', s @ ..] => {
													super::push(out, 3384);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1502);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2231);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1503);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2232);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2807);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3177);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3385);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1504);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2233);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2808);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1505);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2234);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3178);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 792);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1507);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2237);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2810);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3179);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 794);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1509);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2239);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2240);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2811);
									s.is_empty()
								}
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
			super::push(out, 796);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1511);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2242);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2243);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2812);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2813);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3180);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 797);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2244);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2814);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2246);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1514);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2247);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2816);
									s.is_empty()
								}
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
			super::push(out, 798);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1515);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1516);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2249);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2817);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3181);
											s.is_empty()
										}
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
					super::push(out, 1517);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2250);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2818);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3182);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 800);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1518);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2251);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2819);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3183);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3386);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 801);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1519);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2252);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => super::t1::n802(s, out),
		_ => false,
	}
}

/// Words ending -ιησω.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2255);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2823);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3184);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3387);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 804);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 805);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1525);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2256);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2824);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3185);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3388);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 806);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1526);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2257);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2825);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3186);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κησω.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 311);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1527);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2259);
							match s {
								[] => true,
								['φ', 'ο', s @ ..] => {
									super::push(out, 2826);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3187);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3389);
													s.is_empty()
												}
												_ => false,
											}
										}
										['χ', s @ ..] => {
											super::push(out, 3188);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3390);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 808);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2260);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1530);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2261);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2828);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3189);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3391);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3476(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2262);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2829);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3190);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3392);
													s.is_empty()
												}
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
							super::push(out, 2263);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2830);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3191);
											s.is_empty()
										}
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
					super::push(out, 1533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 811);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2264);
							match s {
								[] => true,
								['λ', 'ο', 'π', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3192);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 3193);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λησω.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 813);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1535);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2265);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2831);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3194);
											s.is_empty()
										}
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
					super::push(out, 1536);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1537);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2266);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2832);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3195);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 815);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2267);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 2268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1539);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 816);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2270);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2271);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2834);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3196);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 2272);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2835);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3197);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3393);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1543);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1544);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2274);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 818);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1546);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2275);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2837);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3198);
											s.is_empty()
										}
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
					super::push(out, 1547);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 819);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1548);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2277);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2838);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3199);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3394);
													s.is_empty()
												}
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
							super::push(out, 2278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1549);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2279);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2839);
									s.is_empty()
								}
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
			super::push(out, 820);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1550);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1551);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2280);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1552);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2282);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2841);
									s.is_empty()
								}
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
			super::push(out, 822);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1554);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2283);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2284);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2842);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3200);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3395);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1556);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μησω.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1559);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2286);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2843);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3201);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 827);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 828);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1560);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1561);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1562);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1563);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2288);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2844);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3202);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3396);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 829);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1565);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2289);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2845);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3203);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3397);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1566);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2290);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2846);
									s.is_empty()
								}
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
			super::push(out, 831);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1567);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2291);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2847);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3204);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3398);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νησω.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1568);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1569);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2292);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1570);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2293);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2294);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2850);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1571);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2295);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2851);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3205);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3399);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1572);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2296);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2852);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3206);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 833);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1574);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2297);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2853);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 3207);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2298);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2854);
									s.is_empty()
								}
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
			super::push(out, 834);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1575);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1576);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2299);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1580);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1581);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2301);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1582);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2302);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2857);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3208);
											s.is_empty()
										}
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
					super::push(out, 1583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 838);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2304);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2858);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3209);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3400);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1587);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2305);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1588);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1589);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1590);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2307);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2308);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2861);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1592);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2309);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2862);
									match s {
										[] => true,
										['π', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3210);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 840);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 841);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2310);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2863);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3211);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 842);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1595);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2311);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2864);
									match s {
										[] => true,
										['ρ', s @ ..] => super::t1::n3216(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => super::t1::n1596(s, out),
				['σ', s @ ..] => super::t1::n1597(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t1::n843(s, out),
		_ => false,
	}
}

/// Words ending -ξησω.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 315);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1600);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1601);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2865);
									s.is_empty()
								}
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
			super::push(out, 845);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1602);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οησω.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 316);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 846);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 847);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1603);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πησω.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 317);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1606);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2318);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2866);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3213);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3402);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3477(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1607);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1609);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2319);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2867);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3214);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 852);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1610);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1611);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2321);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2868);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1612);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2869);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2870);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3215);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3403);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 853);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1613);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2323);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1614);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

