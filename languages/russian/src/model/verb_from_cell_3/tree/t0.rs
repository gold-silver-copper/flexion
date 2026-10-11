use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ь', s @ ..] => super::t0::n1(s, out),
		['я', s @ ..] => super::t0::n2(s, out),
		_ => false,
	}
}

/// Words ending -ь.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => super::t0::n4(s, out),
		_ => false,
	}
}

/// Words ending -я.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['с', s @ ..] => super::t0::n5(s, out),
		_ => false,
	}
}

/// Words ending -шь.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 64);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 65);
							match s {
								[] => true,
								['р', 'п', 'ы', s @ ..] => {
									super::push(out, 531);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 2704);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 66);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 67);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 532);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t0::n8(s, out),
		['и', s @ ..] => super::t0::n9(s, out),
		['ы', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 61);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2649);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2650);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 62);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2651);
									s.is_empty()
								}
								['о', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ся.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['ь', s @ ..] => super::t0::n11(s, out),
		_ => false,
	}
}

/// Words ending -ешь.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14);
			s.is_empty()
		}
		['а', s @ ..] => super::t0::n15(s, out),
		['б', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 88);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 758);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 759);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 760);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 3397);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9274);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 9275);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 9276);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 9277);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 9278);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 9279);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 16633);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 3398);
									match s {
										[] => true,
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 16634);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 16635);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 16636);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 16637);
													match s {
														[] => true,
														['в', s @ ..] => super::t0::n24817(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 761);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 762);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 763);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 3399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 89);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3400);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3401);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 3402);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 3403);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 9280);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 90);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 764);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 3404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 91);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 765);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 93);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 768);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3405);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 3406);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3407);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 3408);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9281);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 3409);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 9282);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 769);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 3410);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 9283);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 94);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3411);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 3412);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 3413);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 3414);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 9284);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 95);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 770);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 771);
							s.is_empty()
						}
						['ы', s @ ..] => super::t0::n780(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t0::n96(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t0::n18(s, out),
		['д', s @ ..] => super::t0::n19(s, out),
		['е', s @ ..] => super::t0::n20(s, out),
		['ж', s @ ..] => super::t0::n21(s, out),
		['з', s @ ..] => super::t0::n22(s, out),
		['и', s @ ..] => super::t0::n23(s, out),
		['к', s @ ..] => super::t0::n24(s, out),
		['л', s @ ..] => super::t0::n25(s, out),
		['м', s @ ..] => super::t0::n26(s, out),
		['н', s @ ..] => super::t0::n27(s, out),
		['о', s @ ..] => super::t0::n28(s, out),
		['р', s @ ..] => super::t0::n29(s, out),
		['с', s @ ..] => super::t0::n30(s, out),
		['т', s @ ..] => super::t0::n31(s, out),
		['у', s @ ..] => super::t0::n32(s, out),
		['ч', s @ ..] => super::t0::n33(s, out),
		['ш', s @ ..] => super::t0::n34(s, out),
		['щ', s @ ..] => super::t0::n35(s, out),
		['ъ', s @ ..] => super::t0::n36(s, out),
		['ы', s @ ..] => super::t0::n37(s, out),
		['ь', s @ ..] => super::t0::n38(s, out),
		['ю', s @ ..] => super::t0::n39(s, out),
		['я', s @ ..] => super::t0::n40(s, out),
		_ => false,
	}
}

/// Words ending -ишь.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 270);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 271);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1578);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 272);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1579);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4866);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 4867);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4868);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 4869);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 10977);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 273);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1580);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 1581);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1582);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t0::n42(s, out),
		['в', s @ ..] => super::t0::n43(s, out),
		['д', s @ ..] => super::t0::n44(s, out),
		['е', s @ ..] => {
			super::push(out, 45);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 309);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1737);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 5277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 310);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1738);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 5278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 311);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 1739);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5279);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5280);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 5281);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 11412);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 1740);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 5282);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 11413);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 5283);
									match s {
										[] => true,
										['о', 'л', 'с', 'а', s @ ..] => {
											super::push(out, 11414);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 312);
					match s {
						[] => true,
						['о', 'р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5284);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 5285);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 313);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t0::n46(s, out),
		['з', s @ ..] => super::t0::n47(s, out),
		['л', s @ ..] => super::t0::n48(s, out),
		['м', s @ ..] => super::t0::n49(s, out),
		['н', s @ ..] => super::t0::n50(s, out),
		['о', s @ ..] => {
			super::push(out, 51);
			match s {
				[] => true,
				['б', s @ ..] => super::t0::n405(s, out),
				['в', s @ ..] => super::t0::n406(s, out),
				['г', s @ ..] => super::t0::n407(s, out),
				['д', s @ ..] => super::t0::n408(s, out),
				['к', s @ ..] => super::t0::n409(s, out),
				['л', s @ ..] => super::t0::n410(s, out),
				['н', s @ ..] => super::t0::n411(s, out),
				['п', s @ ..] => super::t0::n412(s, out),
				['р', s @ ..] => super::t0::n413(s, out),
				['т', s @ ..] => super::t0::n414(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t0::n52(s, out),
		['р', s @ ..] => super::t0::n53(s, out),
		['с', s @ ..] => super::t0::n54(s, out),
		['т', s @ ..] => super::t0::n55(s, out),
		['у', s @ ..] => super::t0::n56(s, out),
		['ф', s @ ..] => super::t0::n57(s, out),
		['ч', s @ ..] => super::t1::n58(s, out),
		['ш', s @ ..] => super::t1::n59(s, out),
		['щ', s @ ..] => super::t1::n60(s, out),
		_ => false,
	}
}

/// Words ending -ься.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['ш', s @ ..] => super::t1::n63(s, out),
		_ => false,
	}
}

/// Words ending -аешь.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2705);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 2706);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 7763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2707);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 7764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2708);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 7765);
									s.is_empty()
								}
								['х', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15042);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 15043);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 15044);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 15045);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 15046);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 22347);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 2709);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 7766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2710);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 15047);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 15048);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 7767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 2711);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 15049);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 22348);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 15050);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 22349);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 536);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2712);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 7768);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 15051);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 2713);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 7769);
									s.is_empty()
								}
								['о', 'п', s @ ..] => {
									super::push(out, 7770);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15052);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 7771);
									match s {
										[] => true,
										['а', 'н', s @ ..] => {
											super::push(out, 15053);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 15054);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 22350);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 7772);
									match s {
										[] => true,
										['о', 'о', s @ ..] => {
											super::push(out, 15055);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 22351);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', 'в', 'о', s @ ..] => {
									super::push(out, 7773);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 15056);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 537);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2714);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									super::push(out, 7774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 538);
					match s {
						[] => true,
						['д', s @ ..] => super::t1::n2762(s, out),
						['з', s @ ..] => super::t1::n2763(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t1::n542(s, out),
				['у', s @ ..] => super::t1::n543(s, out),
				['ы', s @ ..] => super::t1::n544(s, out),
				['ю', s @ ..] => super::t1::n545(s, out),
				['я', s @ ..] => super::t1::n546(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t1::n69(s, out),
		['г', s @ ..] => super::t1::n70(s, out),
		['д', s @ ..] => super::t1::n71(s, out),
		['ж', s @ ..] => super::t1::n72(s, out),
		['з', s @ ..] => super::t1::n73(s, out),
		['к', s @ ..] => super::t1::n74(s, out),
		['л', s @ ..] => super::t1::n75(s, out),
		['м', s @ ..] => super::t1::n76(s, out),
		['н', s @ ..] => super::t1::n77(s, out),
		['п', s @ ..] => super::t1::n78(s, out),
		['р', s @ ..] => super::t1::n79(s, out),
		['с', s @ ..] => super::t1::n80(s, out),
		['т', s @ ..] => super::t1::n81(s, out),
		['ф', s @ ..] => {
			super::push(out, 82);
			s.is_empty()
		}
		['х', s @ ..] => super::t1::n83(s, out),
		['ц', s @ ..] => super::t1::n84(s, out),
		['ч', s @ ..] => super::t1::n85(s, out),
		['ш', s @ ..] => super::t1::n86(s, out),
		['щ', s @ ..] => super::t1::n87(s, out),
		_ => false,
	}
}

/// Words ending -выскребешь.
pub(crate) fn n24817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23461);
	s.is_empty()
}

/// Words ending -ырвешь.
pub(crate) fn n780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 772);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3415);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывешь.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 3416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9285);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9286);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9287);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9288);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9289);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16638);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3417);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9290);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гешь.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3418);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дешь.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9292);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 9293);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9294);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 9295);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 16639);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 9296);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 16640);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3420);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3421);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9297);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9298);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 9299);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9300);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 9301);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 9302);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 9303);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 16641);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 3422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3423);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3424);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3425);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3426);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3427);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 781);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3430);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9305);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 9306);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9307);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 9308);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 16642);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 9309);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 9310);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 9311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3431);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 9312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 3432);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 9313);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 16643);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 9314);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 16644);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 9315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 3433);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3434);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 9316);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3436);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9317);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9318);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 16645);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3437);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 9319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t1::n101(s, out),
		['й', s @ ..] => super::t1::n102(s, out),
		['у', s @ ..] => super::t1::n103(s, out),
		['ы', s @ ..] => super::t1::n104(s, out),
		['ю', s @ ..] => super::t1::n105(s, out),
		['я', s @ ..] => super::t1::n106(s, out),
		_ => false,
	}
}

/// Words ending -еешь.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['л', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9331);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3454);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 9333);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 16647);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9334);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							super::push(out, 3456);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3457);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 9336);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 16648);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 3458);
							match s {
								[] => true,
								['о', 'г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16649);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 16650);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9337);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 808);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3459);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 9339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', 'р', '\u{1}', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t1::n108(s, out),
		['г', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['и', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3506);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 824);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3508);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16722);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 16723);
											s.is_empty()
										}
										['о', 'с', s @ ..] => {
											super::push(out, 16724);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16725);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 16726);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3509);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 9396);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 16727);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 9397);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 16728);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 3510);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 9398);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 16729);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 825);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9399);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t1::n3584(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				['и', 'ж', s @ ..] => super::t1::n839(s, out),
				['л', s @ ..] => super::t1::n840(s, out),
				['н', s @ ..] => super::t1::n841(s, out),
				['о', s @ ..] => super::t1::n842(s, out),
				['р', s @ ..] => super::t1::n843(s, out),
				['у', s @ ..] => super::t1::n844(s, out),
				['ь', s @ ..] => super::t1::n845(s, out),
				['ю', s @ ..] => super::t1::n846(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t1::n111(s, out),
		['з', s @ ..] => super::t1::n112(s, out),
		['л', s @ ..] => super::t1::n113(s, out),
		['м', s @ ..] => super::t1::n114(s, out),
		['н', s @ ..] => super::t1::n115(s, out),
		['п', s @ ..] => super::t1::n116(s, out),
		['р', s @ ..] => super::t1::n117(s, out),
		['с', s @ ..] => super::t1::n118(s, out),
		['т', s @ ..] => super::t1::n119(s, out),
		['у', s @ ..] => super::t1::n120(s, out),
		['х', s @ ..] => super::t1::n121(s, out),
		['ч', s @ ..] => super::t1::n122(s, out),
		['ш', s @ ..] => super::t1::n123(s, out),
		_ => false,
	}
}

/// Words ending -жешь.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3730);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3731);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3732);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3733);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 9648);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 16898);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 3734);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3735);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3736);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3737);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3738);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3739);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9650);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16899);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 927);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 3740);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3741);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3742);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3743);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 9652);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 16900);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 23554);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 23555);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 9653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 3744);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3745);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3746);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 9654);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16901);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 16902);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 9655);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 16903);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 23556);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 9656);
									match s {
										[] => true,
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 23557);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 23558);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 3747);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3748);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 9657);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3749);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3750);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3751);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3752);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 9658);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 9659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3753);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3754);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9660);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16904);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 16905);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 929);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 930);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 931);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 932);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3756);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16906);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 16907);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t1::n128(s, out),
		['л', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 939);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t1::n130(s, out),
		['р', s @ ..] => super::t1::n131(s, out),
		['я', s @ ..] => super::t1::n132(s, out),
		_ => false,
	}
}

/// Words ending -зешь.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3778);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3779);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3780);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3781);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3782);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3783);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3784);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 3785);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 3786);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3787);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3788);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3789);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3790);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 953);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9671);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 9672);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9673);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 9674);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 9675);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 16913);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 954);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 955);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9676);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9677);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 9678);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 16914);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иешь.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 957);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3793);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3794);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 9679);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 9680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 3795);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 9681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 9682);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 16915);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 9683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 3796);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3797);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3798);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 9684);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 9685);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 9686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3799);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 9687);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 16916);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 3800);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 9688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 3801);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9689);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 958);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3802);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3803);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 9690);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 962);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 3805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кешь.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 964);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 967);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лешь.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 968);
					match s {
						[] => true,
						['л', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9691);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 9692);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 969);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3808);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3809);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3810);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9695);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 9696);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9697);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 9698);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 9699);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 9700);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 16917);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3811);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3812);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 9701);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 3813);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9702);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3814);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 9703);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16918);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 16919);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 16920);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 16921);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 23560);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 3815);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 9704);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 16922);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 9705);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 9706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3816);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 9707);
									match s {
										[] => true,
										['г', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 23561);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 23562);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 975);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3817);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3818);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 9708);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16923);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 9709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 3819);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3820);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3821);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3822);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3823);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9710);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3824);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3825);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3826);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3827);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t1::n150(s, out),
		['с', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 980);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 3833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 982);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 983);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 984);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мешь.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 985);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 986);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 988);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 989);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 3836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 991);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 994);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 3838);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нешь.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 999);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 1001);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 1005);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1006);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3842);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 9724);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1007);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1008);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1010);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3843);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3844);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 9725);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 3845);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 9726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3846);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 9727);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3847);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3848);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 9728);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 16932);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 23564);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 16933);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 16934);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 16935);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 9729);
									s.is_empty()
								}
								['е', 'л', s @ ..] => {
									super::push(out, 9730);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 16936);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', 'л', s @ ..] => {
									super::push(out, 9731);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 9732);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 16937);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 9733);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 9734);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 9735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3849);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 9736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 3850);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 9737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 1011);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 3851);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 9738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 1012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1013);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3852);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16938);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 16939);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3853);
							match s {
								[] => true,
								['х', 'ы', s @ ..] => {
									super::push(out, 9739);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 16940);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 3854);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3855);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1015);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 3856);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9741);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 3857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1016);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3858);
							match s {
								[] => true,
								['к', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16941);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 16942);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 1018);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t1::n1046(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t1::n160(s, out),
		['г', s @ ..] => super::t1::n161(s, out),
		['д', s @ ..] => super::t1::n162(s, out),
		['е', s @ ..] => super::t1::n163(s, out),
		['ж', s @ ..] => super::t1::n164(s, out),
		['з', s @ ..] => super::t1::n165(s, out),
		['и', s @ ..] => super::t1::n166(s, out),
		['й', s @ ..] => super::t1::n167(s, out),
		['к', s @ ..] => super::t1::n168(s, out),
		['м', s @ ..] => super::t1::n169(s, out),
		['о', s @ ..] => super::t1::n170(s, out),
		['п', s @ ..] => super::t2::n171(s, out),
		['р', s @ ..] => super::t2::n172(s, out),
		['с', s @ ..] => super::t2::n173(s, out),
		['т', s @ ..] => super::t2::n174(s, out),
		['у', s @ ..] => super::t2::n175(s, out),
		['ф', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
		}
		['х', s @ ..] => super::t2::n177(s, out),
		['ц', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['а', s @ ..] => super::t2::n1171(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t2::n179(s, out),
		['ш', s @ ..] => super::t2::n180(s, out),
		['ы', s @ ..] => super::t2::n181(s, out),
		['ь', s @ ..] => super::t2::n182(s, out),
		['ю', s @ ..] => super::t2::n183(s, out),
		['я', s @ ..] => super::t2::n184(s, out),
		_ => false,
	}
}

/// Words ending -оешь.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 1157);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1159);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1165);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1166);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1168);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 4138);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 1172);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4139);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4140);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4141);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4142);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4143);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1173);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4144);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1174);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1175);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -решь.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1176);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4146);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4147);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4148);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4149);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4150);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4151);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1179);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4152);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4153);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4154);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4155);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4156);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1181);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1186);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1187);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1188);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 4160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 1191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4161);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1193);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1194);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4164);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 4165);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4167);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4168);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4169);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4170);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4171);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1196);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1197);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1199);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1201);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1202);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4173);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1207);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сешь.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4175);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4176);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4177);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4178);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4179);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4180);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4181);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4182);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4183);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4184);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4185);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4186);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4187);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4188);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4189);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4190);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4191);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4192);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4193);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4194);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тешь.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['ц', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10054);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10055);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10056);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 10057);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 10058);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 10059);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17129);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10060);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10061);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 10062);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 10063);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17130);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4195);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4196);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4197);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4198);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4199);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10065);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10066);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 10067);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17131);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4200);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4201);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4202);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4203);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4204);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10068);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1215);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уешь.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1218);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4206);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 10069);
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17132);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 17133);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 17134);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 17135);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 17136);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 23608);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1219);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4207);
							match s {
								[] => true,
								['р', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17137);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 17138);
											s.is_empty()
										}
										['в', s @ ..] => {
											super::push(out, 17139);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 17140);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 17141);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 23609);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1220);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4208);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 10070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 4209);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 10071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 4210);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 10072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4211);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10073);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17142);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 17143);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 23610);
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 23611);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 17144);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1221);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4212);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17145);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 17146);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 1222);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4213);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10074);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 10075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t2::n210(s, out),
		['г', s @ ..] => super::t2::n211(s, out),
		['д', s @ ..] => super::t2::n212(s, out),
		['ж', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1251);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1252);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['в', s @ ..] => super::t2::n4411(s, out),
						['м', s @ ..] => {
							super::push(out, 4266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t2::n10688(s, out),
								['а', s @ ..] => super::t2::n10689(s, out),
								['з', s @ ..] => super::t2::n10690(s, out),
								['ы', s @ ..] => super::t2::n10691(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t2::n4413(s, out),
						_ => false,
					}
				}
				['ж', s @ ..] => super::t2::n1291(s, out),
				['з', s @ ..] => super::t2::n1292(s, out),
				['м', s @ ..] => super::t2::n1293(s, out),
				['н', s @ ..] => super::t2::n1294(s, out),
				['о', s @ ..] => super::t2::n1295(s, out),
				['р', s @ ..] => super::t2::n1296(s, out),
				['с', s @ ..] => super::t2::n1297(s, out),
				['т', s @ ..] => super::t2::n1298(s, out),
				['у', s @ ..] => super::t2::n1299(s, out),
				['ы', s @ ..] => super::t2::n1300(s, out),
				['я', s @ ..] => super::t2::n1301(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t2::n214(s, out),
		['к', s @ ..] => super::t2::n215(s, out),
		['л', s @ ..] => super::t2::n216(s, out),
		['м', s @ ..] => super::t2::n217(s, out),
		['н', s @ ..] => super::t2::n218(s, out),
		['п', s @ ..] => super::t2::n219(s, out),
		['р', s @ ..] => super::t2::n220(s, out),
		['с', s @ ..] => super::t2::n221(s, out),
		['т', s @ ..] => super::t2::n222(s, out),
		['ф', s @ ..] => super::t2::n223(s, out),
		['х', s @ ..] => super::t2::n224(s, out),
		['ц', s @ ..] => super::t2::n225(s, out),
		['ч', s @ ..] => super::t2::n226(s, out),
		['ш', s @ ..] => super::t2::n227(s, out),
		['щ', s @ ..] => super::t2::n228(s, out),
		_ => false,
	}
}

/// Words ending -чешь.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 4578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10680);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10681);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 17873);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 17874);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24350);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 10682);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10683);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 10684);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1413);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 4579);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10685);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10686);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10687);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 10688);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 10689);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17876);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4580);
							match s {
								[] => true,
								['щ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17877);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 17878);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 17879);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10690);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10691);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 17880);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 10692);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4581);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4582);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4583);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4584);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4585);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4586);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4587);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 10695);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17881);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 17882);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 10696);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 17883);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4588);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4589);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4590);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4591);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4592);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4593);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4594);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4595);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4596);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4597);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4598);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4599);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4600);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 10699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4601);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 10700);
									s.is_empty()
								}
								['о', s @ ..] => super::t2::n11299(s, out),
								_ => false,
							}
						}
						['ы', s @ ..] => super::t2::n4789(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t2::n231(s, out),
		['к', s @ ..] => super::t2::n232(s, out),
		['л', s @ ..] => super::t2::n233(s, out),
		['о', s @ ..] => super::t2::n234(s, out),
		['п', s @ ..] => super::t2::n235(s, out),
		['х', s @ ..] => super::t2::n236(s, out),
		['ы', s @ ..] => super::t2::n237(s, out),
		['я', s @ ..] => super::t2::n238(s, out),
		_ => false,
	}
}

/// Words ending -шешь.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4625);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4627);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4628);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4629);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4630);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4631);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1443);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10736);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10737);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4632);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4633);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4634);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4635);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4636);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10740);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4637);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4638);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4639);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10741);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4640);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4641);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4642);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4643);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4644);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4645);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1446);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1447);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1448);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10743);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10744);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10745);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 10746);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 10747);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17919);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 4648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щешь.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['е', 'л', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10748);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['е', 'р', 'к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10750);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10751);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 4649);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10753);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10754);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 17920);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 10755);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10756);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 17921);
											match s {
												[] => true,
												['у', 'р', 'а', s @ ..] => {
													super::push(out, 24357);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 17922);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 10757);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 10758);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 10759);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17923);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 4650);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10760);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10761);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 10762);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1453);
					match s {
						[] => true,
						['е', 'р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10763);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10764);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10765);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 10766);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17924);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 17925);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1454);
					s.is_empty()
				}
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4651);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4652);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4653);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4654);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1455);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 4655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1456);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10768);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 1457);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1458);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 4657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1459);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10770);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10771);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10772);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 10773);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17926);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['о', 'р', '\u{1}', s @ ..] => {
					super::push(out, 1460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 1461);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4658);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 4659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъешь.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 248);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыешь.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьешь.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1463);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1464);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1465);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1466);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1467);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4663);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17927);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 17928);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 4664);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10774);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10775);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 10776);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10777);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 17929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 4666);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 4667);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4668);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4669);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1468);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1469);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1470);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 1473);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4674);
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							super::push(out, 4675);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4676);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 4677);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1476);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1477);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1478);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1479);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1480);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1481);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4681);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 4682);
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10780);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 10781);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 17930);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 10782);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 4683);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 4684);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4685);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4686);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1483);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1484);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 1485);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 4689);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 10784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 1486);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1487);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 1489);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4692);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 10785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t2::n1533(s, out),
				['у', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				['ы', s @ ..] => super::t2::n1535(s, out),
				_ => false,
			}
		}
		['ш', s @ ..] => super::t2::n255(s, out),
		_ => false,
	}
}

/// Words ending -юешь.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1502);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4713);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 4714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10801);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 10802);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17933);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 4715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 1503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4716);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4717);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4718);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4719);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1504);
					match s {
						[] => true,
						['б', 'а', 'т', 'ш', 'а', s @ ..] => {
							super::push(out, 4720);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 4721);
							match s {
								[] => true,
								['м', 'е', 'т', 'ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17934);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 17935);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 17936);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'т', 'с', s @ ..] => {
							super::push(out, 4722);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 4723);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 4724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1505);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4725);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4726);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 10804);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17937);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 1506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4727);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4728);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 10805);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 10806);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 10807);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 17938);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24358);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 4729);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 4730);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4731);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10808);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 10809);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 4732);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 10810);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17939);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 17940);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4733);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4734);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4735);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4736);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10811);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 4737);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1507);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4738);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 10813);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 4739);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 10814);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 17941);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4740);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4741);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4742);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4743);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4744);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1509);
					match s {
						[] => true,
						['а', s @ ..] => super::t2::n4947(s, out),
						['т', s @ ..] => {
							super::push(out, 4746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t2::n1553(s, out),
				['х', s @ ..] => super::t2::n1554(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t2::n258(s, out),
		['о', s @ ..] => super::t2::n259(s, out),
		['р', s @ ..] => super::t2::n260(s, out),
		['т', s @ ..] => super::t2::n261(s, out),
		_ => false,
	}
}

/// Words ending -яешь.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4760);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4762);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4763);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4764);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4765);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10831);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 17949);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 17950);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1524);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4767);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4768);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4769);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t2::n264(s, out),
		['н', s @ ..] => {
			super::push(out, 264);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1544);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4818);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 10897);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 18004);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t2::n1588(s, out),
				['д', s @ ..] => {
					super::push(out, 1546);
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							super::push(out, 4821);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 10906);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 18017);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 10907);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t2::n1590(s, out),
				['ж', s @ ..] => super::t2::n1591(s, out),
				['з', s @ ..] => {
					super::push(out, 1549);
					s.is_empty()
				}
				['и', s @ ..] => super::t2::n1593(s, out),
				['л', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				['м', s @ ..] => super::t2::n1595(s, out),
				['о', s @ ..] => super::t2::n1596(s, out),
				['р', s @ ..] => super::t2::n1597(s, out),
				['с', s @ ..] => super::t2::n1598(s, out),
				['т', s @ ..] => super::t2::n1599(s, out),
				['у', s @ ..] => super::t2::n1600(s, out),
				['ч', s @ ..] => super::t2::n1601(s, out),
				['ы', s @ ..] => super::t2::n1602(s, out),
				['ь', 'л', 'о', 'в', 'у', 'о', s @ ..] => super::t2::n1603(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t2::n266(s, out),
		['р', s @ ..] => super::t2::n267(s, out),
		['с', s @ ..] => super::t2::n268(s, out),
		['у', s @ ..] => super::t2::n269(s, out),
		['ъ', s @ ..] => super::t2::n270(s, out),
		_ => false,
	}
}

/// Words ending -бишь.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4870);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4872);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4873);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4874);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4875);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4877);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4878);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10979);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1587);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10981);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10982);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 10983);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 1588);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10985);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10986);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18056);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 10987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4879);
							match s {
								[] => true,
								['о', 'в', 'з', s @ ..] => {
									super::push(out, 10988);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 18057);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4880);
							match s {
								[] => true,
								['а', 'ш', 'о', s @ ..] => {
									super::push(out, 10989);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 18058);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1590);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4882);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 10990);
									s.is_empty()
								}
								['ж', s @ ..] => {
									super::push(out, 10991);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18059);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 18060);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 10992);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18061);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 18062);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 18063);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 18064);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 24414);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10993);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 10994);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 18065);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 18066);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 10995);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 10996);
									s.is_empty()
								}
								['д', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 18067);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 24415);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 18068);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => super::t2::n11616(s, out),
								['о', s @ ..] => {
									super::push(out, 10997);
									s.is_empty()
								}
								['с', s @ ..] => super::t2::n11618(s, out),
								['ы', s @ ..] => super::t2::n11619(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t2::n5105(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t2::n278(s, out),
		['о', s @ ..] => super::t2::n279(s, out),
		['р', s @ ..] => super::t2::n280(s, out),
		['у', s @ ..] => super::t2::n281(s, out),
		['ы', s @ ..] => super::t2::n282(s, out),
		['ю', s @ ..] => super::t2::n283(s, out),
		['я', s @ ..] => super::t2::n284(s, out),
		_ => false,
	}
}

/// Words ending -вишь.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1613);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4958);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18104);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 18105);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 11068);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 4959);
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 11069);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 11070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 4960);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 11071);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 11072);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 18106);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'о', s @ ..] => {
							super::push(out, 4961);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 11073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4962);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 4963);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 11074);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 18107);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1614);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4964);
							match s {
								[] => true,
								['р', 'к', '\u{1}', s @ ..] => {
									super::push(out, 11075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 1615);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4965);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 4966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4967);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4968);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4969);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4970);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4971);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4972);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 1617);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11077);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 11078);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 11079);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 18108);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t2::n1661(s, out),
				['л', s @ ..] => super::t2::n1662(s, out),
				['р', s @ ..] => super::t2::n1663(s, out),
				['с', s @ ..] => super::t2::n1664(s, out),
				['т', s @ ..] => super::t2::n1665(s, out),
				['ч', s @ ..] => {
					super::push(out, 1623);
					match s {
						[] => true,
						['р', 'у', 'к', s @ ..] => super::t2::n5211(s, out),
						_ => false,
					}
				}
				['ш', s @ ..] => super::t2::n1667(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t2::n286(s, out),
		['з', s @ ..] => super::t2::n287(s, out),
		['и', s @ ..] => super::t2::n288(s, out),
		['л', s @ ..] => super::t2::n289(s, out),
		['о', s @ ..] => super::t2::n290(s, out),
		['р', s @ ..] => super::t2::n291(s, out),
		['т', s @ ..] => super::t2::n292(s, out),
		['у', s @ ..] => super::t2::n293(s, out),
		['я', s @ ..] => super::t2::n294(s, out),
		_ => false,
	}
}

/// Words ending -дишь.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5050);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5051);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 11176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5052);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 11177);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 11178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 5053);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 11179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5054);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5055);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 5056);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 5057);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5058);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 5059);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5060);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 11180);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 5061);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1661);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5062);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 11182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 1662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5063);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5064);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 5065);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5067);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5068);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 5069);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11183);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 11184);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 11185);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 11186);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 11187);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18179);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 18180);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 18181);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 24454);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 11188);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 11189);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 18182);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 11190);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 18183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 5070);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5071);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5072);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5073);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11191);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 11192);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 11193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 5074);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5075);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 5076);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11194);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 11195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1664);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5077);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 11196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t2::n1708(s, out),
				['с', s @ ..] => super::t2::n1709(s, out),
				['ч', s @ ..] => {
					super::push(out, 1667);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5091);
							s.is_empty()
						}
						['а', s @ ..] => super::t2::n5329(s, out),
						['о', s @ ..] => super::t2::n5330(s, out),
						_ => false,
					}
				}
				['ш', s @ ..] => super::t2::n1711(s, out),
				['щ', s @ ..] => super::t2::n1712(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t2::n296(s, out),
		['е', s @ ..] => super::t2::n297(s, out),
		['ж', s @ ..] => super::t2::n298(s, out),
		['з', s @ ..] => super::t2::n299(s, out),
		['и', s @ ..] => super::t2::n300(s, out),
		['й', s @ ..] => super::t2::n301(s, out),
		['л', s @ ..] => super::t2::n302(s, out),
		['н', s @ ..] => super::t2::n303(s, out),
		['о', s @ ..] => super::t2::n304(s, out),
		['р', s @ ..] => super::t2::n305(s, out),
		['у', s @ ..] => super::t3::n306(s, out),
		['ы', s @ ..] => super::t3::n307(s, out),
		['ю', s @ ..] => super::t3::n308(s, out),
		['я', s @ ..] => super::t3::n309(s, out),
		_ => false,
	}
}

/// Words ending -жишь.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1743);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 5287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5288);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11415);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 11416);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 11417);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 5289);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 11418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 5290);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 11419);
									match s {
										[] => true,
										['а', 'т', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24516);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 24517);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 5291);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 11420);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 5292);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 11421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5294);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 11422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 5295);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 11423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5296);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 11424);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 5297);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11425);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 18324);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 24518);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 11426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5298);
							match s {
								[] => true,
								['д', 'у', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18325);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 18326);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 5299);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 11427);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18327);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 18328);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 1748);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1749);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				['б', s @ ..] => super::t3::n1795(s, out),
				['в', s @ ..] => super::t3::n1796(s, out),
				['д', s @ ..] => {
					super::push(out, 1753);
					match s {
						[] => true,
						['а', s @ ..] => super::t3::n5550(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t3::n1798(s, out),
				['м', s @ ..] => super::t3::n1799(s, out),
				['н', s @ ..] => super::t3::n1800(s, out),
				['р', s @ ..] => super::t3::n1801(s, out),
				['ъ', s @ ..] => super::t3::n1802(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t3::n318(s, out),
		['з', s @ ..] => super::t3::n319(s, out),
		['и', s @ ..] => super::t3::n320(s, out),
		['л', s @ ..] => super::t3::n321(s, out),
		['м', s @ ..] => super::t3::n322(s, out),
		['н', s @ ..] => super::t3::n323(s, out),
		['о', s @ ..] => super::t3::n324(s, out),
		['р', s @ ..] => super::t3::n325(s, out),
		['у', s @ ..] => super::t3::n326(s, out),
		['ы', s @ ..] => super::t3::n327(s, out),
		['ю', s @ ..] => super::t3::n328(s, out),
		['я', s @ ..] => super::t3::n329(s, out),
		_ => false,
	}
}

/// Words ending -зишь.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1796);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 5396);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 11544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5397);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 11545);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5398);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5399);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18397);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 18398);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 18399);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 18400);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 5400);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 11546);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1798);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5401);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 5402);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 11547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 5403);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 5404);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11548);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 11549);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 18401);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 11550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 5405);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5406);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 5407);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 11551);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5408);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 11552);
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 11553);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 5409);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5410);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 11554);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18402);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 5411);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11555);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18403);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 18404);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 24542);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 5412);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11556);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5413);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5414);
							match s {
								[] => true,
								['з', 'з', s @ ..] => {
									super::push(out, 11557);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 18405);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => super::t3::n5665(s, out),
						['з', s @ ..] => super::t3::n5666(s, out),
						['о', s @ ..] => super::t3::n5667(s, out),
						['с', s @ ..] => super::t3::n5668(s, out),
						['т', s @ ..] => super::t3::n5669(s, out),
						['ы', s @ ..] => super::t3::n5670(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t3::n1846(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t3::n331(s, out),
		['е', s @ ..] => super::t3::n332(s, out),
		['и', s @ ..] => super::t3::n333(s, out),
		['й', s @ ..] => super::t3::n334(s, out),
		['м', s @ ..] => super::t3::n335(s, out),
		['н', s @ ..] => super::t3::n336(s, out),
		['о', s @ ..] => super::t3::n337(s, out),
		['р', s @ ..] => super::t3::n338(s, out),
		['у', s @ ..] => super::t3::n339(s, out),
		['ь', s @ ..] => super::t3::n340(s, out),
		['ю', s @ ..] => super::t3::n341(s, out),
		['я', s @ ..] => super::t3::n342(s, out),
		_ => false,
	}
}

/// Words ending -лишь.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1840);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5488);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 11630);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18455);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 18456);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 18457);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 11631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t3::n5743(s, out),
						['ы', s @ ..] => super::t3::n5744(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t3::n1886(s, out),
				['г', s @ ..] => super::t3::n1887(s, out),
				['д', s @ ..] => super::t3::n1888(s, out),
				['ж', s @ ..] => super::t3::n1889(s, out),
				['к', s @ ..] => super::t3::n1890(s, out),
				['м', s @ ..] => super::t3::n1891(s, out),
				['н', s @ ..] => super::t3::n1892(s, out),
				['п', s @ ..] => super::t3::n1893(s, out),
				['р', s @ ..] => super::t3::n1894(s, out),
				['с', s @ ..] => super::t3::n1895(s, out),
				['т', s @ ..] => super::t3::n1896(s, out),
				['ч', s @ ..] => super::t3::n1897(s, out),
				['ш', s @ ..] => super::t3::n1898(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t3::n344(s, out),
		['в', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['о', s @ ..] => super::t3::n1902(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t3::n346(s, out),
		['д', s @ ..] => super::t3::n347(s, out),
		['е', s @ ..] => super::t3::n348(s, out),
		['з', s @ ..] => super::t3::n349(s, out),
		['и', s @ ..] => super::t3::n350(s, out),
		['й', s @ ..] => super::t3::n351(s, out),
		['к', s @ ..] => super::t3::n352(s, out),
		['л', s @ ..] => super::t3::n353(s, out),
		['м', s @ ..] => super::t3::n354(s, out),
		['о', s @ ..] => super::t3::n355(s, out),
		['п', s @ ..] => super::t3::n356(s, out),
		['р', s @ ..] => super::t3::n357(s, out),
		['с', s @ ..] => super::t3::n358(s, out),
		['т', s @ ..] => super::t3::n359(s, out),
		['у', s @ ..] => super::t3::n360(s, out),
		['ф', s @ ..] => super::t3::n361(s, out),
		['х', s @ ..] => super::t3::n362(s, out),
		['ш', s @ ..] => super::t3::n363(s, out),
		['ы', s @ ..] => super::t3::n364(s, out),
		['ю', s @ ..] => super::t3::n365(s, out),
		['я', s @ ..] => super::t3::n366(s, out),
		_ => false,
	}
}

/// Words ending -мишь.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1963);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 1964);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 1965);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5807);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5808);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 11906);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 18595);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1966);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 5809);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 11907);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24598);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 24599);
													match s {
														[] => true,
														['р', s @ ..] => super::t3::n30955(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5810);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 11908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 5811);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11909);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 11910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1967);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 5812);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18596);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 18597);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 18598);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 24600);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 24601);
													match s {
														[] => true,
														['п', s @ ..] => super::t3::n30956(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1968);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 5813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1969);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5814);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 11911);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 5815);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11912);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 11913);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1970);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5816);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 11914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 1971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5817);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5818);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 5819);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 367);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1972);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 5821);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11915);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 11916);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 11917);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 5822);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18599);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 18600);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 1973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5823);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5824);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5825);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 11918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 5826);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 1974);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1975);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 1976);
					match s {
						[] => true,
						['с', 'о', s @ ..] => {
							super::push(out, 5828);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 11919);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1977);
					match s {
						[] => true,
						['к', s @ ..] => super::t3::n6092(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t3::n2023(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t3::n371(s, out),
		['о', s @ ..] => super::t3::n372(s, out),
		['р', s @ ..] => super::t3::n373(s, out),
		['т', s @ ..] => super::t3::n374(s, out),
		['у', s @ ..] => super::t3::n375(s, out),
		['х', s @ ..] => super::t3::n376(s, out),
		['ы', s @ ..] => super::t3::n377(s, out),
		['ь', s @ ..] => super::t3::n378(s, out),
		['ю', s @ ..] => super::t3::n379(s, out),
		['я', s @ ..] => super::t3::n380(s, out),
		_ => false,
	}
}

/// Words ending -нишь.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5880);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5881);
							match s {
								[] => true,
								['р', 'а', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24615);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 24616);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 24617);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', '\u{1}', s @ ..] => {
											super::push(out, 18637);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', '\u{1}', s @ ..] => {
									super::push(out, 11965);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 11966);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18638);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 5882);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 11967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5883);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5884);
							match s {
								[] => true,
								['е', 'д', '\u{1}', s @ ..] => {
									super::push(out, 11968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t3::n2050(s, out),
				['г', s @ ..] => super::t3::n2051(s, out),
				['д', s @ ..] => {
					super::push(out, 2007);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 5895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t3::n2053(s, out),
				['и', s @ ..] => super::t3::n2054(s, out),
				['к', s @ ..] => super::t3::n2055(s, out),
				['л', s @ ..] => super::t3::n2056(s, out),
				['м', s @ ..] => super::t3::n2057(s, out),
				['п', s @ ..] => super::t3::n2058(s, out),
				['р', s @ ..] => super::t3::n2059(s, out),
				['с', s @ ..] => super::t3::n2060(s, out),
				['т', s @ ..] => super::t3::n2061(s, out),
				['ф', s @ ..] => super::t3::n2062(s, out),
				['щ', s @ ..] => super::t3::n2063(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t3::n382(s, out),
		['в', s @ ..] => super::t3::n383(s, out),
		['г', s @ ..] => super::t3::n384(s, out),
		['д', s @ ..] => super::t3::n385(s, out),
		['е', s @ ..] => super::t3::n386(s, out),
		['ж', s @ ..] => super::t3::n387(s, out),
		['з', s @ ..] => super::t3::n388(s, out),
		['и', s @ ..] => super::t3::n389(s, out),
		['й', s @ ..] => super::t3::n390(s, out),
		['л', s @ ..] => super::t3::n391(s, out),
		['м', s @ ..] => super::t3::n392(s, out),
		['н', s @ ..] => super::t3::n393(s, out),
		['о', s @ ..] => super::t3::n394(s, out),
		['п', s @ ..] => super::t3::n395(s, out),
		['р', s @ ..] => super::t3::n396(s, out),
		['с', s @ ..] => super::t3::n397(s, out),
		['т', s @ ..] => super::t3::n398(s, out),
		['у', s @ ..] => super::t3::n399(s, out),
		['ч', s @ ..] => super::t3::n400(s, out),
		['ш', s @ ..] => super::t3::n401(s, out),
		['ы', s @ ..] => super::t3::n402(s, out),
		['ю', s @ ..] => super::t3::n403(s, out),
		['я', s @ ..] => super::t3::n404(s, out),
		_ => false,
	}
}

/// Words ending -боишь.
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 2127);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6140);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -воишь.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 405);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2128);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6141);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6142);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6143);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6144);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2129);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 2130);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гоишь.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 2131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -доишь.
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 407);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2132);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2133);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2134);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6147);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2135);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6149);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коишь.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2136);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12254);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24661);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24662);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29200);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 18839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лоишь.
pub(crate) fn n410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 2137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6150);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6151);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6152);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6153);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6154);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2138);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6156);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6157);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ноишь.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 2139);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6158);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6159);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6160);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поишь.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2140);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2141);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2142);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2143);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6163);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2144);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2145);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6164);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роишь.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2146);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2147);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2148);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2149);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6167);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6168);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6169);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6170);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6171);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2150);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2151);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6172);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6173);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6174);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12259);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12260);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 18840);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 18841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12261);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12262);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12263);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12264);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6175);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тоишь.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 2152);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6176);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6177);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6178);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6179);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6180);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6181);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12270);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 18843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12271);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 18844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12272);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12273);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6182);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6183);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12276);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6184);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6185);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12277);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пишь.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2153);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6186);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 12278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6187);
							s.is_empty()
						}
						['и', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12279);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 12280);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 6188);
							s.is_empty()
						}
						['э', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12281);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 12282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6189);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 6190);
							s.is_empty()
						}
						['о', 'с', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12283);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 12284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2156);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2157);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6191);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12285);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 12286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 6192);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 12287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 6193);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12288);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 12289);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 12290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2158);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2159);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6195);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6196);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 6197);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 6198);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 6199);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 6200);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6201);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 6202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12291);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 12292);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 12293);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 12294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 6203);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 12295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2160);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 2161);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6205);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6206);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 6207);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 12297);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 2162);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6208);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6209);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 12298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 6210);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 12299);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 6211);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 12300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 6212);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 12301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 6213);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 12302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6214);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6215);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6216);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 6217);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 6218);
							match s {
								[] => true,
								['в', s @ ..] => super::t3::n13015(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t3::n2210(s, out),
				['с', s @ ..] => super::t3::n2211(s, out),
				['ш', s @ ..] => super::t3::n2212(s, out),
				['щ', s @ ..] => super::t3::n2213(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t3::n418(s, out),
		['л', s @ ..] => super::t3::n419(s, out),
		['м', s @ ..] => super::t3::n420(s, out),
		['о', s @ ..] => super::t3::n421(s, out),
		['п', s @ ..] => super::t3::n422(s, out),
		['р', s @ ..] => super::t3::n423(s, out),
		['с', s @ ..] => super::t3::n424(s, out),
		['у', s @ ..] => super::t3::n425(s, out),
		['ы', s @ ..] => super::t3::n426(s, out),
		['я', s @ ..] => super::t3::n427(s, out),
		_ => false,
	}
}

/// Words ending -ришь.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2199);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6292);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 12357);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 18874);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 12358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6293);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6294);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 6295);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 6296);
							match s {
								[] => true,
								['е', s @ ..] => super::t3::n13073(s, out),
								['о', s @ ..] => super::t3::n13074(s, out),
								_ => false,
							}
						}
						['е', s @ ..] => super::t3::n6586(s, out),
						['з', s @ ..] => super::t3::n6587(s, out),
						['к', s @ ..] => super::t3::n6588(s, out),
						['о', s @ ..] => super::t3::n6589(s, out),
						['с', s @ ..] => super::t3::n6590(s, out),
						['т', s @ ..] => super::t3::n6591(s, out),
						['ы', s @ ..] => super::t3::n6592(s, out),
						_ => false,
					}
				}
				['г', s @ ..] => super::t3::n2247(s, out),
				['д', s @ ..] => super::t3::n2248(s, out),
				['ж', s @ ..] => super::t3::n2249(s, out),
				['з', s @ ..] => super::t3::n2250(s, out),
				['и', s @ ..] => super::t3::n2251(s, out),
				['к', s @ ..] => super::t3::n2252(s, out),
				['м', s @ ..] => super::t3::n2253(s, out),
				['н', s @ ..] => super::t3::n2254(s, out),
				['п', s @ ..] => super::t3::n2255(s, out),
				['с', s @ ..] => super::t3::n2256(s, out),
				['т', s @ ..] => super::t3::n2257(s, out),
				['у', s @ ..] => super::t3::n2258(s, out),
				['ф', s @ ..] => super::t3::n2259(s, out),
				['х', s @ ..] => super::t3::n2260(s, out),
				['ц', s @ ..] => super::t3::n2261(s, out),
				['ч', s @ ..] => super::t3::n2262(s, out),
				['ш', s @ ..] => super::t3::n2263(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t3::n429(s, out),
		['в', s @ ..] => super::t3::n430(s, out),
		['г', s @ ..] => super::t3::n431(s, out),
		['д', s @ ..] => super::t3::n432(s, out),
		['е', s @ ..] => super::t3::n433(s, out),
		['з', s @ ..] => super::t3::n434(s, out),
		['и', s @ ..] => super::t3::n435(s, out),
		['к', s @ ..] => super::t3::n436(s, out),
		['о', s @ ..] => super::t3::n437(s, out),
		['т', s @ ..] => super::t4::n438(s, out),
		['у', s @ ..] => super::t4::n439(s, out),
		['ф', s @ ..] => super::t4::n440(s, out),
		['х', s @ ..] => super::t4::n441(s, out),
		['щ', s @ ..] => super::t4::n442(s, out),
		['ы', s @ ..] => super::t4::n443(s, out),
		['ю', s @ ..] => super::t4::n444(s, out),
		['я', s @ ..] => super::t4::n445(s, out),
		_ => false,
	}
}

/// Words ending -сишь.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 445);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6637);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6638);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 12692);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 6639);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 12693);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24730);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 24731);
													match s {
														[] => true,
														['з', s @ ..] => super::t4::n30991(s, out),
														['н', s @ ..] => super::t4::n30992(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 6640);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 12694);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 19049);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6641);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 12695);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 6642);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 12696);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19050);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2325);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6643);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12697);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 12698);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 12699);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 12700);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 12701);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 19051);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 12702);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 19052);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6644);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6645);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2327);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 6647);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 12703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2328);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6648);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 12704);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 6649);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12705);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 12706);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 12707);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 19053);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 24732);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2329);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6650);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 12708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2330);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6651);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 12709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 6652);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 12710);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['к', s @ ..] => super::t4::n6950(s, out),
						['о', s @ ..] => {
							super::push(out, 6654);
							match s {
								[] => true,
								['д', 'и', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t4::n20132(s, out),
										['о', s @ ..] => super::t4::n20133(s, out),
										['с', s @ ..] => super::t4::n20134(s, out),
										['т', s @ ..] => super::t4::n20135(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t4::n2378(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n447(s, out),
		['и', s @ ..] => super::t4::n448(s, out),
		['й', s @ ..] => super::t4::n449(s, out),
		['к', s @ ..] => super::t4::n450(s, out),
		['м', s @ ..] => super::t4::n451(s, out),
		['н', s @ ..] => super::t4::n452(s, out),
		['о', s @ ..] => super::t4::n453(s, out),
		['п', s @ ..] => super::t4::n454(s, out),
		['р', s @ ..] => super::t4::n455(s, out),
		['с', s @ ..] => super::t4::n456(s, out),
		['у', s @ ..] => super::t4::n457(s, out),
		['ы', s @ ..] => super::t4::n458(s, out),
		['э', s @ ..] => super::t4::n459(s, out),
		_ => false,
	}
}

/// Words ending -тишь.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2368);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 6747);
							match s {
								[] => true,
								['о', 'г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19120);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 19121);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2369);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6748);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 12810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6749);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 12811);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 19122);
											match s {
												[] => true,
												['в', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n31000(s, out),
														['б', s @ ..] => super::t4::n31001(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 6750);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 12812);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 19123);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6751);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6752);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 12813);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 12814);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6753);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 12815);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19124);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 19125);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 12816);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 6754);
							match s {
								[] => true,
								['в', s @ ..] => super::t4::n13554(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t4::n2417(s, out),
				['и', s @ ..] => super::t4::n2418(s, out),
				['к', s @ ..] => super::t4::n2419(s, out),
				['л', s @ ..] => super::t4::n2420(s, out),
				['м', s @ ..] => super::t4::n2421(s, out),
				['н', s @ ..] => super::t4::n2422(s, out),
				['о', s @ ..] => super::t4::n2423(s, out),
				['п', s @ ..] => super::t4::n2424(s, out),
				['р', s @ ..] => super::t4::n2425(s, out),
				['х', s @ ..] => super::t4::n2426(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t4::n461(s, out),
		['е', s @ ..] => super::t4::n462(s, out),
		['и', s @ ..] => super::t4::n463(s, out),
		['й', s @ ..] => super::t4::n464(s, out),
		['к', s @ ..] => super::t4::n465(s, out),
		['л', s @ ..] => super::t4::n466(s, out),
		['н', s @ ..] => super::t4::n467(s, out),
		['о', s @ ..] => super::t4::n468(s, out),
		['п', s @ ..] => super::t4::n469(s, out),
		['р', s @ ..] => super::t4::n470(s, out),
		['с', s @ ..] => super::t4::n471(s, out),
		['у', s @ ..] => super::t4::n472(s, out),
		['ф', s @ ..] => super::t4::n473(s, out),
		['х', s @ ..] => super::t4::n474(s, out),
		['ч', s @ ..] => super::t4::n475(s, out),
		['ы', s @ ..] => super::t4::n476(s, out),
		['ю', s @ ..] => super::t4::n477(s, out),
		['я', s @ ..] => super::t4::n478(s, out),
		_ => false,
	}
}

/// Words ending -уишь.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фишь.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2467);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 6958);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13094);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 13095);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 19306);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 24839);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 19307);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 13096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 6959);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13097);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13098);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 13099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 480);
			match s {
				[] => true,
				['л', 'о', 'о', s @ ..] => {
					super::push(out, 2468);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 6960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'а', s @ ..] => {
					super::push(out, 2469);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 6961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2470);
					match s {
						[] => true,
						['р', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13100);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 13101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2471);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13102);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 13103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2472);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 6962);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 6963);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13104);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13105);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 19308);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

