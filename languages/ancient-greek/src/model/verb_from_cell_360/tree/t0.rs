use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ι', 'α', 'μ', s @ ..] => super::t0::n1(s, out),
		_ => false,
	}
}

/// Words ending -μαι.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 13);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 121);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 553);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1298);
											match s {
												[] => true,
												['φ', s @ ..] => {
													super::push(out, 2112);
													match s {
														[] => true,
														['μ', s @ ..] => super::t0::n2876(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 122);
							match s {
								[] => true,
								['ε', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 15);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 124);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 125);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 555);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 1299);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2113);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n2877(s, out),
														_ => false,
													}
												}
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
							super::push(out, 126);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 556);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1300);
											s.is_empty()
										}
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
					super::push(out, 16);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 127);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 557);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1301);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2114);
													s.is_empty()
												}
												['ξ', s @ ..] => {
													super::push(out, 2115);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n2878(s, out),
														_ => false,
													}
												}
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
							super::push(out, 128);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 558);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 1302);
											s.is_empty()
										}
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
					super::push(out, 17);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 129);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 559);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 1303);
											s.is_empty()
										}
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
					super::push(out, 18);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 130);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 560);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 1304);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 2116);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n2879(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n20(s, out),
				['τ', s @ ..] => super::t0::n21(s, out),
				['φ', s @ ..] => super::t0::n22(s, out),
				_ => false,
			}
		}
		['γ', s @ ..] => super::t0::n3(s, out),
		['ε', s @ ..] => super::t0::n4(s, out),
		['η', s @ ..] => super::t0::n5(s, out),
		['ι', s @ ..] => super::t0::n6(s, out),
		['λ', s @ ..] => super::t0::n7(s, out),
		['μ', s @ ..] => super::t0::n8(s, out),
		['ο', s @ ..] => super::t0::n9(s, out),
		['ρ', s @ ..] => super::t0::n10(s, out),
		['σ', s @ ..] => super::t0::n11(s, out),
		['υ', s @ ..] => super::t0::n12(s, out),
		['ω', s @ ..] => super::t0::n13(s, out),
		_ => false,
	}
}

/// Words ending -μφιβεβαμαι.
pub(crate) fn n2876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2864);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3457);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3879);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τεθυμιαμαι.
pub(crate) fn n2877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2865);
	s.is_empty()
}

/// Words ending -εξεληλαμαι.
pub(crate) fn n2878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2866);
	s.is_empty()
}

/// Words ending -εριεσπαμαι.
pub(crate) fn n2879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2867);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ραμαι.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 561);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 562);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1306);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2117);
									s.is_empty()
								}
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
			super::push(out, 133);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 563);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1307);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2118);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2868);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 134);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 564);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1308);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2119);
									s.is_empty()
								}
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
			super::push(out, 135);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 565);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1309);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2120);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2869);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3459);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 136);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 566);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 568);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1311);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2121);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2870);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1312);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2122);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2871);
											s.is_empty()
										}
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
					super::push(out, 569);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1313);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2123);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2872);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3460);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3897(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταμαι.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 570);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1314);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1315);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2124);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2873);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1316);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1317);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2126);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2874);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 140);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 573);
					match s {
						[] => true,
						['θ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1321);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2127);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 1322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φαμαι.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -γμαι.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 22);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 144);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 575);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1324);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2128);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 2876);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3476(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
						['λ', s @ ..] => {
							super::push(out, 576);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 577);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 1326);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2129);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2877);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 146);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 578);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1327);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2130);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2878);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 147);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 579);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1328);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1329);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2131);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2879);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 148);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 580);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1330);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2132);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 2880);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 149);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 581);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1331);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2133);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 2881);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 1332);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2134);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 2882);
													s.is_empty()
												}
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
							super::push(out, 582);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1333);
									match s {
										[] => true,
										['δ', s @ ..] => {
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
							super::push(out, 583);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1334);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2136);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2883);
													match s {
														[] => true,
														['ι', s @ ..] => super::t0::n3477(s, out),
														_ => false,
													}
												}
												['μ', s @ ..] => super::t0::n2896(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => super::t0::n587(s, out),
						_ => false,
					}
				}
				['σ', s @ ..] => super::t0::n152(s, out),
				['τ', s @ ..] => super::t0::n153(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n24(s, out),
		['η', s @ ..] => super::t0::n25(s, out),
		['ι', s @ ..] => super::t0::n26(s, out),
		['ο', s @ ..] => super::t0::n27(s, out),
		['ρ', s @ ..] => super::t0::n28(s, out),
		['υ', s @ ..] => super::t0::n29(s, out),
		['ω', s @ ..] => super::t0::n30(s, out),
		_ => false,
	}
}

/// Words ending -εμαι.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 30);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 195);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 650);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1397);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 1398);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2193);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 2921);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2194);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2922);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 1399);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 2195);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 2923);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 196);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 32);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 197);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 652);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1400);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2196);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2924);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n3503(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 33);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 198);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 653);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1401);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2197);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 2925);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 199);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 654);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 200);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 655);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ημαι.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 35);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 36);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 201);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 656);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1404);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2198);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2926);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n3504(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 202);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 657);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 203);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 658);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1406);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 659);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1407);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2200);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 204);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 205);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 661);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 1408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 206);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 662);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 663);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 1409);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 664);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1410);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2201);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 2927);
													s.is_empty()
												}
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
							super::push(out, 665);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 666);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1412);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2202);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 2928);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3505(s, out),
														_ => false,
													}
												}
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
							super::push(out, 667);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1413);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 2203);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2929);
													match s {
														[] => true,
														['κ', s @ ..] => super::t0::n3506(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => super::t0::n671(s, out),
						_ => false,
					}
				}
				['ι', s @ ..] => super::t0::n209(s, out),
				['ο', s @ ..] => super::t0::n210(s, out),
				['ρ', s @ ..] => super::t0::n211(s, out),
				['υ', s @ ..] => super::t0::n212(s, out),
				['ω', s @ ..] => super::t0::n213(s, out),
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n39(s, out),
		['ε', s @ ..] => super::t0::n40(s, out),
		['ζ', s @ ..] => super::t0::n41(s, out),
		['θ', s @ ..] => super::t0::n42(s, out),
		['ι', s @ ..] => super::t0::n43(s, out),
		['κ', s @ ..] => super::t0::n44(s, out),
		['λ', s @ ..] => super::t0::n45(s, out),
		['μ', s @ ..] => super::t0::n46(s, out),
		['ν', s @ ..] => super::t0::n47(s, out),
		['ξ', s @ ..] => super::t0::n48(s, out),
		['ο', s @ ..] => super::t0::n49(s, out),
		['π', s @ ..] => super::t0::n50(s, out),
		['ρ', s @ ..] => super::t0::n51(s, out),
		['σ', s @ ..] => super::t0::n52(s, out),
		['τ', s @ ..] => super::t0::n53(s, out),
		['υ', s @ ..] => super::t0::n54(s, out),
		['φ', s @ ..] => super::t0::n55(s, out),
		['χ', s @ ..] => super::t0::n56(s, out),
		['ψ', s @ ..] => super::t0::n57(s, out),
		_ => false,
	}
}

/// Words ending -ιμαι.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 320);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 883);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1653);
									s.is_empty()
								}
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
			super::push(out, 58);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 321);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 322);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 884);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1654);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 885);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1655);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2430);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2431);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3109);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3635(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2432);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3110);
													s.is_empty()
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 2433);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3111);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3636(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2434);
											s.is_empty()
										}
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
					super::push(out, 323);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 886);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1656);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2435);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3112);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3637(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 324);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 325);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 326);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 889);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 327);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 891);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1657);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2436);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3113);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3638(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 60);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 328);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 892);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1658);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2437);
											s.is_empty()
										}
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
					super::push(out, 329);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 893);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1659);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2438);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3114);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 61);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 330);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['κ', s @ ..] => super::t0::n334(s, out),
				['π', s @ ..] => {
					super::push(out, 332);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 896);
							match s {
								[] => true,
								['π', s @ ..] => super::t0::n1665(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => super::t0::n336(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['ε', s @ ..] => super::t0::n337(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λμαι.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 335);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 899);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1663);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
				['τ', s @ ..] => {
					super::push(out, 336);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 900);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1664);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2446);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 2447);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3120);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3644(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2448);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3121);
													s.is_empty()
												}
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
							super::push(out, 901);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1665);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2449);
											s.is_empty()
										}
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
					super::push(out, 337);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 902);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1666);
									s.is_empty()
								}
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
			super::push(out, 65);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 338);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 903);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 339);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 66);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 340);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 341);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 906);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1668);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2450);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μμαι.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 342);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 907);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 343);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 908);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1670);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2451);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 2452);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3122);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n3645(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 344);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 909);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1671);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2453);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3123);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3646(s, out),
														_ => false,
													}
												}
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
							super::push(out, 910);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1672);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2454);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 2455);
											match s {
												[] => true,
												['α', s @ ..] => {
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
				['λ', s @ ..] => {
					super::push(out, 345);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 911);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1673);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 2456);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 912);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1674);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2457);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3125);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n349(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['κ', s @ ..] => super::t0::n350(s, out),
				['λ', s @ ..] => super::t0::n351(s, out),
				['π', s @ ..] => super::t0::n352(s, out),
				['ρ', s @ ..] => {
					super::push(out, 350);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 351);
					match s {
						[] => true,
						['σ', s @ ..] => super::t0::n925(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n71(s, out),
		['ι', s @ ..] => super::t0::n72(s, out),
		['λ', s @ ..] => super::t0::n73(s, out),
		['ο', s @ ..] => super::t0::n74(s, out),
		['υ', s @ ..] => super::t0::n75(s, out),
		_ => false,
	}
}

/// Words ending -ομαι.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 368);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 945);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1705);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 1706);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2496);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3172);
													match s {
														[] => true,
														['μ', s @ ..] => super::t0::n3693(s, out),
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
									super::push(out, 1707);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2497);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3173);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2498);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3174);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3694(s, out),
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
									super::push(out, 1708);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2499);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 1709);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2500);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3175);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n3695(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2501);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3176);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 369);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 946);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1710);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2502);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 76);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 370);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 371);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 948);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 372);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 949);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1712);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2503);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3177);
													match s {
														[] => true,
														['κ', s @ ..] => super::t0::n3696(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρμαι.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 373);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 950);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 374);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 951);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1714);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2504);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
						['φ', s @ ..] => {
							super::push(out, 952);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1715);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2505);
											s.is_empty()
										}
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
					super::push(out, 375);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 953);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 376);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 954);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 377);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 955);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 956);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1719);
									s.is_empty()
								}
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
			super::push(out, 79);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 378);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 957);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1720);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2506);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', '\u{1}', s @ ..] => {
					super::push(out, 379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 380);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 381);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 382);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 383);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 961);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1721);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 962);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1722);
									s.is_empty()
								}
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
			super::push(out, 81);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 385);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 963);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 1723);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 386);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 964);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1724);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σμαι.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['α', s @ ..] => super::t0::n84(s, out),
		['ε', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 405);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1034);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 406);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1035);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1814);
									s.is_empty()
								}
								['δ', s @ ..] => {
									super::push(out, 1815);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2595);
											s.is_empty()
										}
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
					super::push(out, 407);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1036);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1816);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 1817);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2596);
											s.is_empty()
										}
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
					super::push(out, 408);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1037);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1818);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2597);
											match s {
												[] => true,
												['τ', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3749(s, out),
														['α', s @ ..] => super::t0::n3750(s, out),
														['ι', s @ ..] => super::t0::n3751(s, out),
														['ο', s @ ..] => super::t0::n3752(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1038);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1819);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2598);
											s.is_empty()
										}
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
					super::push(out, 409);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1039);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1820);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2599);
											s.is_empty()
										}
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
					super::push(out, 410);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 411);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1041);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1821);
									s.is_empty()
								}
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
			super::push(out, 85);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 412);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1042);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1822);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2600);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1043);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1823);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2601);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3250);
													match s {
														[] => true,
														['ν', s @ ..] => super::t0::n3753(s, out),
														_ => false,
													}
												}
												['μ', s @ ..] => {
													super::push(out, 3251);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3754(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 413);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1044);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 414);
					match s {
						[] => true,
						['ε', s @ ..] => super::t0::n1048(s, out),
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n418(s, out),
				['σ', s @ ..] => super::t0::n419(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t0::n87(s, out),
		['ο', s @ ..] => super::t0::n88(s, out),
		['π', s @ ..] => super::t0::n89(s, out),
		['υ', s @ ..] => super::t1::n90(s, out),
		['ω', s @ ..] => super::t1::n91(s, out),
		_ => false,
	}
}

/// Words ending -υμαι.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1150);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1961);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2733);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3361);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3833(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 454);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1151);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1962);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
				['π', s @ ..] => {
					super::push(out, 455);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1152);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1963);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2735);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2736);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3362);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3834(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 92);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 456);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1153);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1964);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2737);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 1965);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2738);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3363);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3835(s, out),
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
									super::push(out, 1966);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2739);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t1::n94(s, out),
		['θ', s @ ..] => {
			super::push(out, 94);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 472);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1198);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['κ', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 473);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1199);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2009);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 2776);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3398);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3862(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 96);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 474);
					match s {
						[] => true,
						['λ', s @ ..] => super::t1::n1203(s, out),
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 475);
					match s {
						[] => true,
						['ε', s @ ..] => super::t1::n1204(s, out),
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 476);
					match s {
						[] => true,
						['κ', s @ ..] => super::t1::n1205(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n98(s, out),
		['ν', s @ ..] => super::t1::n99(s, out),
		['ο', s @ ..] => super::t1::n100(s, out),
		['ρ', s @ ..] => super::t1::n101(s, out),
		['σ', s @ ..] => super::t1::n102(s, out),
		['τ', s @ ..] => super::t1::n103(s, out),
		['χ', s @ ..] => super::t1::n104(s, out),
		_ => false,
	}
}

/// Words ending -ωμαι.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 495);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1224);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2040);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2809);
											s.is_empty()
										}
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
					super::push(out, 496);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1225);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2041);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2810);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3430);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 105);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 497);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 1226);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2042);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 498);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1227);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2043);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2811);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3431);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3886(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1228);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2044);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 2812);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3432);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3887(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 499);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1229);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2045);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2813);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3433);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 106);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 500);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1230);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2046);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2814);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 107);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 501);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1231);
							match s {
								[] => true,
								['δ', s @ ..] => {
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
		['θ', s @ ..] => {
			super::push(out, 108);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 502);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1232);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2048);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2815);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3434);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 109);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 503);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1233);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2049);
									match s {
										[] => true,
										['δ', s @ ..] => {
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
				['ε', s @ ..] => {
					super::push(out, 504);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1234);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2050);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2817);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1235);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2051);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 1236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 505);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1237);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2052);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2819);
											s.is_empty()
										}
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
					super::push(out, 506);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1238);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2053);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2054);
									match s {
										[] => true,
										['α', s @ ..] => super::t1::n2832(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => super::t1::n510(s, out),
				['ρ', s @ ..] => super::t1::n511(s, out),
				['τ', s @ ..] => super::t1::n512(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t1::n112(s, out),
		['λ', s @ ..] => super::t1::n113(s, out),
		['μ', s @ ..] => super::t1::n114(s, out),
		['ν', s @ ..] => super::t1::n115(s, out),
		['ξ', s @ ..] => super::t1::n116(s, out),
		['π', s @ ..] => super::t1::n117(s, out),
		['ρ', s @ ..] => super::t1::n118(s, out),
		['σ', s @ ..] => super::t1::n119(s, out),
		['τ', s @ ..] => super::t1::n120(s, out),
		['φ', s @ ..] => super::t1::n121(s, out),
		['χ', s @ ..] => super::t1::n122(s, out),
		_ => false,
	}
}

/// Words ending -αταπεφωραμαι.
pub(crate) fn n3897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3880);
	s.is_empty()
}

/// Words ending -αδεδιδαγμαι.
pub(crate) fn n3476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3461);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 3881);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4097);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιαπεπραγμαι.
pub(crate) fn n3477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3462);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 3882);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μπεπραγμαι.
pub(crate) fn n2896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2884);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 3463);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3883);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φραγμαι.
pub(crate) fn n587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 584);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1335);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σαγμαι.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1336);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταγμαι.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1337);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2138);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2885);
									s.is_empty()
								}
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
			super::push(out, 587);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1338);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2139);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2140);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2886);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3464);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3884);
													s.is_empty()
												}
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
							super::push(out, 2141);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2887);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3465);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2142);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2888);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3466);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2143);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2889);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3467);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3885);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 588);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1339);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εγμαι.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 589);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1340);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2144);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2145);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2890);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3468);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 2146);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2891);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3469);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 153);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 590);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1341);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2147);
									s.is_empty()
								}
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
			super::push(out, 154);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 591);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1342);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2148);
									s.is_empty()
								}
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
			super::push(out, 155);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1343);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2149);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2892);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3470);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 593);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1344);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2150);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2893);
											s.is_empty()
										}
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
					super::push(out, 594);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1345);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2151);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2152);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2894);
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
								['ν', s @ ..] => {
									super::push(out, 2153);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 2895);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3472);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ξ', s @ ..] => {
									super::push(out, 2154);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2896);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2155);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2897);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3473);
													match s {
														[] => true,
														['γ', s @ ..] => super::t1::n3903(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 595);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1346);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2156);
									s.is_empty()
								}
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
			super::push(out, 156);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1347);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2157);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2898);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3474);
													s.is_empty()
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 2899);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3475);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3904(s, out),
														_ => false,
													}
												}
												['υ', s @ ..] => {
													super::push(out, 3476);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3905(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 157);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 597);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1348);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1349);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2159);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2900);
											s.is_empty()
										}
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
					super::push(out, 599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => super::t1::n161(s, out),
		_ => false,
	}
}

/// Words ending -ηγμαι.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 603);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 604);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1353);
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
				['π', s @ ..] => {
					super::push(out, 605);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1354);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2161);
									s.is_empty()
								}
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
				['α', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1355);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2162);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2901);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 164);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 610);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1357);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2163);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 2164);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2902);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 167);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 611);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1358);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1359);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2165);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2903);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3477);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n3906(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 168);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1360);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2166);
									s.is_empty()
								}
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
				['ε', s @ ..] => {
					super::push(out, 613);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1361);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2167);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2904);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3478);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 614);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1362);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιγμαι.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 615);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1363);
							match s {
								[] => true,
								['δ', s @ ..] => {
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
					super::push(out, 616);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1364);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2170);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2905);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3479);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 172);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 617);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1365);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2171);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2906);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3480);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3907(s, out),
														['υ', s @ ..] => super::t1::n3908(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 618);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1366);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2172);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2907);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3481);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n3909(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 619);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1367);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2173);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2908);
											match s {
												[] => true,
												['ξ', s @ ..] => {
													super::push(out, 3482);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 620);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1368);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2174);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2909);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3483);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 173);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 621);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1369);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 622);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1370);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2176);
									s.is_empty()
								}
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
			super::push(out, 174);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1371);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2177);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2178);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2910);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3484);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3910(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 175);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1372);
							match s {
								[] => true,
								['π', s @ ..] => {
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
		['ξ', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1374);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2180);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2911);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 178);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ογμαι.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1376);
							s.is_empty()
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
				['τ', s @ ..] => {
					super::push(out, 630);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1377);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ργμαι.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1379);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 634);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1380);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 635);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1382);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2183);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2912);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 184);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 636);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υγμαι.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 637);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 638);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1385);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2184);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2913);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3485);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 639);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1386);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2185);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2914);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 186);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 640);
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
		['ν', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 641);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1389);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2186);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2915);
											s.is_empty()
										}
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
					super::push(out, 643);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1390);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2187);
									s.is_empty()
								}
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
			super::push(out, 189);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 644);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1391);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2188);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2189);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2916);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2917);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 190);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1392);
							match s {
								[] => true,
								['α', s @ ..] => {
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
		['ψ', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1393);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωγμαι.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 647);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1395);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2192);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2918);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2919);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3486);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3911(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 2920);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3487);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 194);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τακεκαλεμαι.
pub(crate) fn n3503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3488);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ταλελαβημαι.
pub(crate) fn n3504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3489);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3896);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4101);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εναυπηγημαι.
pub(crate) fn n3505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3490);
	s.is_empty()
}

/// Words ending -κεχορηγημαι.
pub(crate) fn n3506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3491);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3897);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4102);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φηγημαι.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 668);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 1414);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιγημαι.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ογημαι.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2205);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2930);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3492);
											match s {
												[] => true,
												['α', s @ ..] => {
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
				['ο', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2206);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2931);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 3493);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3899);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ργημαι.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2207);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2932);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3494);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 672);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2208);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2209);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2934);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3495);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2210);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2935);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3496);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3900);
													match s {
														[] => true,
														['λ', s @ ..] => super::t1::n4120(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 673);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υγημαι.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2211);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2936);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3497);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3901);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 675);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωγημαι.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2214);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2937);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3498);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δημαι.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 678);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1426);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2215);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2938);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 213);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1427);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2216);
									s.is_empty()
								}
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
			super::push(out, 214);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 680);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1428);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2217);
									s.is_empty()
								}
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
			super::push(out, 215);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 682);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1429);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2218);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2939);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εημαι.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ζημαι.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 685);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1431);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θημαι.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 686);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1432);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 687);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1433);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2221);
									s.is_empty()
								}
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
			super::push(out, 221);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1434);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2222);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2940);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3499);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3919(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 690);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1435);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2223);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 2941);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3500);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3920(s, out),
														_ => false,
													}
												}
												['ι', s @ ..] => {
													super::push(out, 3501);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3921(s, out),
														_ => false,
													}
												}
												['σ', s @ ..] => {
													super::push(out, 3502);
													match s {
														[] => true,
														['ο', s @ ..] => super::t1::n3922(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 222);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1436);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2224);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2942);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 224);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 693);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1437);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2225);
									s.is_empty()
								}
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
			super::push(out, 225);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 694);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1438);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2226);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2943);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 226);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1439);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιημαι.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 697);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1440);
							match s {
								[] => true,
								['π', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3503);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3923(s, out),
														_ => false,
													}
												}
												['τ', s @ ..] => {
													super::push(out, 3504);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n3924(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['μ', s @ ..] => {
											super::push(out, 2944);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3505);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2945);
											match s {
												[] => true,
												['χ', s @ ..] => {
													super::push(out, 3506);
													s.is_empty()
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 2946);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3507);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3925(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1441);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2228);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2947);
											match s {
												[] => true,
												['γ', 'η', '\u{1}', s @ ..] => {
													super::push(out, 3508);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2229);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2948);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 229);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1442);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2230);
									s.is_empty()
								}
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
			super::push(out, 230);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1443);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κημαι.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 700);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1444);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2232);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2949);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3509);
													s.is_empty()
												}
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
							super::push(out, 1445);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2233);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2950);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3510);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2951);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3511);
													s.is_empty()
												}
												['χ', s @ ..] => {
													super::push(out, 3512);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 232);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1446);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2234);
									s.is_empty()
								}
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
			super::push(out, 233);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1447);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2235);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 1448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 703);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1449);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2236);
									s.is_empty()
								}
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
			super::push(out, 234);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1450);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2237);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2952);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3513);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 235);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1451);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2238);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2953);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 236);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 706);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1453);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2239);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 2954);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 708);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 710);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1455);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2240);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
				['ν', s @ ..] => {
					super::push(out, 711);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1456);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1457);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 712);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 713);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1459);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2242);
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
				['τ', s @ ..] => {
					super::push(out, 714);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1460);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λημαι.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 715);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1461);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 1462);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2245);
									s.is_empty()
								}
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
			super::push(out, 239);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 716);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1463);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2246);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2247);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2957);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3514);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2958);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3515);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2959);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3516);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3926(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2960);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3517);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3927(s, out),
														_ => false,
													}
												}
												['ε', s @ ..] => {
													super::push(out, 3518);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n3928(s, out),
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
									super::push(out, 2248);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2961);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3519);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['κ', s @ ..] => {
									super::push(out, 2249);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2962);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 2250);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2963);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2964);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3520);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2251);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2965);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3521);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2966);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3522);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2252);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2967);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3523);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n3929(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 240);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 717);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1465);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2253);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2968);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3524);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3930(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1466);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2254);
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
				['φ', s @ ..] => {
					super::push(out, 719);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1468);
							match s {
								[] => true,
								['δ', s @ ..] => super::t1::n2262(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => super::t1::n724(s, out),
				_ => false,
			}
		}
		['θ', 'η', '\u{1}', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		['ι', s @ ..] => super::t1::n245(s, out),
		['κ', s @ ..] => super::t1::n246(s, out),
		['λ', s @ ..] => super::t1::n247(s, out),
		['ο', s @ ..] => super::t1::n248(s, out),
		['υ', s @ ..] => super::t1::n249(s, out),
		['ω', s @ ..] => super::t1::n250(s, out),
		_ => false,
	}
}

/// Words ending -μημαι.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1481);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2272);
									s.is_empty()
								}
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
			super::push(out, 250);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 733);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 734);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1483);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2273);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2984);
											match s {
												[] => true,
												['π', s @ ..] => {
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3936(s, out),
														['μ', s @ ..] => super::t1::n3937(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 735);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1484);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 736);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1486);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2275);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2985);
											match s {
												[] => true,
												['β', s @ ..] => {
													super::push(out, 3534);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3938(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1487);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2276);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2986);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3535);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 253);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 739);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1488);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2277);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2987);
											s.is_empty()
										}
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
					super::push(out, 740);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1489);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2278);
									s.is_empty()
								}
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
			super::push(out, 254);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 741);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1490);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2279);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2988);
											s.is_empty()
										}
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
					super::push(out, 742);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1491);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2280);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2989);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3536);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n3939(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 743);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1492);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2281);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2990);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3537);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3940(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 255);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1493);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2282);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2991);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 256);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 745);
					match s {
						[] => true,
						['τ', s @ ..] => super::t1::n1497(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t1::n259(s, out),
		['ρ', s @ ..] => super::t1::n260(s, out),
		['σ', s @ ..] => super::t1::n261(s, out),
		['τ', s @ ..] => super::t1::n262(s, out),
		['υ', s @ ..] => super::t1::n263(s, out),
		_ => false,
	}
}

/// Words ending -νημαι.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1510);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2302);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3014);
											s.is_empty()
										}
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
					super::push(out, 756);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1511);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2303);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3015);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3551);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 757);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1512);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2304);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3016);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3552);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 758);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1513);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2305);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3017);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3553);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3948(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 263);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 759);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1514);
							match s {
								[] => true,
								['γ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3018);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 3019);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3554);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3949(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['γ', s @ ..] => {
											super::push(out, 3020);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3555);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 3556);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n3950(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 3021);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3557);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3951(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3022);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3558);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3952(s, out),
														['υ', s @ ..] => super::t1::n3953(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 3559);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3954(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 760);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1515);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2306);
									s.is_empty()
								}
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
			super::push(out, 264);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1516);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 762);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1518);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2308);
									s.is_empty()
								}
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
			super::push(out, 265);
			match s {
				[] => true,
				['β', s @ ..] => super::t1::n767(s, out),
				['δ', s @ ..] => super::t1::n768(s, out),
				['κ', s @ ..] => super::t1::n769(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t1::n268(s, out),
		['μ', s @ ..] => super::t1::n269(s, out),
		['ν', s @ ..] => super::t1::n270(s, out),
		['ο', s @ ..] => super::t1::n271(s, out),
		['π', s @ ..] => super::t1::n272(s, out),
		['ρ', s @ ..] => super::t1::n273(s, out),
		['υ', s @ ..] => super::t1::n274(s, out),
		['χ', s @ ..] => super::t1::n275(s, out),
		['ω', s @ ..] => super::t1::n276(s, out),
		_ => false,
	}
}

/// Words ending -ξημαι.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οημαι.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 797);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1548);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2336);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3045);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3574);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3962(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 277);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 798);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1550);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2337);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3046);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3575);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 3047);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3576);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 2338);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3048);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3577);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 2339);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3049);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2340);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3050);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3578);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 3051);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3579);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 278);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 800);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πημαι.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 801);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1552);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2341);
									s.is_empty()
								}
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
			super::push(out, 280);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1553);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2342);
									s.is_empty()
								}
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
			super::push(out, 281);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1554);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2343);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 804);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1555);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2344);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3052);
											s.is_empty()
										}
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
					super::push(out, 805);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1556);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2345);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3053);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3580);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 282);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1557);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2346);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1558);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1559);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2348);
									s.is_empty()
								}
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
			super::push(out, 283);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 809);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1560);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2349);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3054);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρημαι.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 810);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1561);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2350);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3055);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3581);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 811);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1562);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2351);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3056);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3582);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 285);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 812);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1563);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 813);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1564);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2353);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3057);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3583);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3963(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1565);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2354);
									s.is_empty()
								}
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
			super::push(out, 286);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 814);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1566);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2355);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3058);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3584);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3964(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 815);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 816);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1568);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1569);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 817);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1571);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 819);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1572);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 820);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1574);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2359);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3059);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3585);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 821);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1575);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1576);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2361);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3060);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 1577);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2362);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3061);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 3586);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3965(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 3062);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3587);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t1::n290(s, out),
		['τ', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 828);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1586);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2371);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3070);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t1::n292(s, out),
		['χ', s @ ..] => super::t1::n293(s, out),
		['ω', s @ ..] => super::t1::n294(s, out),
		_ => false,
	}
}

/// Words ending -σημαι.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1601);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2384);
									s.is_empty()
								}
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
			super::push(out, 294);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 840);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1602);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2385);
									s.is_empty()
								}
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
			super::push(out, 295);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 841);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1603);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2386);
									s.is_empty()
								}
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
			super::push(out, 296);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 843);
					match s {
						[] => true,
						['ε', 'π', 'μ', s @ ..] => {
							super::push(out, 1604);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τημαι.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1605);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2388);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3081);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3601);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3973(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1606);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 845);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1607);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2390);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3082);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3602);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 846);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1608);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2392);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3083);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['θ', s @ ..] => {
					super::push(out, 847);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1610);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2393);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3084);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3603);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3974(s, out),
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
									super::push(out, 2394);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3085);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2395);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3086);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3604);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3975(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 848);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1611);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2396);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3087);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3605);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 849);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1612);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2397);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3088);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', 'ε', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1613);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 300);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 850);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1614);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2398);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2399);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3089);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3606);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 851);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1615);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2400);
									match s {
										[] => true,
										['δ', s @ ..] => {
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
				['ξ', s @ ..] => {
					super::push(out, 852);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 853);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1617);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 854);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1618);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2402);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3091);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 301);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 855);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 302);
			match s {
				[] => true,
				['ε', s @ ..] => super::t1::n859(s, out),
				['η', s @ ..] => super::t1::n860(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t1::n305(s, out),
		['π', s @ ..] => super::t1::n306(s, out),
		['ρ', s @ ..] => super::t1::n307(s, out),
		['σ', s @ ..] => super::t1::n308(s, out),
		['υ', s @ ..] => super::t1::n309(s, out),
		['ω', s @ ..] => super::t1::n310(s, out),
		_ => false,
	}
}

/// Words ending -υημαι.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 868);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 869);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φημαι.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 870);
					match s {
						[] => true,
						['γ', s @ ..] => {
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2414);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 2415);
									match s {
										[] => true,
										['ζ', s @ ..] => {
											super::push(out, 3098);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3611);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3979(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 311);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1635);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2416);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1636);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χημαι.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1637);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1638);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2419);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3100);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3612);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 2420);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3101);
											match s {
												[] => true,
												['φ', s @ ..] => {
													super::push(out, 3613);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 313);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 874);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1639);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2421);
									s.is_empty()
								}
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
			super::push(out, 314);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1640);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2422);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3102);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3614);
													s.is_empty()
												}
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
							super::push(out, 1641);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2423);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3103);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3615);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3980(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1642);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2424);
									match s {
										[] => true,
										['μ', s @ ..] => {
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
				['ω', s @ ..] => {
					super::push(out, 877);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1644);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1645);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2425);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3105);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1648);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2426);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3106);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3616);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 880);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1649);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2427);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3107);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3617);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3981(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 881);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1650);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1651);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2428);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3108);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3618);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3982(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1652);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψημαι.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αρατεθειμαι.
pub(crate) fn n3635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3619);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3966);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word εκτεθειμαι.
pub(crate) fn n3636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3620);
	s.is_empty()
}

/// Words ending -ποκεκλειμαι.
pub(crate) fn n3637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3621);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3967);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ατακεκλιμαι.
pub(crate) fn n3638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3622);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 3968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κριμαι.
pub(crate) fn n334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 895);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2439);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2440);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3115);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3623);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 3116);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3624);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 3117);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3625);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3969);
													s.is_empty()
												}
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
							super::push(out, 2441);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3118);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πεπριμαι.
pub(crate) fn n1665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1661);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2442);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3119);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χριμαι.
pub(crate) fn n336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 897);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2443);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ετιμαι.
pub(crate) fn n337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 898);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -επιτεταλμαι.
pub(crate) fn n3644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3628);
	s.is_empty()
}

/// Words ending -συντεθαμμαι.
pub(crate) fn n3645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3629);
	s.is_empty()
}

/// Words ending -ανακεκαμμαι.
pub(crate) fn n3646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3630);
	s.is_empty()
}

/// Words ending -ραμμαι.
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 913);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1675);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3126);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3127);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3631);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3970);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 3632);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3971);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4142(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 3633);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3972);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n4143(s, out),
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
									super::push(out, 3128);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3634);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 3635);
											match s {
												[] => true,
												['ξ', '\u{1}', s @ ..] => {
													super::push(out, 3973);
													s.is_empty()
												}
												['σ', '\u{1}', s @ ..] => {
													super::push(out, 3974);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 3129);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3636);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3975);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 3637);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3976);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4144(s, out),
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
									super::push(out, 3130);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3638);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3977);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 3639);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3978);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 3131);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3640);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3979);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3641);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3980);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4145(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 914);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1676);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2459);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3132);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3642);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 915);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1677);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2460);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3133);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3643);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 916);
			match s {
				[] => true,
				['ε', s @ ..] => super::t1::n1682(s, out),
				['σ', s @ ..] => {
					super::push(out, 1679);
					match s {
						[] => true,
						['ε', s @ ..] => super::t1::n2471(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κεμμαι.
pub(crate) fn n350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 347);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 917);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1680);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2463);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3143);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3654);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3988);
													s.is_empty()
												}
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
							super::push(out, 2464);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3144);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3655);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λεμμαι.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 348);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1681);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1682);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2466);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3145);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3656);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3989);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πεμμαι.
pub(crate) fn n352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 349);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2468);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3146);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3657);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3990);
													s.is_empty()
												}
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
							super::push(out, 2469);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3147);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2470);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3148);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3658);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3991);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n4148(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -στεμμαι.
pub(crate) fn n925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 922);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2472);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ημμαι.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 923);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1687);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2473);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3150);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3659);
													s.is_empty()
												}
												['π', s @ ..] => {
													super::push(out, 3660);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4009(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 3151);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3661);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2474);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3152);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3662);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2475);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3153);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3663);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ε', s @ ..] => {
											super::push(out, 3154);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 3155);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3664);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 2476);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3156);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3665);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4010(s, out),
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
									super::push(out, 2477);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3157);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3666);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 355);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 926);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 927);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1688);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2478);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3158);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 356);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 929);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιμμαι.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1690);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2479);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3159);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 3160);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3667);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n4011(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 3161);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3668);
													s.is_empty()
												}
												_ => false,
											}
										}
										['λ', s @ ..] => {
											super::push(out, 3162);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3669);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3163);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3670);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n4012(s, out),
														['υ', s @ ..] => super::t1::n4013(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 3671);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4014(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 359);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1691);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2480);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1692);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2481);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3164);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3672);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n4015(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3165);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3673);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n4016(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 360);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1694);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1695);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2483);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3166);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2484);
									match s {
										[] => true,
										['α', s @ ..] => {
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
				['τ', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1696);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λμμαι.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1697);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2486);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3168);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3674);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4017(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ομμαι.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1698);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2487);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2488);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3169);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3675);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμμαι.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1699);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2490);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3170);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3676);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4018(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 940);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1700);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2491);
									s.is_empty()
								}
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
			super::push(out, 365);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1701);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2492);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3171);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 366);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1702);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1703);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2494);
									s.is_empty()
								}
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
			super::push(out, 367);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μεταδεδομαι.
pub(crate) fn n3693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3677);
	s.is_empty()
}

/// Words ending -αντιδεδομαι.
pub(crate) fn n3694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3678);
	s.is_empty()
}

/// Words ending -ταποδεδομαι.
pub(crate) fn n3695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3679);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 4002);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κλελυτρομαι.
pub(crate) fn n3696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3680);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ασμαι.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 965);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1725);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2507);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 3179);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3681);
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 3682);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 3683);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4021(s, out),
														_ => false,
													}
												}
												['κ', s @ ..] => {
													super::push(out, 3684);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 388);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 966);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1726);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2508);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3180);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3685);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 967);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1727);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2509);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3181);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 3182);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3686);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n4022(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 968);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1728);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2510);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3183);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3687);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n4023(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 389);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 969);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1730);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2512);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3184);
											s.is_empty()
										}
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
					super::push(out, 970);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1731);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2513);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3185);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3688);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n4024(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 971);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1732);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2514);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3186);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3689);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 390);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1733);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n394(s, out),
		['κ', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 984);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1748);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2531);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3199);
											s.is_empty()
										}
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
					super::push(out, 985);
					match s {
						[] => true,
						['κ', s @ ..] => super::t1::n1753(s, out),
						_ => false,
					}
				}
				['ι', s @ ..] => super::t1::n989(s, out),
				['ρ', s @ ..] => super::t1::n990(s, out),
				['υ', s @ ..] => super::t1::n991(s, out),
				_ => false,
			}
		}
		['λ', s @ ..] => super::t1::n396(s, out),
		['μ', s @ ..] => super::t1::n397(s, out),
		['ν', s @ ..] => super::t1::n398(s, out),
		['ξ', s @ ..] => super::t1::n399(s, out),
		['ο', s @ ..] => super::t1::n400(s, out),
		['π', s @ ..] => super::t1::n401(s, out),
		['ρ', s @ ..] => super::t1::n402(s, out),
		['τ', s @ ..] => super::t1::n403(s, out),
		['υ', s @ ..] => super::t1::n404(s, out),
		['φ', s @ ..] => super::t1::n405(s, out),
		['χ', s @ ..] => super::t1::n406(s, out),
		['ω', s @ ..] => super::t1::n407(s, out),
		_ => false,
	}
}

/// The word τετελεσμαι.
pub(crate) fn n3749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3733);
	s.is_empty()
}

/// Words ending -ατετελεσμαι.
pub(crate) fn n3750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3734);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 4023);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιτετελεσμαι.
pub(crate) fn n3751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3735);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4024);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4146);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οτετελεσμαι.
pub(crate) fn n3752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3736);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4025);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4147);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4148);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ναπεπλησμαι.
pub(crate) fn n3753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3737);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εμπεπλησμαι.
pub(crate) fn n3754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3738);
	s.is_empty()
}

/// Words ending -ενησμαι.
pub(crate) fn n1048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1045);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1825);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρησμαι.
pub(crate) fn n418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1826);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2602);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3252);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 3253);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3739);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σησμαι.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 416);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1827);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ισμαι.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1048);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1828);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 2603);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3254);
											s.is_empty()
										}
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
					super::push(out, 1049);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1829);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1050);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1830);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2605);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3255);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3740);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1051);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1831);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2606);
									s.is_empty()
								}
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
			super::push(out, 418);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1052);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1832);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2607);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3256);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3741);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 419);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1053);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1054);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1834);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2608);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3257);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3742);
													match s {
														[] => true,
														['ι', s @ ..] => super::t1::n4044(s, out),
														_ => false,
													}
												}
												['κ', s @ ..] => {
													super::push(out, 3743);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4045(s, out),
														_ => false,
													}
												}
												['λ', s @ ..] => {
													super::push(out, 3744);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n4046(s, out),
														_ => false,
													}
												}
												['π', s @ ..] => {
													super::push(out, 3745);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n4047(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1055);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1836);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2609);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3258);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3746);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1057);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1837);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2610);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3259);
											s.is_empty()
										}
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
					super::push(out, 1058);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1838);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2611);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3260);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3747);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n4048(s, out),
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
									super::push(out, 2612);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3261);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3748);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4049(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t1::n424(s, out),
		['θ', s @ ..] => super::t1::n425(s, out),
		['κ', s @ ..] => super::t1::n426(s, out),
		['λ', s @ ..] => super::t1::n427(s, out),
		['μ', s @ ..] => super::t1::n428(s, out),
		['ν', s @ ..] => super::t1::n429(s, out),
		['ο', s @ ..] => super::t1::n430(s, out),
		['π', s @ ..] => super::t1::n431(s, out),
		['ρ', s @ ..] => super::t1::n432(s, out),
		['τ', s @ ..] => super::t1::n433(s, out),
		['φ', s @ ..] => super::t1::n434(s, out),
		['χ', s @ ..] => super::t1::n435(s, out),
		['ω', s @ ..] => super::t1::n436(s, out),
		_ => false,
	}
}

/// Words ending -οσμαι.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πσμαι.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1123);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1931);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2702);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3337);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3801);
													match s {
														[] => true,
														['ι', s @ ..] => super::t1::n4082(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

