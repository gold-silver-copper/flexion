use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['क', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 24);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ख', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['\u{947}', s @ ..] => {
					super::push(out, 25);
					s.is_empty()
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 26);
					s.is_empty()
				}
				['\u{947}', s @ ..] => {
					super::push(out, 27);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 28);
					match s {
						[] => true,
						['व', s @ ..] => {
							super::push(out, 92);
							match s {
								[] => true,
								['उ', '\u{1}', s @ ..] => {
									super::push(out, 229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 29);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 93);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 30);
					match s {
						[] => true,
						['भ', s @ ..] => {
							super::push(out, 94);
							s.is_empty()
						}
						['य', s @ ..] => {
							super::push(out, 95);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ो', s @ ..] => {
					super::push(out, 31);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 32);
					match s {
						[] => true,
						['ञ', s @ ..] => {
							super::push(out, 96);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 97);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 33);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['ी', s @ ..] => {
					super::push(out, 34);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 35);
					s.is_empty()
				}
				['\u{943}', 'म', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 36);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 37);
					match s {
						[] => true,
						['त', s @ ..] => {
							super::push(out, 98);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 99);
							match s {
								[] => true,
								['आ', '\u{1}', s @ ..] => {
									super::push(out, 230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 8);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 38);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 39);
					match s {
						[] => true,
						['प', s @ ..] => {
							super::push(out, 100);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 101);
							s.is_empty()
						}
						['स', s @ ..] => {
							super::push(out, 102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 40);
					match s {
						[] => true,
						['छ', '\u{94d}', 'च', 'ि', 'च', '\u{1}', s @ ..] => {
							super::push(out, 103);
							s.is_empty()
						}
						['व', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 231);
									s.is_empty()
								}
								['ि', 'व', '\u{1}', s @ ..] => {
									super::push(out, 232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ो', s @ ..] => {
					super::push(out, 41);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 9);
			match s {
				[] => true,
				['ो', s @ ..] => {
					super::push(out, 42);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 43);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 104);
							s.is_empty()
						}
						['न', s @ ..] => {
							super::push(out, 105);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 44);
					match s {
						[] => true,
						['ख', s @ ..] => {
							super::push(out, 107);
							s.is_empty()
						}
						['घ', s @ ..] => {
							super::push(out, 108);
							s.is_empty()
						}
						['ज', s @ ..] => {
							super::push(out, 109);
							s.is_empty()
						}
						['त', s @ ..] => {
							super::push(out, 110);
							match s {
								[] => true,
								['त', '\u{1}', s @ ..] => {
									super::push(out, 233);
									s.is_empty()
								}
								_ => false,
							}
						}
						['स', s @ ..] => {
							super::push(out, 111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 11);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 45);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['त', s @ ..] => {
							super::push(out, 112);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'ल', 'क', 'च', '\u{1}', s @ ..] => {
					super::push(out, 47);
					s.is_empty()
				}
				_ => false,
			}
		}
		['म', s @ ..] => {
			super::push(out, 12);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 48);
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 114);
							s.is_empty()
						}
						['न', s @ ..] => {
							super::push(out, 115);
							s.is_empty()
						}
						['य', s @ ..] => {
							super::push(out, 116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 13);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['घ', s @ ..] => {
							super::push(out, 117);
							s.is_empty()
						}
						['य', s @ ..] => {
							super::push(out, 118);
							match s {
								[] => true,
								['इ', '\u{1}', s @ ..] => {
									super::push(out, 234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 50);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 51);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 119);
							s.is_empty()
						}
						['ग', s @ ..] => {
							super::push(out, 120);
							s.is_empty()
						}
						['च', s @ ..] => {
							super::push(out, 121);
							match s {
								[] => true,
								['च', '\u{1}', s @ ..] => {
									super::push(out, 235);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ध', s @ ..] => {
							super::push(out, 122);
							s.is_empty()
						}
						['भ', s @ ..] => {
							super::push(out, 123);
							s.is_empty()
						}
						['म', s @ ..] => {
							super::push(out, 124);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 15);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 52);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['ट', s @ ..] => {
							super::push(out, 126);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 127);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 128);
							match s {
								[] => true,
								['\u{941}', 'ज', '\u{1}', s @ ..] => {
									super::push(out, 236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 54);
					s.is_empty()
				}
				['\u{942}', s @ ..] => {
					super::push(out, 55);
					match s {
						[] => true,
						['भ', 'ब', '\u{1}', s @ ..] => {
							super::push(out, 129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['श', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['\u{902}', s @ ..] => {
					super::push(out, 56);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 57);
					match s {
						[] => true,
						['न', 'आ', '\u{1}', s @ ..] => {
							super::push(out, 130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 58);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 59);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['\u{947}', s @ ..] => {
					super::push(out, 60);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 61);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 62);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 131);
							s.is_empty()
						}
						['र', s @ ..] => super::t0::n133(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['स', s @ ..] => super::t0::n18(s, out),
		['ह', s @ ..] => super::t0::n19(s, out),
		['ि', s @ ..] => super::t0::n20(s, out),
		['\u{947}', s @ ..] => super::t0::n21(s, out),
		['ौ', s @ ..] => super::t0::n22(s, out),
		['\u{94d}', s @ ..] => super::t0::n23(s, out),
		_ => false,
	}
}

/// Words ending -र्ष.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['क', 'च', '\u{1}', s @ ..] => {
			super::push(out, 237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -स.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 63);
			s.is_empty()
		}
		['आ', s @ ..] => {
			super::push(out, 64);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['घ', s @ ..] => {
					super::push(out, 133);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 134);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ह.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		['आ', s @ ..] => {
			super::push(out, 67);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 135);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 137);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 139);
					match s {
						[] => true,
						['त', 'त', '\u{1}', s @ ..] => {
							super::push(out, 238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ि.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['ट', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 140);
					match s {
						[] => true,
						['ष', s @ ..] => {
							match s {
								[] => true,
								['न', s @ ..] => {
									super::push(out, 444);
									s.is_empty()
								}
								['व', s @ ..] => {
									super::push(out, 445);
									s.is_empty()
								}
								['ा', s @ ..] => {
									super::push(out, 446);
									s.is_empty()
								}
								['\u{947}', s @ ..] => {
									super::push(out, 447);
									match s {
										[] => true,
										['द', 'ि', 'द', '\u{1}', s @ ..] => {
											super::push(out, 703);
											s.is_empty()
										}
										['व', s @ ..] => {
											super::push(out, 704);
											match s {
												[] => true,
												['ि', s @ ..] => {
													super::push(out, 883);
													s.is_empty()
												}
												['\u{947}', s @ ..] => {
													super::push(out, 884);
													s.is_empty()
												}
												['\u{94d}', s @ ..] => {
													super::push(out, 885);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['\u{94d}', s @ ..] => {
									super::push(out, 448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ढ', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['\u{947}', s @ ..] => {
					super::push(out, 141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => super::t0::n73(s, out),
		['ध', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 178);
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 378);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 75);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -े.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 76);
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ञ', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 181);
					match s {
						[] => true,
						['ट', s @ ..] => {
							super::push(out, 380);
							s.is_empty()
						}
						['ष', s @ ..] => {
							super::push(out, 381);
							match s {
								[] => true,
								['ई', s @ ..] => {
									super::push(out, 640);
									s.is_empty()
								}
								['च', s @ ..] => {
									super::push(out, 641);
									match s {
										[] => true,
										['र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
											super::push(out, 845);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 182);
					s.is_empty()
				}
				['ज', s @ ..] => {
					super::push(out, 183);
					match s {
						[] => true,
						['ि', 'व', '\u{94d}', s @ ..] => {
							super::push(out, 382);
							match s {
								[] => true,
								['द', 'उ', '\u{1}', s @ ..] => {
									super::push(out, 642);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ड', s @ ..] => {
					super::push(out, 184);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ण', s @ ..] => {
					super::push(out, 185);
					s.is_empty()
				}
				['त', s @ ..] => {
					super::push(out, 186);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 384);
							match s {
								[] => true,
								['र', 'व', 'ि', s @ ..] => {
									super::push(out, 643);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['थ', s @ ..] => {
					super::push(out, 187);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 188);
					match s {
						[] => true,
						['\u{94d}', 'र', s @ ..] => {
							super::push(out, 385);
							match s {
								[] => true,
								['ऊ', s @ ..] => {
									super::push(out, 644);
									s.is_empty()
								}
								['प', s @ ..] => {
									super::push(out, 645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ध', s @ ..] => {
					super::push(out, 189);
					match s {
						[] => true,
						['ए', s @ ..] => {
							super::push(out, 386);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 387);
							match s {
								[] => true,
								['र', 'व', '\u{1}', s @ ..] => {
									super::push(out, 647);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 190);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['भ', s @ ..] => {
					super::push(out, 191);
					match s {
						[] => true,
						['\u{94d}', 'म', '\u{943}', s @ ..] => {
							super::push(out, 389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['म', s @ ..] => {
					super::push(out, 192);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 648);
									s.is_empty()
								}
								['\u{94d}', s @ ..] => {
									super::push(out, 649);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ष', s @ ..] => {
							super::push(out, 391);
							match s {
								[] => true,
								['\u{94d}', 'क', '\u{1}', s @ ..] => {
									super::push(out, 650);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['य', s @ ..] => super::t0::n194(s, out),
				['र', s @ ..] => {
					super::push(out, 194);
					match s {
						[] => true,
						['व', s @ ..] => {
							super::push(out, 404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 195);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 405);
							match s {
								[] => true,
								['\u{94d}', 'प', '\u{1}', s @ ..] => {
									super::push(out, 679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 196);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 197);
					match s {
						[] => true,
						['ा', s @ ..] => super::t0::n409(s, out),
						['\u{941}', s @ ..] => super::t0::n410(s, out),
						['\u{942}', s @ ..] => super::t0::n411(s, out),
						['\u{94d}', s @ ..] => super::t0::n412(s, out),
						_ => false,
					}
				}
				['स', s @ ..] => super::t0::n199(s, out),
				['ह', s @ ..] => super::t0::n200(s, out),
				['ी', s @ ..] => super::t0::n201(s, out),
				['\u{947}', s @ ..] => super::t0::n202(s, out),
				['\u{94d}', s @ ..] => super::t0::n203(s, out),
				_ => false,
			}
		}
		['ध', s @ ..] => super::t0::n80(s, out),
		['य', s @ ..] => super::t0::n81(s, out),
		['ळ', s @ ..] => super::t0::n82(s, out),
		['श', s @ ..] => super::t0::n83(s, out),
		_ => false,
	}
}

/// Words ending -ौ.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['द', s @ ..] => {
			super::push(out, 84);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 85);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['ो', s @ ..] => {
					super::push(out, 206);
					match s {
						[] => true,
						['म', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', 'र', s @ ..] => {
					match s {
						[] => true,
						['द', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 415);
							s.is_empty()
						}
						['व', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 87);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 88);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 207);
					s.is_empty()
				}
				['च', s @ ..] => {
					super::push(out, 208);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 209);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 210);
					s.is_empty()
				}
				['प', s @ ..] => {
					super::push(out, 211);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 212);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 213);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 417);
							match s {
								[] => true,
								['ष', 'ि', 'व', 'ी', 'ज', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 214);
					match s {
						[] => true,
						['स', s @ ..] => {
							super::push(out, 418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 215);
					match s {
						[] => true,
						['ि', s @ ..] => {
							super::push(out, 419);
							s.is_empty()
						}
						['ी', s @ ..] => {
							super::push(out, 420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 216);
					match s {
						[] => true,
						['ऐ', '\u{1}', s @ ..] => {
							super::push(out, 421);
							s.is_empty()
						}
						['\u{941}', 'ज', 'अ', '\u{1}', s @ ..] => {
							super::push(out, 422);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 217);
					match s {
						[] => true,
						['\u{941}', s @ ..] => {
							super::push(out, 424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 218);
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 425);
							s.is_empty()
						}
						['थ', s @ ..] => {
							super::push(out, 426);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 427);
							s.is_empty()
						}
						['ध', s @ ..] => {
							super::push(out, 428);
							s.is_empty()
						}
						['प', s @ ..] => {
							super::push(out, 429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 219);
					match s {
						[] => true,
						['ख', s @ ..] => {
							super::push(out, 430);
							s.is_empty()
						}
						['च', s @ ..] => {
							super::push(out, 431);
							s.is_empty()
						}
						['ज', s @ ..] => {
							super::push(out, 432);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 433);
							s.is_empty()
						}
						['न', s @ ..] => {
							super::push(out, 434);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 435);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 436);
							s.is_empty()
						}
						['ष', s @ ..] => {
							super::push(out, 437);
							match s {
								[] => true,
								['ऐ', s @ ..] => {
									super::push(out, 689);
									s.is_empty()
								}
								['ा', s @ ..] => {
									super::push(out, 690);
									s.is_empty()
								}
								['\u{948}', s @ ..] => {
									super::push(out, 691);
									match s {
										[] => true,
										['ज', s @ ..] => {
											super::push(out, 877);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ौ', s @ ..] => {
									super::push(out, 692);
									s.is_empty()
								}
								['\u{94d}', s @ ..] => {
									super::push(out, 693);
									match s {
										[] => true,
										['क', s @ ..] => {
											super::push(out, 878);
											match s {
												[] => true,
												['ा', s @ ..] => {
													super::push(out, 960);
													match s {
														[] => true,
														['ध', s @ ..] => super::t0::n992(s, out),
														['भ', s @ ..] => super::t0::n993(s, out),
														['य', s @ ..] => super::t0::n994(s, out),
														['र', s @ ..] => super::t0::n995(s, out),
														['व', s @ ..] => super::t0::n996(s, out),
														_ => false,
													}
												}
												['\u{94d}', s @ ..] => {
													super::push(out, 961);
													match s {
														[] => true,
														['ङ', s @ ..] => super::t0::n997(s, out),
														['र', s @ ..] => super::t0::n998(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['र', s @ ..] => super::t0::n892(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['स', s @ ..] => super::t0::n441(s, out),
						['ह', s @ ..] => super::t0::n442(s, out),
						_ => false,
					}
				}
				['\u{942}', s @ ..] => {
					super::push(out, 220);
					match s {
						[] => true,
						['भ', s @ ..] => super::t0::n443(s, out),
						_ => false,
					}
				}
				['\u{947}', s @ ..] => super::t0::n223(s, out),
				['ो', s @ ..] => super::t0::n224(s, out),
				['\u{94d}', s @ ..] => super::t0::n225(s, out),
				_ => false,
			}
		}
		['न', s @ ..] => super::t0::n89(s, out),
		['र', s @ ..] => super::t0::n90(s, out),
		['स', s @ ..] => super::t0::n91(s, out),
		_ => false,
	}
}

/// Words ending -ति.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['ए', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['क', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['\u{94d}', 'क', 'ि', s @ ..] => {
					super::push(out, 240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ख', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['ि', 'ल', '\u{94d}', s @ ..] => {
					super::push(out, 241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ग', s @ ..] => {
			super::push(out, 145);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 242);
					match s {
						[] => true,
						['ङ', s @ ..] => {
							super::push(out, 449);
							match s {
								[] => true,
								['इ', '\u{1}', s @ ..] => {
									super::push(out, 705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 243);
					match s {
						[] => true,
						['ञ', 'अ', s @ ..] => {
							super::push(out, 451);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['छ', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 244);
					match s {
						[] => true,
						['च', s @ ..] => {
							super::push(out, 453);
							match s {
								[] => true,
								['इ', s @ ..] => {
									super::push(out, 706);
									s.is_empty()
								}
								['उ', s @ ..] => {
									super::push(out, 707);
									s.is_empty()
								}
								['ऋ', s @ ..] => {
									super::push(out, 708);
									s.is_empty()
								}
								['ग', s @ ..] => {
									super::push(out, 709);
									match s {
										[] => true,
										['आ', s @ ..] => {
											super::push(out, 886);
											s.is_empty()
										}
										['व', s @ ..] => {
											super::push(out, 887);
											s.is_empty()
										}
										['ि', s @ ..] => {
											super::push(out, 888);
											s.is_empty()
										}
										_ => false,
									}
								}
								['य', s @ ..] => {
									super::push(out, 710);
									s.is_empty()
								}
								['ि', s @ ..] => {
									super::push(out, 711);
									s.is_empty()
								}
								['\u{943}', s @ ..] => {
									super::push(out, 712);
									s.is_empty()
								}
								['\u{947}', s @ ..] => {
									super::push(out, 713);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ज', s @ ..] => {
					super::push(out, 246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 247);
					match s {
						[] => true,
						['र', 'ि', s @ ..] => {
							super::push(out, 457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 248);
					match s {
						[] => true,
						['ञ', s @ ..] => {
							super::push(out, 458);
							match s {
								[] => true,
								['\u{941}', s @ ..] => {
									super::push(out, 714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['र', 'क', s @ ..] => {
							super::push(out, 459);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['झ', s @ ..] => {
			super::push(out, 149);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['\u{94d}', 'ण', 'व', s @ ..] => {
					super::push(out, 250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ठ', s @ ..] => {
			super::push(out, 151);
			match s {
				[] => true,
				['ह', s @ ..] => {
					super::push(out, 251);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 252);
					match s {
						[] => true,
						['ष', 'ि', 'त', s @ ..] => {
							match s {
								[] => true,
								['ि', s @ ..] => {
									super::push(out, 716);
									s.is_empty()
								}
								['\u{94d}', s @ ..] => {
									super::push(out, 717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 152);
			s.is_empty()
		}
		['ण', s @ ..] => {
			super::push(out, 153);
			s.is_empty()
		}
		['त', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 253);
					s.is_empty()
				}
				['प', s @ ..] => {
					super::push(out, 254);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 255);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 256);
					match s {
						[] => true,
						['र', s @ ..] => {
							match s {
								[] => true,
								['क', '\u{1}', s @ ..] => {
									super::push(out, 718);
									s.is_empty()
								}
								['व', '\u{1}', s @ ..] => {
									super::push(out, 719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['व', s @ ..] => {
					super::push(out, 257);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 460);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 258);
					match s {
						[] => true,
						['ख', s @ ..] => {
							super::push(out, 462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', s @ ..] => {
					super::push(out, 259);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 260);
					match s {
						[] => true,
						['न', s @ ..] => {
							super::push(out, 463);
							match s {
								[] => true,
								['ि', 'न', s @ ..] => {
									super::push(out, 720);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 889);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['र', s @ ..] => {
							super::push(out, 464);
							match s {
								[] => true,
								['म', '\u{1}', s @ ..] => {
									super::push(out, 721);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['\u{947}', 'र', '\u{94d}', 'स', '\u{1}', s @ ..] => {
					super::push(out, 261);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 262);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['व', s @ ..] => {
					super::push(out, 263);
					match s {
						[] => true,
						['\u{94d}', 'स', s @ ..] => {
							super::push(out, 466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['व', s @ ..] => {
					super::push(out, 264);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 265);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 266);
					match s {
						[] => true,
						['छ', '\u{1}', s @ ..] => {
							super::push(out, 467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 267);
					match s {
						[] => true,
						['म', '\u{943}', 'त', '\u{1}', s @ ..] => {
							super::push(out, 468);
							s.is_empty()
						}
						['र', 'स', '\u{1}', s @ ..] => {
							super::push(out, 469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ब', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 268);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 269);
					match s {
						[] => true,
						['म', '\u{941}', 'ल', s @ ..] => {
							super::push(out, 470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			super::push(out, 160);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['ग', '\u{1}', s @ ..] => {
					super::push(out, 270);
					s.is_empty()
				}
				['च', s @ ..] => {
					super::push(out, 271);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 272);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 273);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 274);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['य', s @ ..] => super::t0::n163(s, out),
		['र', s @ ..] => super::t0::n164(s, out),
		['ल', s @ ..] => super::t0::n165(s, out),
		['ळ', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['व', s @ ..] => super::t0::n167(s, out),
		['श', s @ ..] => super::t0::n168(s, out),
		['ष', s @ ..] => super::t0::n169(s, out),
		['स', s @ ..] => super::t0::n170(s, out),
		['ह', s @ ..] => super::t0::n171(s, out),
		['ा', s @ ..] => super::t0::n172(s, out),
		['ि', s @ ..] => super::t0::n173(s, out),
		['ी', s @ ..] => super::t0::n174(s, out),
		['\u{947}', s @ ..] => super::t0::n175(s, out),
		['ो', s @ ..] => super::t0::n176(s, out),
		['ौ', s @ ..] => super::t0::n177(s, out),
		['\u{94d}', s @ ..] => super::t0::n178(s, out),
		_ => false,
	}
}

/// Words ending -यते.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['ज', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ड', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['ह', s @ ..] => {
					super::push(out, 654);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				['ी', s @ ..] => {
					super::push(out, 656);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 657);
					match s {
						[] => true,
						['त', s @ ..] => {
							super::push(out, 846);
							s.is_empty()
						}
						['द', '\u{94d}', 'न', 'इ', '\u{1}', s @ ..] => {
							super::push(out, 847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ल', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 397);
			match s {
				[] => true,
				['ो', 'ज', '\u{1}', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['ा', 'व', '\u{1}', s @ ..] => {
					super::push(out, 660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 399);
			s.is_empty()
		}
		['ि', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['ड', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['\u{94d}', 'ड', 'उ', '\u{1}', s @ ..] => {
							super::push(out, 848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 663);
					match s {
						[] => true,
						['\u{94d}', s @ ..] => {
							super::push(out, 849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ल', s @ ..] => {
					super::push(out, 664);
					match s {
						[] => true,
						['ि', s @ ..] => {
							super::push(out, 850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 403);
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['उ', s @ ..] => {
							super::push(out, 851);
							s.is_empty()
						}
						['ऋ', s @ ..] => {
							super::push(out, 852);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 853);
							s.is_empty()
						}
						['\u{941}', s @ ..] => {
							super::push(out, 854);
							s.is_empty()
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ज', s @ ..] => {
					super::push(out, 668);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['उ', s @ ..] => {
							super::push(out, 856);
							s.is_empty()
						}
						['प', s @ ..] => {
							super::push(out, 857);
							match s {
								[] => true,
								['\u{94d}', s @ ..] => {
									super::push(out, 951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', s @ ..] => {
							super::push(out, 858);
							match s {
								[] => true,
								['छ', s @ ..] => {
									super::push(out, 952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ध', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['इ', s @ ..] => {
							super::push(out, 859);
							s.is_empty()
						}
						['\u{941}', s @ ..] => {
							super::push(out, 860);
							match s {
								[] => true,
								['ब', '\u{94d}', s @ ..] => {
									super::push(out, 953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				['म', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 673);
					match s {
						[] => true,
						['\u{942}', 'प', '\u{1}', s @ ..] => {
							super::push(out, 861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 674);
					match s {
						[] => true,
						['ी', s @ ..] => {
							super::push(out, 862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['श', s @ ..] => {
					super::push(out, 675);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 863);
							s.is_empty()
						}
						['\u{943}', s @ ..] => {
							super::push(out, 864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 676);
					match s {
						[] => true,
						['ि', s @ ..] => {
							super::push(out, 865);
							s.is_empty()
						}
						['\u{943}', s @ ..] => {
							super::push(out, 866);
							match s {
								[] => true,
								['म', 'आ', s @ ..] => {
									super::push(out, 954);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{94d}', s @ ..] => {
							super::push(out, 867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 677);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ह', s @ ..] => {
					super::push(out, 678);
					match s {
						[] => true,
						['ऊ', s @ ..] => {
							super::push(out, 869);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 955);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{941}', s @ ..] => {
							super::push(out, 870);
							s.is_empty()
						}
						['\u{943}', s @ ..] => {
							super::push(out, 871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ाषते.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	s.is_empty()
}

/// Words ending -ुषते.
pub(crate) fn n410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 407);
	match s {
		[] => true,
		['ज', '\u{1}', s @ ..] => {
			super::push(out, 680);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ूषते.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['ङ', s @ ..] => {
			super::push(out, 681);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 682);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्षते.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['क', s @ ..] => {
			match s {
				[] => true,
				['ई', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				['च', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['द', s @ ..] => {
					super::push(out, 874);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 875);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 958);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 876);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -सते.
pub(crate) fn n199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	s.is_empty()
}

/// Words ending -हते.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 199);
	s.is_empty()
}

/// Words ending -ीते.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['ह', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['ि', 'ज', '\u{1}', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ेते.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	match s {
		[] => true,
		['श', s @ ..] => {
			super::push(out, 411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्ते.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['\u{943}', s @ ..] => {
					super::push(out, 684);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -धे.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['\u{94d}', s @ ..] => {
			super::push(out, 203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ये.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['\u{94d}', s @ ..] => {
			super::push(out, 204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ळे.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['ई', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -शे.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	s.is_empty()
}

/// Words ending -धाक्षीत्.
pub(crate) fn n992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 978);
	s.is_empty()
}

/// Words ending -भाक्षीत्.
pub(crate) fn n993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 979);
	s.is_empty()
}

/// Words ending -याक्षीत्.
pub(crate) fn n994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 980);
	s.is_empty()
}

/// Words ending -राक्षीत्.
pub(crate) fn n995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 981);
	match s {
		[] => true,
		['\u{94d}', 'प', s @ ..] => {
			super::push(out, 988);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -वाक्षीत्.
pub(crate) fn n996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 982);
	s.is_empty()
}

/// Words ending -ङ्क्षीत्.
pub(crate) fn n997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 983);
	s.is_empty()
}

/// Words ending -र्क्षीत्.
pub(crate) fn n998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 984);
	s.is_empty()
}

/// Words ending -र्षीत्.
pub(crate) fn n892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 879);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 962);
			match s {
				[] => true,
				['भ', s @ ..] => {
					super::push(out, 985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -सीत्.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 695);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['त', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['ा', s @ ..] => {
							super::push(out, 963);
							s.is_empty()
						}
						['ौ', s @ ..] => {
							super::push(out, 964);
							match s {
								[] => true,
								['न', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 986);
									s.is_empty()
								}
								['र', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -हीत्.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 697);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 698);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -भूत्.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	s.is_empty()
}

/// Words ending -ेत्.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['छ', '\u{94d}', 'च', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 441);
			s.is_empty()
		}
		['भ', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 442);
			s.is_empty()
		}
		['र', '\u{94d}', 'श', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ोत्.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['र', s @ ..] => {
			match s {
				[] => true,
				['अ', '\u{1}', s @ ..] => {
					super::push(out, 699);
					s.is_empty()
				}
				['\u{94d}', 'श', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -्त्.
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['र', s @ ..] => {
			match s {
				[] => true,
				['आ', '\u{1}', s @ ..] => {
					super::push(out, 701);
					s.is_empty()
				}
				['व', 'अ', '\u{1}', s @ ..] => {
					super::push(out, 702);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -न्.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['ग', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		['त', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 225);
			s.is_empty()
		}
		['र', '\u{94d}', 'क', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -र्.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['व', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 227);
			s.is_empty()
		}
		['ा', 'व', '\u{94d}', 'स', 'अ', '\u{1}', s @ ..] => {
			super::push(out, 228);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -स्.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	s.is_empty()
}

/// Words ending -यति.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ख', s @ ..] => {
			super::push(out, 277);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ग', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['च', s @ ..] => {
			super::push(out, 279);
			match s {
				[] => true,
				['ो', s @ ..] => {
					super::push(out, 474);
					s.is_empty()
				}
				['\u{94d}', 'र', 'अ', s @ ..] => {
					super::push(out, 475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ज', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 477);
					match s {
						[] => true,
						['भ', 'स', s @ ..] => {
							super::push(out, 723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', 'त', '\u{1}', s @ ..] => {
					super::push(out, 478);
					s.is_empty()
				}
				['\u{942}', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				['ो', s @ ..] => {
					super::push(out, 480);
					match s {
						[] => true,
						['य', s @ ..] => {
							super::push(out, 724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 481);
					match s {
						[] => true,
						['र', s @ ..] => {
							match s {
								[] => true,
								['ऊ', s @ ..] => {
									super::push(out, 890);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 965);
											s.is_empty()
										}
										_ => false,
									}
								}
								['भ', '\u{1}', s @ ..] => {
									super::push(out, 891);
									s.is_empty()
								}
								['म', '\u{1}', s @ ..] => {
									super::push(out, 892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ट', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 482);
					match s {
						[] => true,
						['ट', 'घ', s @ ..] => {
							super::push(out, 725);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ठ', s @ ..] => {
			super::push(out, 282);
			s.is_empty()
		}
		['ड', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['ो', s @ ..] => {
					super::push(out, 483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ण', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['\u{94d}', 'र', 'न', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['थ', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 486);
					match s {
						[] => true,
						['न', 'र', '\u{94d}', 'ग', '\u{1}', s @ ..] => {
							super::push(out, 727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 487);
					s.is_empty()
				}
				['\u{947}', s @ ..] => {
					super::push(out, 488);
					match s {
						[] => true,
						['व', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 894);
									s.is_empty()
								}
								['ि', s @ ..] => {
									super::push(out, 895);
									match s {
										[] => true,
										['त', 'र', '\u{94d}', 'प', '\u{1}', s @ ..] => {
											super::push(out, 966);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 491);
					s.is_empty()
				}
				['आ', s @ ..] => {
					super::push(out, 492);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 493);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 495);
					s.is_empty()
				}
				['च', s @ ..] => {
					super::push(out, 496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 497);
					match s {
						[] => true,
						['\u{94d}', 'क', '\u{1}', s @ ..] => {
							super::push(out, 730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 498);
					match s {
						[] => true,
						['ञ', s @ ..] => {
							super::push(out, 731);
							s.is_empty()
						}
						['ड', s @ ..] => {
							super::push(out, 732);
							match s {
								[] => true,
								['\u{94d}', 'ड', 'उ', '\u{1}', s @ ..] => {
									super::push(out, 896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['त', s @ ..] => {
							super::push(out, 733);
							match s {
								[] => true,
								['\u{902}', s @ ..] => {
									super::push(out, 897);
									s.is_empty()
								}
								_ => false,
							}
						}
						['थ', s @ ..] => {
							super::push(out, 734);
							match s {
								[] => true,
								['\u{94d}', s @ ..] => {
									match s {
										[] => true,
										['त', s @ ..] => {
											super::push(out, 967);
											s.is_empty()
										}
										['स', '\u{1}', s @ ..] => {
											super::push(out, 968);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ध', s @ ..] => {
							super::push(out, 735);
							s.is_empty()
						}
						['म', s @ ..] => {
							super::push(out, 736);
							s.is_empty()
						}
						['ल', s @ ..] => {
							super::push(out, 737);
							s.is_empty()
						}
						['व', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 898);
									s.is_empty()
								}
								['ी', s @ ..] => {
									super::push(out, 899);
									s.is_empty()
								}
								['\u{94d}', s @ ..] => {
									super::push(out, 900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['स', s @ ..] => {
							super::push(out, 738);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 499);
					match s {
						[] => true,
						['ष', s @ ..] => {
							super::push(out, 740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ो', s @ ..] => {
					super::push(out, 500);
					match s {
						[] => true,
						['ग', s @ ..] => {
							super::push(out, 741);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 501);
					match s {
						[] => true,
						['म', s @ ..] => {
							super::push(out, 743);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 744);
							match s {
								[] => true,
								['अ', s @ ..] => {
									super::push(out, 901);
									s.is_empty()
								}
								['त', s @ ..] => {
									super::push(out, 902);
									s.is_empty()
								}
								['म', s @ ..] => {
									super::push(out, 903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ब', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 292);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ग', s @ ..] => {
					super::push(out, 503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ा', s @ ..] => {
					super::push(out, 504);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['य', s @ ..] => super::t0::n297(s, out),
		['र', s @ ..] => super::t0::n298(s, out),
		['ल', s @ ..] => super::t0::n299(s, out),
		['व', s @ ..] => super::t0::n300(s, out),
		['श', s @ ..] => super::t0::n301(s, out),
		['ष', s @ ..] => super::t0::n302(s, out),
		['स', s @ ..] => {
			super::push(out, 300);
			s.is_empty()
		}
		['ह', s @ ..] => super::t0::n304(s, out),
		['ा', s @ ..] => super::t0::n305(s, out),
		['ी', s @ ..] => super::t0::n306(s, out),
		['\u{94d}', s @ ..] => super::t0::n307(s, out),
		_ => false,
	}
}

/// Words ending -रति.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['च', s @ ..] => {
			super::push(out, 305);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 546);
					s.is_empty()
				}
				['आ', s @ ..] => {
					super::push(out, 547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			super::push(out, 307);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['\u{94d}', 'ध', s @ ..] => {
					super::push(out, 550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ष', s @ ..] => {
					super::push(out, 553);
					match s {
						[] => true,
						['\u{94d}', 'क', '\u{1}', s @ ..] => {
							super::push(out, 793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -लति.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['ख', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['फ', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ल', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['\u{94d}', 'ह', s @ ..] => {
					super::push(out, 557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['म', '\u{94d}', s @ ..] => {
					super::push(out, 558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -वति.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['अ', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['ी', s @ ..] => {
					super::push(out, 560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 561);
					match s {
						[] => true,
						['द', '\u{1}', s @ ..] => {
							super::push(out, 794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ा', s @ ..] => {
			super::push(out, 320);
			s.is_empty()
		}
		['ी', s @ ..] => {
			super::push(out, 321);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['र', 'च', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -शति.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['ा', 'क', '\u{1}', s @ ..] => {
			super::push(out, 323);
			s.is_empty()
		}
		['ि', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['व', 'प', s @ ..] => {
					super::push(out, 563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['र', '\u{94d}', 'क', 'आ', s @ ..] => {
					super::push(out, 564);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -षति.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['ओ', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['ज', s @ ..] => {
					super::push(out, 566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{942}', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['च', s @ ..] => {
					super::push(out, 567);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['भ', s @ ..] => {
					super::push(out, 568);
					match s {
						[] => true,
						['\u{941}', 'ब', '\u{1}', s @ ..] => {
							super::push(out, 796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['श', s @ ..] => {
					super::push(out, 569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['अ', s @ ..] => {
							super::push(out, 798);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['उ', s @ ..] => {
							super::push(out, 799);
							s.is_empty()
						}
						['प', s @ ..] => {
							super::push(out, 800);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['य', s @ ..] => {
							super::push(out, 801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', s @ ..] => {
							super::push(out, 802);
							match s {
								[] => true,
								['व', s @ ..] => {
									super::push(out, 935);
									match s {
										[] => true,
										['ि', 'व', '\u{1}', s @ ..] => {
											super::push(out, 977);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 803);
							match s {
								[] => true,
								['ि', s @ ..] => {
									super::push(out, 936);
									s.is_empty()
								}
								_ => false,
							}
						}
						['घ', s @ ..] => {
							super::push(out, 804);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['व', s @ ..] => {
							super::push(out, 805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -सति.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['ज', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['\u{94d}', 'ग', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ल', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -हति.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['ज', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 577);
					s.is_empty()
				}
				['\u{943}', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ई', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ऊ', s @ ..] => {
			super::push(out, 337);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{941}', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['प', s @ ..] => {
					super::push(out, 582);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 341);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 584);
					match s {
						[] => true,
						['अ', s @ ..] => {
							super::push(out, 806);
							s.is_empty()
						}
						['ग', s @ ..] => {
							super::push(out, 807);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ाति.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['ग', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ण', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 586);
					match s {
						[] => true,
						['ष', s @ ..] => {
							super::push(out, 809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ी', s @ ..] => {
					super::push(out, 587);
					s.is_empty()
				}
				['\u{943}', s @ ..] => {
					super::push(out, 588);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 810);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 589);
					match s {
						[] => true,
						['ष', s @ ..] => {
							super::push(out, 811);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 812);
							match s {
								[] => true,
								['\u{943}', 'ग', 'र', s @ ..] => {
									super::push(out, 940);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['थ', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['द', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 591);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ध', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['ा', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 594);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['ध', s @ ..] => {
							super::push(out, 814);
							s.is_empty()
						}
						['भ', s @ ..] => {
							super::push(out, 815);
							match s {
								[] => true,
								['उ', s @ ..] => {
									super::push(out, 941);
									s.is_empty()
								}
								['त', s @ ..] => {
									super::push(out, 942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['स', s @ ..] => {
							super::push(out, 816);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['प', s @ ..] => {
			super::push(out, 348);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 596);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['आ', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['व', s @ ..] => {
			super::push(out, 352);
			s.is_empty()
		}
		['स', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['\u{94d}', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['ज', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -िति.
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['द', s @ ..] => {
			super::push(out, 355);
			s.is_empty()
		}
		['न', s @ ..] => {
			super::push(out, 356);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 357);
			s.is_empty()
		}
		['स', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['व', '\u{94d}', s @ ..] => {
					match s {
						[] => true,
						['छ', s @ ..] => {
							super::push(out, 817);
							s.is_empty()
						}
						['श', s @ ..] => {
							super::push(out, 818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ीति.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['त', s @ ..] => {
			super::push(out, 359);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['भ', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 603);
					match s {
						[] => true,
						['ो', 'र', '\u{1}', s @ ..] => {
							super::push(out, 819);
							s.is_empty()
						}
						['\u{94d}', 'ब', '\u{1}', s @ ..] => {
							super::push(out, 820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ेति.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 361);
			s.is_empty()
		}
		['भ', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['ि', 'ब', '\u{1}', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 363);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['\u{94d}', 'क', 'आ', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ोति.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['ण', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['ऋ', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				['\u{943}', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['घ', s @ ..] => {
							super::push(out, 821);
							s.is_empty()
						}
						['त', s @ ..] => {
							super::push(out, 822);
							s.is_empty()
						}
						['व', s @ ..] => {
							super::push(out, 823);
							s.is_empty()
						}
						['श', s @ ..] => {
							super::push(out, 824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 609);
					match s {
						[] => true,
						['ष', s @ ..] => {
							super::push(out, 825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 367);
			match s {
				[] => true,
				['त', s @ ..] => {
					super::push(out, 610);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 612);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 613);
					match s {
						[] => true,
						['ह', s @ ..] => {
							super::push(out, 826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{941}', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['द', s @ ..] => {
							super::push(out, 827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 615);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 828);
							s.is_empty()
						}
						['घ', s @ ..] => {
							super::push(out, 829);
							match s {
								[] => true,
								['ि', 'त', '\u{94d}', 'स', '\u{1}', s @ ..] => {
									super::push(out, 944);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ध', s @ ..] => {
							super::push(out, 830);
							match s {
								[] => true,
								['ऋ', s @ ..] => {
									super::push(out, 945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['प', s @ ..] => {
							super::push(out, 831);
							s.is_empty()
						}
						['भ', s @ ..] => {
							super::push(out, 832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['ी', 'व', '\u{94d}', 'स', '\u{1}', s @ ..] => {
							super::push(out, 833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['\u{941}', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ौति.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['त', s @ ..] => {
			super::push(out, 370);
			s.is_empty()
		}
		['न', s @ ..] => {
			super::push(out, 371);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 372);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्ति.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['क', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['ण', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['\u{943}', 'व', s @ ..] => {
							super::push(out, 834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['त', s @ ..] => {
					super::push(out, 619);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 620);
					match s {
						[] => true,
						['अ', s @ ..] => {
							super::push(out, 835);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['त', s @ ..] => {
							super::push(out, 836);
							s.is_empty()
						}
						['भ', s @ ..] => {
							super::push(out, 837);
							s.is_empty()
						}
						['ि', s @ ..] => {
							super::push(out, 838);
							s.is_empty()
						}
						['\u{941}', s @ ..] => {
							super::push(out, 839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 621);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 622);
					s.is_empty()
				}
				['\u{947}', s @ ..] => {
					super::push(out, 623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 624);
					s.is_empty()
				}
				['ण', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['\u{943}', s @ ..] => {
							super::push(out, 840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['न', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['उ', s @ ..] => {
							super::push(out, 841);
							s.is_empty()
						}
						['ि', s @ ..] => {
							super::push(out, 842);
							match s {
								[] => true,
								['छ', '\u{1}', s @ ..] => {
									super::push(out, 947);
									s.is_empty()
								}
								['भ', '\u{1}', s @ ..] => {
									super::push(out, 948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{947}', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['व', s @ ..] => {
							super::push(out, 843);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 949);
									s.is_empty()
								}
								['\u{947}', 'व', '\u{1}', s @ ..] => {
									super::push(out, 950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['ह', s @ ..] => {
					super::push(out, 628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['क', '\u{1}', s @ ..] => {
					super::push(out, 629);
					s.is_empty()
				}
				['ग', s @ ..] => {
					super::push(out, 630);
					s.is_empty()
				}
				['घ', s @ ..] => {
					super::push(out, 631);
					s.is_empty()
				}
				['प', s @ ..] => {
					super::push(out, 632);
					s.is_empty()
				}
				['भ', s @ ..] => {
					super::push(out, 633);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 634);
					match s {
						[] => true,
						['इ', '\u{1}', s @ ..] => {
							super::push(out, 844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['अ', s @ ..] => {
					super::push(out, 636);
					s.is_empty()
				}
				['न', s @ ..] => {
					super::push(out, 637);
					s.is_empty()
				}
				['भ', 'ब', '\u{1}', s @ ..] => {
					super::push(out, 638);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ययति.
pub(crate) fn n297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 294);
	match s {
		[] => true,
		['ा', 'ल', '\u{941}', 'क', '\u{1}', s @ ..] => {
			super::push(out, 505);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 506);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -रयति.
pub(crate) fn n298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 295);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['द', s @ ..] => {
					super::push(out, 748);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['स', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
				['श', '\u{1}', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -लयति.
pub(crate) fn n299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 296);
	match s {
		[] => true,
		['ा', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['क', s @ ..] => {
					super::push(out, 751);
					s.is_empty()
				}
				['च', s @ ..] => {
					super::push(out, 752);
					s.is_empty()
				}
				['द', s @ ..] => {
					super::push(out, 753);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['प', s @ ..] => {
					super::push(out, 754);
					s.is_empty()
				}
				['ष', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['\u{94d}', 'क', 'र', s @ ..] => {
							super::push(out, 907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ो', s @ ..] => {
			super::push(out, 510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -वयति.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 511);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 512);
			match s {
				[] => true,
				['भ', s @ ..] => {
					super::push(out, 756);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['र', s @ ..] => {
					super::push(out, 757);
					s.is_empty()
				}
				['ल', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ी', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['ज', s @ ..] => {
					super::push(out, 759);
					s.is_empty()
				}
				['स', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -शयति.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 515);
			s.is_empty()
		}
		['ा', 'प', s @ ..] => {
			super::push(out, 516);
			s.is_empty()
		}
		['\u{947}', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 518);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -षयति.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['ो', s @ ..] => {
			super::push(out, 519);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 520);
			match s {
				[] => true,
				['क', 'त', '\u{1}', s @ ..] => {
					super::push(out, 761);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -हयति.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	match s {
		[] => true,
		['\u{902}', s @ ..] => {
			super::push(out, 521);
			s.is_empty()
		}
		['ा', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['र', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				['व', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['\u{94d}', 'र', 'ि', 'न', '\u{1}', s @ ..] => {
							super::push(out, 909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{947}', s @ ..] => {
			super::push(out, 523);
			s.is_empty()
		}
		['ो', s @ ..] => {
			super::push(out, 524);
			s.is_empty()
		}
		['\u{94d}', s @ ..] => {
			super::push(out, 525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ायति.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['घ', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['ऋ', '\u{1}', s @ ..] => {
					super::push(out, 764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['य', s @ ..] => {
			super::push(out, 527);
			s.is_empty()
		}
		['र', s @ ..] => {
			super::push(out, 528);
			match s {
				[] => true,
				['ि', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 766);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ीयति.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['प', s @ ..] => {
			super::push(out, 529);
			s.is_empty()
		}
		['ष', s @ ..] => {
			super::push(out, 530);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -्यति.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['ग', s @ ..] => {
			super::push(out, 531);
			s.is_empty()
		}
		['च', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['उ', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['त', s @ ..] => {
			super::push(out, 533);
			s.is_empty()
		}
		['द', s @ ..] => {
			super::push(out, 534);
			s.is_empty()
		}
		['ध', s @ ..] => {
			super::push(out, 535);
			match s {
				[] => true,
				['ऋ', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['न', s @ ..] => {
			super::push(out, 536);
			s.is_empty()
		}
		['प', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['त', s @ ..] => {
					super::push(out, 771);
					s.is_empty()
				}
				['ि', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['भ', s @ ..] => {
			super::push(out, 538);
			s.is_empty()
		}
		['म', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['ा', s @ ..] => {
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 910);
							s.is_empty()
						}
						['त', s @ ..] => {
							super::push(out, 911);
							s.is_empty()
						}
						['द', s @ ..] => {
							super::push(out, 912);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 913);
							match s {
								[] => true,
								['\u{94d}', 'श', '\u{1}', s @ ..] => {
									super::push(out, 969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['श', s @ ..] => {
							super::push(out, 914);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 970);
									s.is_empty()
								}
								['र', s @ ..] => {
									super::push(out, 971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['र', s @ ..] => {
			super::push(out, 540);
			s.is_empty()
		}
		['व', s @ ..] => {
			super::push(out, 541);
			s.is_empty()
		}
		['श', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				['प', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ष', s @ ..] => {
			super::push(out, 543);
			match s {
				[] => true,
				['ए', s @ ..] => {
					super::push(out, 775);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 776);
					match s {
						[] => true,
						['ख', s @ ..] => {
							super::push(out, 916);
							match s {
								[] => true,
								['\u{947}', 'ल', '\u{1}', s @ ..] => {
									super::push(out, 972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['च', s @ ..] => {
							super::push(out, 917);
							match s {
								[] => true,
								['\u{94d}', 'ञ', 'अ', '\u{1}', s @ ..] => {
									super::push(out, 973);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ल', s @ ..] => {
							super::push(out, 918);
							match s {
								[] => true,
								['\u{94d}', 'श', '\u{1}', s @ ..] => {
									super::push(out, 974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{941}', s @ ..] => {
					super::push(out, 777);
					s.is_empty()
				}
				['\u{943}', s @ ..] => {
					super::push(out, 778);
					match s {
						[] => true,
						['त', s @ ..] => {
							super::push(out, 919);
							s.is_empty()
						}
						['म', s @ ..] => {
							super::push(out, 920);
							s.is_empty()
						}
						['ह', s @ ..] => {
							super::push(out, 921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['क', s @ ..] => {
							super::push(out, 922);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 923);
							match s {
								[] => true,
								['ई', '\u{1}', s @ ..] => {
									super::push(out, 975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['स', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['\u{902}', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				['अ', s @ ..] => {
					super::push(out, 781);
					s.is_empty()
				}
				['ज', s @ ..] => {
					super::push(out, 782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						['र', s @ ..] => {
							super::push(out, 925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['म', s @ ..] => {
					super::push(out, 783);
					s.is_empty()
				}
				['य', s @ ..] => {
					super::push(out, 784);
					s.is_empty()
				}
				['र', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['इ', '\u{1}', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['व', s @ ..] => {
					super::push(out, 786);
					s.is_empty()
				}
				['ा', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['ञ', s @ ..] => {
							super::push(out, 927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{94d}', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ह', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['न', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 928);
							s.is_empty()
						}
						['प', s @ ..] => {
							super::push(out, 929);
							match s {
								[] => true,
								['उ', s @ ..] => {
									super::push(out, 976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ि', s @ ..] => {
							super::push(out, 930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ि', s @ ..] => {
					super::push(out, 790);
					s.is_empty()
				}
				['\u{941}', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['र', s @ ..] => {
							super::push(out, 931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

