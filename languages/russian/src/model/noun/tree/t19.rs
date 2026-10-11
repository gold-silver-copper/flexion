use flexion::Path;

/// Words ending -ьяк.
pub(crate) fn n2434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2427);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11600);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32144);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32145);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11601);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32146);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60441);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88185);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 88186);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109467);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123417);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32147);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32148);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60443);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88187);
									s.is_empty()
								}
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
			super::push(out, 11602);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32149);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11603);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32150);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60445);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11604);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 32151);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -азвал.
pub(crate) fn n32438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32164);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 60456);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 88194);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 109472);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123418);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 131814);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136739);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивал.
pub(crate) fn n11662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11620);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 32165);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 60457);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 88195);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109473);
									s.is_empty()
								}
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
			super::push(out, 32166);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 60458);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йвал.
pub(crate) fn n11663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11621);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 32167);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 60459);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -квал.
pub(crate) fn n11664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11622);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32168);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 32169);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 60460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 32170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нвал.
pub(crate) fn n11665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11623);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 32171);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 60461);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88198);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109474);
									s.is_empty()
								}
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
			super::push(out, 32172);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 60462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 60463);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 88200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овал.
pub(crate) fn n11666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11624);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32173);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 32174);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 60464);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 88201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 60465);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88202);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 109475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 60466);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 88203);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 60467);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 88204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 60468);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 88205);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 60469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 32175);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рвал.
pub(crate) fn n11667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11625);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32176);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 60470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 60471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 60472);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 88207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 32177);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 60473);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -свал.
pub(crate) fn n11668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11626);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32178);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 60474);
					match s {
						[] => true,
						['а', 'с', '-', s @ ..] => {
							super::push(out, 88208);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 109478);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123419);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 131815);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 109479);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123420);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 131816);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отвал.
pub(crate) fn n32455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32179);
	s.is_empty()
}

/// Words ending -увал.
pub(crate) fn n11670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11628);
	s.is_empty()
}

/// Words ending -хвал.
pub(crate) fn n11671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11629);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 60475);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 60476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 32180);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 60477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 32181);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 60478);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывал.
pub(crate) fn n11672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11630);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32182);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 60479);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ювал.
pub(crate) fn n11673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11631);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 32183);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 60480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гал.
pub(crate) fn n2438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2431);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11632);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11633);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32184);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60481);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88211);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 32185);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60482);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32187);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 32188);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60484);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88213);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109480);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 60485);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 60486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дал.
pub(crate) fn n2439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2432);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11634);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 32189);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60487);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88215);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109481);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11635);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11636);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11637);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32192);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60490);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88216);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109482);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11638);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32193);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60491);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 60492);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 60493);
							s.is_empty()
						}
						['ч', '\u{1}', s @ ..] => {
							super::push(out, 60494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11639);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32194);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 60495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еал.
pub(crate) fn n2440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2433);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 11640);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жал.
pub(crate) fn n2441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2434);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11641);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32196);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зал.
pub(crate) fn n2442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2435);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 11642);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 11643);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11644);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32198);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 88219);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109483);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123421);
													match s {
														[] => true,
														['з', s @ ..] => super::t36::n133176(s, out),
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
									super::push(out, 88220);
									s.is_empty()
								}
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
			super::push(out, 11645);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 11646);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иал.
pub(crate) fn n2443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2436);
	match s {
		[] => true,
		['в', 'а', s @ ..] => {
			super::push(out, 11647);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11648);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32201);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60499);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88221);
									s.is_empty()
								}
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
			super::push(out, 11649);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11650);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32203);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60500);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88222);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109484);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 60501);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88223);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109485);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11651);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32204);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60502);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88224);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109486);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11652);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32205);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60503);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88225);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 88226);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109487);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123422);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133177(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 32206);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60504);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88227);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109488);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123423);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32207);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60505);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88228);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109489);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123424);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133178(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 32208);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 11653);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32209);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 60507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60508);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88231);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 60509);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88232);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109490);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123425);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60510);
							match s {
								[] => true,
								['т', 'о', 'п', 'и', 'т', s @ ..] => {
									super::push(out, 88233);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 109491);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123426);
													match s {
														[] => true,
														['у', s @ ..] => super::t36::n133179(s, out),
														_ => false,
													}
												}
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
							super::push(out, 60511);
							match s {
								[] => true,
								['в', s @ ..] => super::t36::n89183(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t36::n32489(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кал.
pub(crate) fn n2444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2437);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11654);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32211);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 32212);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 32213);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 32214);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60513);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32215);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60514);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 32216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11655);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11656);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32217);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88237);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 88238);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109493);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109494);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123428);
													match s {
														[] => true,
														['р', s @ ..] => super::t36::n133181(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 109495);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 88239);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109496);
											s.is_empty()
										}
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
					super::push(out, 32218);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60515);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88240);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109497);
											s.is_empty()
										}
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
					super::push(out, 32219);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60516);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88241);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109498);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123429);
													match s {
														[] => true,
														['с', s @ ..] => super::t36::n133182(s, out),
														_ => false,
													}
												}
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
							super::push(out, 60517);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 60518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 11657);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32220);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11658);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11659);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32221);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 32222);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 60520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 32223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11660);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32224);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 32225);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 60521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60522);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 60523);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88242);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 109499);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11661);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -алал.
pub(crate) fn n2445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2438);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 11662);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32226);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мал.
pub(crate) fn n2446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2439);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11663);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 32227);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60524);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 88243);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109500);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123430);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133183(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 32228);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 32229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11664);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32230);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60525);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88244);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109501);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11665);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32231);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60526);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88245);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109502);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11666);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11667);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32233);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 60527);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88246);
									s.is_empty()
								}
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
			super::push(out, 11668);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 32234);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 11669);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32235);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60529);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88247);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109503);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11670);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нал.
pub(crate) fn n2447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2440);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11671);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11672);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11673);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11674);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32238);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 32239);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60530);
							match s {
								[] => true,
								['р', 'о', '\u{1}', s @ ..] => {
									super::push(out, 88248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 60531);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88249);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109504);
											s.is_empty()
										}
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
					super::push(out, 32240);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60532);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88250);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109505);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 88251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32241);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 32242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11675);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32243);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60534);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88252);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109506);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 60535);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88253);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109507);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123431);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 88254);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109508);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 123432);
													match s {
														[] => true,
														['о', 'р', 'п', '-', s @ ..] => super::t36::n133184(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 60536);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123433);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 123434);
													match s {
														[] => true,
														['е', s @ ..] => super::t36::n133185(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 109509);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123435);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 88255);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109510);
											s.is_empty()
										}
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
					super::push(out, 32244);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32245);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60537);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88256);
									s.is_empty()
								}
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
			super::push(out, 11676);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'б', 'и', 'р', 'т', 'в', s @ ..] => {
			super::push(out, 11677);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32247);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пал.
pub(crate) fn n2448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2441);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11678);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11679);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 32249);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60539);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88258);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 60540);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88259);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109511);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123436);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11680);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11681);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рал.
pub(crate) fn n2449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2442);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32251);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 32252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11682);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11683);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32254);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60542);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32255);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60543);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88261);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32256);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60544);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 88262);
									s.is_empty()
								}
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
			super::push(out, 11684);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32257);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60545);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', 'е', 'ф', '\u{1}', s @ ..] => {
					super::push(out, 32258);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60546);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 60547);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32259);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60548);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88266);
									s.is_empty()
								}
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
			super::push(out, 11685);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32260);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60549);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88267);
									s.is_empty()
								}
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
			super::push(out, 11686);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32261);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 32262);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32263);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60552);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 60553);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88268);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109512);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123437);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32264);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 32265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11687);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32266);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11688);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32267);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60556);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 60557);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88270);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109513);
											s.is_empty()
										}
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
					super::push(out, 32268);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60558);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88271);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32269);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32270);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32271);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60561);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 60562);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88272);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109514);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 123438);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133186(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11689);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32272);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 32273);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60563);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88273);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 32274);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сал.
pub(crate) fn n2450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2443);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 11690);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32275);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11691);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32276);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11692);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32277);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'е', 'в', 'и', 'н', 'у', '\u{1}', s @ ..] => {
			super::push(out, 11693);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 11694);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32278);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тал.
pub(crate) fn n2451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2444);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11695);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11696);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32281);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 60570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11697);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32282);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60571);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88274);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109515);
											s.is_empty()
										}
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
					super::push(out, 32283);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60572);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88275);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109516);
											s.is_empty()
										}
										['ф', s @ ..] => {
											super::push(out, 109517);
											s.is_empty()
										}
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
					super::push(out, 32284);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60573);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88276);
									s.is_empty()
								}
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
			super::push(out, 11698);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32285);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60574);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 88277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11699);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60575);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 60576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 32286);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60577);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88278);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109518);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 123439);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 88279);
									s.is_empty()
								}
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
			super::push(out, 11700);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32287);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11701);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32289);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11702);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32290);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32291);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60582);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 88280);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109519);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 123440);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133187(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11703);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32292);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11704);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32293);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60584);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уал.
pub(crate) fn n2452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2445);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11705);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 11706);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32294);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60585);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88282);
									s.is_empty()
								}
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
			super::push(out, 11707);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 11708);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32295);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11709);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32296);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11710);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32297);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32298);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60589);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88283);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109520);
											s.is_empty()
										}
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
					super::push(out, 32299);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60590);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фал.
pub(crate) fn n2453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2446);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11711);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11712);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 32300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11713);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хал.
pub(crate) fn n2454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2447);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11714);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11715);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32303);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 60592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11716);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цал.
pub(crate) fn n2455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2448);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 11717);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чал.
pub(crate) fn n2456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2449);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11718);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шал.
pub(crate) fn n2457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2450);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11719);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32306);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60593);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88285);
									s.is_empty()
								}
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
			super::push(out, 11720);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32307);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыба-ангел.
pub(crate) fn n124724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123441);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 131829);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 136749);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дел.
pub(crate) fn n2467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2460);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11739);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11740);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32329);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60610);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88293);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 109523);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123442);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32330);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11741);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32331);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11742);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11743);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11744);
			match s {
				[] => true,
				['в', 'о', s @ ..] => {
					super::push(out, 32334);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60613);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60614);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88295);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109524);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 60615);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88296);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109525);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123443);
													match s {
														[] => true,
														['б', s @ ..] => super::t36::n133189(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 32335);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 32336);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 32337);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 32338);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60617);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 60618);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32339);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 32340);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60620);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88299);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109526);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 60621);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88300);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109527);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11745);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11746);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11747);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -екозел.
pub(crate) fn n61267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60625);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 88302);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 109528);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 123444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узел.
pub(crate) fn n11797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11751);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 32346);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 32347);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 32348);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 60626);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 60627);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 88303);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109529);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 123445);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 131831);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 88304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иел.
pub(crate) fn n2470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2463);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11752);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 32349);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кел.
pub(crate) fn n2471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2464);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11753);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 32351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11754);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32352);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лел.
pub(crate) fn n2472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2465);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11755);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32353);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мел.
pub(crate) fn n2473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2466);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11756);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11757);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32354);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60631);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 88305);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 109530);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123446);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11758);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32355);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 11759);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32356);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60633);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нел.
pub(crate) fn n2474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2467);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 11760);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 32357);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60634);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88307);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109531);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 123447);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пел.
pub(crate) fn n2475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2468);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11761);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32358);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 32359);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60635);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 60636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11762);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 32360);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11763);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32361);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11764);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 32362);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рел.
pub(crate) fn n2476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2469);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11765);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11766);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32364);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60639);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32365);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60640);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88310);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109532);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11767);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11768);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32367);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60641);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 60642);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 60643);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88312);
									match s {
										[] => true,
										['а', 'с', '\u{1}', s @ ..] => {
											super::push(out, 109533);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 88313);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 88314);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109534);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 88315);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109535);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123448);
													s.is_empty()
												}
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
							super::push(out, 60644);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88316);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109536);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 60645);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88317);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109537);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сел.
pub(crate) fn n2477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2470);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11769);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 32368);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 60646);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88318);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109538);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123449);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11770);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32369);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11771);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32370);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32371);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11772);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32372);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 32373);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60650);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32374);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60651);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32375);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60652);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88321);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109539);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11773);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11774);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32377);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11775);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32378);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тел.
pub(crate) fn n2478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2471);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11776);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11777);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32380);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 32381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32382);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11778);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32383);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11779);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32384);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32385);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60660);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 60661);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88322);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11780);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32386);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фел.
pub(crate) fn n2479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2472);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 11781);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 32387);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цел.
pub(crate) fn n2480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2473);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 11782);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32388);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11783);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32389);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чел.
pub(crate) fn n2481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2474);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11784);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11785);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 11786);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32391);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 32392);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -богомил.
pub(crate) fn n89281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88330);
	s.is_empty()
}

/// Words ending -рмил.
pub(crate) fn n11864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11815);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 32419);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тмил.
pub(crate) fn n11865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11816);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 32420);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -умил.
pub(crate) fn n11866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11817);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 32421);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 60680);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 88331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нил.
pub(crate) fn n2495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2488);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32423);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 32424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11818);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11819);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11820);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32426);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60682);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88332);
									s.is_empty()
								}
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
			super::push(out, 11821);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 11822);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32427);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60683);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оил.
pub(crate) fn n2496(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32428);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 32429);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11823);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32430);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пил.
pub(crate) fn n2497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2489);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11824);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 11825);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11826);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32431);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60685);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88334);
									s.is_empty()
								}
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
			super::push(out, 11827);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32432);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11828);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11829);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32433);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рил.
pub(crate) fn n2498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2490);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11830);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 11831);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32434);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60688);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32435);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60689);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88336);
									s.is_empty()
								}
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
			super::push(out, 11832);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11833);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11835);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32438);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60691);
							match s {
								[] => true,
								['о', 'т', s @ ..] => {
									super::push(out, 88337);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109543);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 123450);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133191(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сил.
pub(crate) fn n2499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2491);
	match s {
		[] => true,
		['а', 'в', s @ ..] => {
			super::push(out, 11837);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32439);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 60692);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88338);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109544);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11838);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11839);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11840);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32442);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тил.
pub(crate) fn n2500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2492);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11841);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 11842);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32443);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 60693);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88339);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109545);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11843);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32444);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 32445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 11844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уил.
pub(crate) fn n2501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2493);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 11845);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11846);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32447);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фил.
pub(crate) fn n2502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2494);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32448);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 32449);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 60695);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88340);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109546);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123451);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11847);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32450);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60696);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88341);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109547);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123452);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11848);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32451);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60697);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88342);
									s.is_empty()
								}
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
			super::push(out, 11849);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32452);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32453);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60698);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88343);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 109548);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123453);
													s.is_empty()
												}
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
							super::push(out, 60699);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 32454);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60700);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88345);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109549);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60701);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 60702);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88346);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109550);
											s.is_empty()
										}
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
					super::push(out, 32455);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60704);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88347);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109551);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 60705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32456);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60706);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88348);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 60707);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88349);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109552);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60708);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 60709);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88350);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109553);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123454);
													s.is_empty()
												}
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
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88351);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 88352);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109554);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 123455);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133192(s, out),
														_ => false,
													}
												}
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
							super::push(out, 60710);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88353);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109555);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123456);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 60711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32457);
					s.is_empty()
				}
				['р', s @ ..] => super::t36::n32745(s, out),
				['с', s @ ..] => super::t36::n32746(s, out),
				['т', s @ ..] => super::t36::n32747(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хил.
pub(crate) fn n2503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2495);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11850);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 11851);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 32461);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цил.
pub(crate) fn n2504(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11852);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32462);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11853);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32463);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60718);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чил.
pub(crate) fn n2505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2496);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11854);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32464);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60719);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шил.
pub(crate) fn n2506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2497);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 11855);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32465);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фильм-мюзикл.
pub(crate) fn n141126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139686);
	s.is_empty()
}

/// Words ending -еви-металл.
pub(crate) fn n124744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123461);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 131835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 136751);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -апон-халл.
pub(crate) fn n124745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123462);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 131836);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 136752);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тосинкелл.
pub(crate) fn n124746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123463);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 131837);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 136753);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -акдоннелл.
pub(crate) fn n124747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123464);
	s.is_empty()
}

/// Words ending -илл.
pub(crate) fn n2516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2507);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11884);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32491);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 32492);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60740);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88375);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109570);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123465);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11885);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32493);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60741);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32494);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60742);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 88377);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109571);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 123466);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 88378);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109572);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123467);
													s.is_empty()
												}
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
							super::push(out, 60743);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88379);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109573);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 32495);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 60744);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88380);
									s.is_empty()
								}
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
			super::push(out, 11886);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 11887);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32496);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60745);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88381);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109574);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11888);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32497);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11889);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32498);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 32499);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88382);
									s.is_empty()
								}
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
			super::push(out, 11890);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11891);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11892);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32501);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 11893);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32502);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 32503);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 32504);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32505);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60752);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88383);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109575);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123468);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 11894);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32507);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60753);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 88384);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109576);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орнуолл.
pub(crate) fn n89337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88386);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 109577);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 123469);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улл.
pub(crate) fn n2518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2509);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 11898);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32510);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элл.
pub(crate) fn n2519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2510);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 11899);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32511);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60757);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88388);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109578);
											s.is_empty()
										}
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
					super::push(out, 32512);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -драконбол.
pub(crate) fn n124753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123470);
	s.is_empty()
}

/// The word дискобол.
pub(crate) fn n124754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123471);
	s.is_empty()
}

/// Words ending -баскетбол.
pub(crate) fn n124755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123472);
	s.is_empty()
}

/// Words ending -цбол.
pub(crate) fn n11967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11914);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32529);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 60772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вол.
pub(crate) fn n2521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2512);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11916);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11917);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32530);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11918);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32531);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60774);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88403);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109585);
											s.is_empty()
										}
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
					super::push(out, 32532);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 11919);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32533);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11920);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32534);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60777);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 88404);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 109586);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123473);
													match s {
														[] => true,
														['р', s @ ..] => super::t36::n133197(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11921);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 32535);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60778);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 88405);
									s.is_empty()
								}
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
			super::push(out, 11922);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32536);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60779);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88406);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109587);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123474);
													match s {
														[] => true,
														['р', s @ ..] => super::t36::n133198(s, out),
														_ => false,
													}
												}
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
							super::push(out, 60780);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88407);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109588);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11923);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 32537);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гол.
pub(crate) fn n2522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2513);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11924);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11925);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32538);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11926);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 32539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11927);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32540);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60784);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88408);
									s.is_empty()
								}
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
			super::push(out, 11928);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11929);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32542);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11930);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32544);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60787);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88409);
									s.is_empty()
								}
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
			super::push(out, 11931);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11932);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32546);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 32547);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 60788);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88410);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109589);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123475);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60789);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дол.
pub(crate) fn n2523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2514);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11933);
			s.is_empty()
		}
		['з', 'е', s @ ..] => {
			super::push(out, 11934);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32549);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60790);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88411);
									s.is_empty()
								}
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
			super::push(out, 11935);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32550);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 32551);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 32552);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60792);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 32553);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60793);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88413);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109590);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 60794);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32554);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32555);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60796);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88415);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109591);
											s.is_empty()
										}
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
					super::push(out, 32556);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60797);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32557);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 11936);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32558);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11937);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32559);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еол.
pub(crate) fn n2524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2515);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11938);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11939);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32561);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32562);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зол.
pub(crate) fn n2525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2516);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 11940);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 11941);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йол.
pub(crate) fn n2527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2518);
	s.is_empty()
}

/// Words ending -кол.
pub(crate) fn n2528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2519);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11942);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11943);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32563);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 32564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60801);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 60802);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 32565);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60803);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 88418);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109592);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123476);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11944);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11945);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32566);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11946);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32567);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60805);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88419);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 109593);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 123477);
													match s {
														[] => true,
														['о', s @ ..] => super::t36::n133199(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 32568);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 32569);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60806);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32570);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60807);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88421);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109594);
											s.is_empty()
										}
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
					super::push(out, 32571);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32573);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11947);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11948);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11949);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лол.
pub(crate) fn n2529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2520);
	s.is_empty()
}

/// Words ending -мол.
pub(crate) fn n2530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2521);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11950);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11951);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11952);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11953);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32576);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60811);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88422);
									s.is_empty()
								}
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
			super::push(out, 11954);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32577);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60812);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32578);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60813);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88424);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109595);
											s.is_empty()
										}
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
					super::push(out, 32579);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60814);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11955);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32580);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нол.
pub(crate) fn n2531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2522);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11956);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32581);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60816);
							match s {
								[] => true,
								['д', 'о', s @ ..] => {
									super::push(out, 88425);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109596);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123478);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11957);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пол.
pub(crate) fn n2532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2523);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11959);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 11960);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32582);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60817);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88426);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109597);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 11961);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32583);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60818);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88427);
									s.is_empty()
								}
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
			super::push(out, 11962);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32584);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11963);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32585);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 32586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11964);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32587);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 60820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 32588);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11965);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32589);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 32590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11966);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32591);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11967);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32592);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 32593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11968);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32594);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60823);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32595);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 11969);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32596);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60825);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11970);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32597);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рол.
pub(crate) fn n2533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2524);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11971);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11972);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 11973);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 11974);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32599);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сол.
pub(crate) fn n2534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2525);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11975);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32600);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60828);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88430);
									s.is_empty()
								}
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
			super::push(out, 11976);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32601);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60829);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88431);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 109598);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123479);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32602);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 32603);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 32604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32605);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 32606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60831);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 60832);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88432);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109599);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 123480);
													match s {
														[] => true,
														['щ', s @ ..] => super::t36::n133200(s, out),
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
									super::push(out, 88433);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109600);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 123481);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11977);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11978);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тол.
pub(crate) fn n2535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2526);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11979);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32610);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60834);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88434);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109601);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11980);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 11981);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32611);
					match s {
						[] => true,
						['м', 'о', s @ ..] => {
							super::push(out, 60835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11982);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32612);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60836);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88435);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109602);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11983);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32613);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32614);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60837);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32615);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60838);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32616);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60839);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88438);
									s.is_empty()
								}
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
			super::push(out, 11984);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уол.
pub(crate) fn n2536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2527);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11985);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фол.
pub(crate) fn n2537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2528);
	s.is_empty()
}

/// Words ending -хол.
pub(crate) fn n2538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2529);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11986);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 32619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11987);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 32620);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 60840);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88439);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 109603);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123482);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133201(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11988);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32621);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60841);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88440);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109604);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11989);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32622);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60842);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 32623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60843);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 60844);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 32624);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word эмо-герл.
pub(crate) fn n124766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123483);
	s.is_empty()
}

/// Words ending -сомнамбул.
pub(crate) fn n124767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123484);
	s.is_empty()
}

/// Words ending -вельзевул.
pub(crate) fn n124768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123485);
	s.is_empty()
}

/// Words ending -ункул.
pub(crate) fn n32945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32658);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 60866);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 88456);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 109610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 60867);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 88457);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 109611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скул.
pub(crate) fn n12074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12021);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 32659);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 60868);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 88458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109612);
									s.is_empty()
								}
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
			super::push(out, 32660);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лул.
pub(crate) fn n2557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2548);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 12022);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мул.
pub(crate) fn n2558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2549);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12023);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 12024);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32661);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нул.
pub(crate) fn n2559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2550);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12025);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оул.
pub(crate) fn n2560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2551);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 12026);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 12027);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12028);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 12029);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12030);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пул.
pub(crate) fn n2561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2552);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 12031);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12032);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32666);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60870);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88459);
									s.is_empty()
								}
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
			super::push(out, 12033);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32667);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60871);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88460);
									s.is_empty()
								}
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
			super::push(out, 12034);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рул.
pub(crate) fn n2562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2553);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 12035);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 32669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сул.
pub(crate) fn n2563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2554);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12036);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12037);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32671);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12038);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32672);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тул.
pub(crate) fn n2564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2555);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 12039);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32673);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12040);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32674);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60874);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 60875);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88461);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 109613);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 123486);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133202(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12041);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32675);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60876);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88462);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109614);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32676);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 32677);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60877);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88463);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 109615);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12043);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цул.
pub(crate) fn n2565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2556);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 12044);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спиричуэл.
pub(crate) fn n124770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123487);
	s.is_empty()
}

/// Words ending -коре-сарам.
pub(crate) fn n133203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131844);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136758);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ноттингем.
pub(crate) fn n124776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123493);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 131845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -голем.
pub(crate) fn n33065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32777);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 60947);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улем.
pub(crate) fn n12208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12153);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шлем.
pub(crate) fn n12209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12154);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32779);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мем.
pub(crate) fn n2612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2602);
	s.is_empty()
}

/// Words ending -нем.
pub(crate) fn n2613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2603);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 12155);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60948);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оем.
pub(crate) fn n2614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2604);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 12156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рем.
pub(crate) fn n2615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2605);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12157);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12158);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32781);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 32782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12159);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 32785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12160);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32786);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 32787);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 32788);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 12161);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32790);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60953);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88505);
									match s {
										[] => true,
										['а', 'и', s @ ..] => {
											super::push(out, 109631);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123494);
													match s {
														[] => true,
														['л', s @ ..] => super::t36::n133205(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12162);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 12163);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сем.
pub(crate) fn n2616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2606);
	s.is_empty()
}

/// Words ending -тем.
pub(crate) fn n2617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2607);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 12164);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32793);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12165);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12166);
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					super::push(out, 32795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12167);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32796);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60956);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88506);
									s.is_empty()
								}
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
			super::push(out, 12168);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уем.
pub(crate) fn n2618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2608);
	s.is_empty()
}

/// Words ending -фем.
pub(crate) fn n2619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2609);
	s.is_empty()
}

/// Words ending -хем.
pub(crate) fn n2620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12169);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32798);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12170);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32799);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 60959);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88507);
									s.is_empty()
								}
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
			super::push(out, 12171);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32800);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12172);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32801);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъем.
pub(crate) fn n2621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2611);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 12173);
			match s {
				[] => true,
				['о', '-', s @ ..] => {
					super::push(out, 32802);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60962);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88508);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109632);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123495);
													match s {
														[] => true,
														['н', s @ ..] => super::t36::n133206(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12174);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 12175);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыем.
pub(crate) fn n2622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2612);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12176);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32803);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьем.
pub(crate) fn n2623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2613);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 12177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -изм.
pub(crate) fn n2625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2615);
	match s {
		[] => true,
		['а', 'д', s @ ..] => {
			super::push(out, 12181);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32806);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 32807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32808);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32809);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60964);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88510);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109634);
											s.is_empty()
										}
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
					super::push(out, 32810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32811);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32812);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 32813);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88511);
									s.is_empty()
								}
								['к', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109635);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 123496);
													match s {
														[] => true,
														['т', s @ ..] => super::t36::n133207(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 109636);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 109637);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 123497);
													match s {
														[] => true,
														['т', s @ ..] => super::t36::n133208(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 88512);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109638);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123498);
													match s {
														[] => true,
														['р', s @ ..] => super::t36::n133209(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'ф', s @ ..] => {
					super::push(out, 32814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 12182);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32815);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60965);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88513);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109639);
											s.is_empty()
										}
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
					super::push(out, 32816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32817);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 32818);
					match s {
						[] => true,
						['у', 'б', s @ ..] => {
							super::push(out, 60966);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 88514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'о', s @ ..] => {
					super::push(out, 32819);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60967);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88515);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109640);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 123499);
													match s {
														[] => true,
														['е', s @ ..] => super::t36::n133210(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 32820);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32821);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 32822);
					s.is_empty()
				}
				['у', 'н', s @ ..] => {
					super::push(out, 32823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32824);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 32825);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60970);
							s.is_empty()
						}
						['н', 'е', s @ ..] => {
							super::push(out, 60971);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 60972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 12183);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 12184);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 32826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12185);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32827);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 60973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32828);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 32829);
					match s {
						[] => true,
						['о', s @ ..] => super::t36::n61618(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t36::n12245(s, out),
		['м', s @ ..] => super::t36::n12246(s, out),
		['н', s @ ..] => super::t36::n12247(s, out),
		['о', s @ ..] => super::t36::n12248(s, out),
		['п', s @ ..] => super::t36::n12249(s, out),
		['р', s @ ..] => super::t36::n12250(s, out),
		['с', s @ ..] => super::t36::n12251(s, out),
		['т', s @ ..] => super::t36::n12252(s, out),
		['у', s @ ..] => super::t36::n12253(s, out),
		['ф', s @ ..] => super::t36::n12254(s, out),
		['х', s @ ..] => super::t36::n12255(s, out),
		['ц', s @ ..] => super::t36::n12256(s, out),
		['ш', 'а', s @ ..] => super::t36::n12257(s, out),
		_ => false,
	}
}

/// Words ending -ерушалаим.
pub(crate) fn n124819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123535);
	s.is_empty()
}

/// Words ending -абдулазим.
pub(crate) fn n124820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123536);
	s.is_empty()
}

/// Words ending -язим.
pub(crate) fn n12278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12218);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 32899);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61058);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ким.
pub(crate) fn n2633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2623);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12219);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 12220);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32900);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61059);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88591);
									s.is_empty()
								}
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
			super::push(out, 12221);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 12222);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лим.
pub(crate) fn n2634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2624);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12223);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32901);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32902);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61060);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88592);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109688);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 123537);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12224);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 12225);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32904);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12226);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32905);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 32906);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32907);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 32908);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61063);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12227);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12228);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12229);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32911);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мим.
pub(crate) fn n2635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2625);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 12230);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32912);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61066);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88594);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109689);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123538);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 61067);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88595);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109690);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ним.
pub(crate) fn n2636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2626);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12231);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32913);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61068);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88596);
									s.is_empty()
								}
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
			super::push(out, 12232);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12233);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32915);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61070);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88597);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109691);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123539);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32916);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 32917);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61071);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88598);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109692);
											s.is_empty()
										}
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
					super::push(out, 32918);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 32919);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 61073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 32920);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61074);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 61075);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88600);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32921);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61076);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88601);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109693);
											s.is_empty()
										}
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
					super::push(out, 32922);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61077);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88602);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109694);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12234);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32923);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61078);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88603);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109695);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 123540);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пим.
pub(crate) fn n2637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2627);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12235);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рим.
pub(crate) fn n2638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2628);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12236);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32924);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 61079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 12237);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32925);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 32926);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61080);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88604);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109696);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12238);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32927);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32928);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61082);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88605);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109697);
											s.is_empty()
										}
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
					super::push(out, 32929);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 61083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12239);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 32930);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61084);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32931);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32933);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61086);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88607);
									s.is_empty()
								}
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
			super::push(out, 12240);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 32934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12241);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12242);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 12243);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32937);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61087);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88608);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109698);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123541);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 61088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сим.
pub(crate) fn n2639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2629);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12244);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32938);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61089);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32939);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 61090);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 61091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32940);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61092);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88610);
									s.is_empty()
								}
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
			super::push(out, 12245);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32941);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61093);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 61094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12246);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32942);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 61095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32943);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61096);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 88611);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 61097);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88612);
									s.is_empty()
								}
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
			super::push(out, 12247);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32944);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 32945);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61098);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тим.
pub(crate) fn n2640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2630);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12248);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 12249);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32946);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61099);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88614);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109699);
											s.is_empty()
										}
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
					super::push(out, 32947);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61100);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88615);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109700);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 12250);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32948);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61101);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88616);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109701);
											s.is_empty()
										}
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
					super::push(out, 32949);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61102);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88617);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109702);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 123542);
													match s {
														[] => true,
														['д', s @ ..] => super::t36::n133235(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 32950);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61103);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 88618);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109703);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123543);
													match s {
														[] => true,
														['х', s @ ..] => super::t36::n133236(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12251);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32951);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61104);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88619);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12252);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32952);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 32953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12253);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32954);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61106);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88620);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 109704);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123544);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 32955);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уим.
pub(crate) fn n2641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2631);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 12254);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фим.
pub(crate) fn n2642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2632);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12255);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32956);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61107);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88621);
									s.is_empty()
								}
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
			super::push(out, 12256);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12257);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хим.
pub(crate) fn n2643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2633);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12258);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32958);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61108);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88622);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109705);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 123545);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32959);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 32960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12259);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32961);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 61109);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88623);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109706);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12260);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 32962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12261);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12262);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32964);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61110);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88624);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109707);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12263);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32965);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цим.
pub(crate) fn n2644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2634);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 12264);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32966);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61112);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88625);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109708);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12265);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32967);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61113);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88626);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 12266);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32968);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61114);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88627);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чим.
pub(crate) fn n2645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2635);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12267);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32969);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 32970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12268);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 12269);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32971);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шим.
pub(crate) fn n2646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2636);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12270);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32972);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61116);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88628);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109709);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12271);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word хав-тайм.
pub(crate) fn n124830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123546);
	s.is_empty()
}

/// Words ending -райм-тайм.
pub(crate) fn n124831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123547);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 131877);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -завклубом.
pub(crate) fn n124832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123548);
	s.is_empty()
}

/// Words ending -нига-альбом.
pub(crate) fn n124833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123549);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 131878);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 136780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -икток-дом.
pub(crate) fn n124834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123550);
	s.is_empty()
}

/// Words ending -авскладом.
pub(crate) fn n124835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123551);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 131879);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -исправдом.
pub(crate) fn n124836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123552);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 131880);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -деддом.
pub(crate) fn n61782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61130);
	s.is_empty()
}

/// Words ending -оддом.
pub(crate) fn n33292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32992);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 61131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -едом.
pub(crate) fn n12354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12293);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32993);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 32994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61132);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идом.
pub(crate) fn n12355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12294);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 32995);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 61133);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 88639);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109720);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдом.
pub(crate) fn n12356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12295);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32996);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 61134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мдом.
pub(crate) fn n12357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12296);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 32997);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 61135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндом.
pub(crate) fn n12358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12297);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32998);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 61136);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 61137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 32999);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 33000);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 61139);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одом.
pub(crate) fn n12359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12298);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 33001);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 61140);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 88641);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109721);
									s.is_empty()
								}
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
			super::push(out, 33002);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 33003);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61141);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109722);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 61142);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 88643);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109723);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 88644);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109724);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123553);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдом.
pub(crate) fn n12360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12299);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33004);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 61143);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88645);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 61144);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88646);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109726);
									s.is_empty()
								}
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
			super::push(out, 33005);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61145);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тдом.
pub(crate) fn n12361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12300);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33006);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эдом.
pub(crate) fn n12362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12301);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33007);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жом.
pub(crate) fn n2657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2647);
	s.is_empty()
}

/// Words ending -иом.
pub(crate) fn n2658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2648);
	s.is_empty()
}

/// Words ending -йом.
pub(crate) fn n2659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2649);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 12302);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ком.
pub(crate) fn n2660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2650);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12303);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 12304);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 61148);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 61149);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'р', '\u{1}', s @ ..] => {
					super::push(out, 33009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12305);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 12306);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33010);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12307);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12308);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12309);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12310);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33014);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61151);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88650);
									s.is_empty()
								}
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
			super::push(out, 12311);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33015);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61152);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33016);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'а', s @ ..] => {
			super::push(out, 12312);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33017);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88652);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33018);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88653);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', 'о', s @ ..] => {
			super::push(out, 12313);
			match s {
				[] => true,
				['б', 'а', 'р', 'д', s @ ..] => {
					super::push(out, 33020);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61156);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88654);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109727);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лом.
pub(crate) fn n2661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2651);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12314);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12315);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33021);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 33022);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61157);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 88655);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109728);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123554);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133241(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33023);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61158);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88656);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 109729);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 61159);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88657);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109730);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123555);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12316);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 12317);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 12318);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33024);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61160);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88658);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109731);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123556);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33025);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 12319);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12320);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33026);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61162);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88659);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109732);
											s.is_empty()
										}
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
					super::push(out, 33027);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33028);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61164);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88660);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109733);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123557);
													s.is_empty()
												}
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
							super::push(out, 61165);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88661);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109734);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61166);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 61167);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88662);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109735);
											s.is_empty()
										}
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
					super::push(out, 33029);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 61168);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88663);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109736);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12321);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33030);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12322);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 12323);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33031);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ном.
pub(crate) fn n2662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2652);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 12324);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12325);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12326);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33034);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12327);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33035);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 61172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 33036);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 61173);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88664);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88665);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109737);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'а', s @ ..] => {
									match s {
										[] => true,
										['г', '\u{1}', s @ ..] => {
											super::push(out, 109738);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 109739);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 123558);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 61174);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33037);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61175);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оом.
pub(crate) fn n2663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2653);
	s.is_empty()
}

/// Words ending -пом.
pub(crate) fn n2664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2654);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12328);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 12329);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33038);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12330);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33039);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61177);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ром.
pub(crate) fn n2665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2655);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12331);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 12332);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12333);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12334);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 33042);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12335);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33043);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 61179);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33044);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12336);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33045);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61181);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12337);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33047);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61182);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88671);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109740);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 123559);
													match s {
														[] => true,
														['я', s @ ..] => super::t36::n133242(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'р', s @ ..] => {
					super::push(out, 33048);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 61183);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 61184);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88672);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109741);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123560);
													match s {
														[] => true,
														['э', s @ ..] => super::t36::n133243(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12338);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 12339);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сом.
pub(crate) fn n2666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2656);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -том.
pub(crate) fn n2667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2657);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12341);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12342);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33050);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 33051);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61186);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88673);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109742);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33052);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61187);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88674);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 109743);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 61188);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33053);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 61189);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33054);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61190);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 61191);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 88677);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109744);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 123561);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12343);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12344);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33056);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61192);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88678);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 109745);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123562);
													s.is_empty()
												}
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
							super::push(out, 61193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12345);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33057);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 33058);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61194);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 61195);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88679);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109746);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123563);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12346);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33059);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61196);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 33061);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хом.
pub(crate) fn n2668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2658);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12347);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -комбикорм.
pub(crate) fn n124848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123564);
	s.is_empty()
}

/// Words ending -плодокорм.
pub(crate) fn n124849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123565);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 131884);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аниенбаум.
pub(crate) fn n124850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123566);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 131885);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 136783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одиеум.
pub(crate) fn n61877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61224);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 88699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -леум.
pub(crate) fn n12440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12377);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33099);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -неум.
pub(crate) fn n12441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12378);
	match s {
		[] => true,
		['и', 'г', s @ ..] => {
			super::push(out, 33100);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 61225);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 88700);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109756);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123567);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 131886);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жум.
pub(crate) fn n2681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2671);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 12379);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33101);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 33102);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 61226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12380);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 33103);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зум.
pub(crate) fn n2682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2672);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12381);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33104);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 61227);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88701);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 109757);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 61228);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88702);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иум.
pub(crate) fn n2683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2673);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 12382);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33105);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61229);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88703);
									s.is_empty()
								}
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
			super::push(out, 12383);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33106);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61230);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88704);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109758);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 123568);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12384);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33107);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61231);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33108);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 12385);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33110);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12386);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 12387);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 12388);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12389);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 12390);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33112);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кум.
pub(crate) fn n2684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2674);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12391);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12392);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33113);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 33114);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61235);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88706);
									s.is_empty()
								}
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
			super::push(out, 12393);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33115);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61236);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88707);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109759);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 123569);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12394);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33116);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 61237);
							match s {
								[] => true,
								['е', 'т', '-', s @ ..] => {
									super::push(out, 88708);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 109760);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123570);
													match s {
														[] => true,
														['х', s @ ..] => super::t36::n133247(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12395);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12396);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33119);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 61238);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88709);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109761);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 123571);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лум.
pub(crate) fn n2685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2675);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 12397);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 12398);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 12399);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12400);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 12401);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33121);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 33122);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 61239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12402);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33123);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12403);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33124);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61241);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88710);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109762);
											s.is_empty()
										}
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
					super::push(out, 33125);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61242);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88711);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109763);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мум.
pub(crate) fn n2686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2676);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12404);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33126);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 33127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12405);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33128);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61243);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88712);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109764);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 61244);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88713);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109765);
											s.is_empty()
										}
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
					super::push(out, 33129);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61245);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88714);
									s.is_empty()
								}
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
			super::push(out, 12406);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33130);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 33131);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нум.
pub(crate) fn n2687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2677);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12407);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33132);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61246);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88715);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109766);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12408);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12409);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33134);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12410);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12411);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33136);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61248);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88716);
									s.is_empty()
								}
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
			super::push(out, 12412);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 12413);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оум.
pub(crate) fn n2688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2678);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 12414);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33137);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61249);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пум.
pub(crate) fn n2689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2679);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 12415);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33138);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рум.
pub(crate) fn n2690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2680);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 12416);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33139);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61251);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88718);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109767);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123572);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 12417);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33140);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 33141);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 12418);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 12419);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 12420);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 12421);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 12422);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33142);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61253);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88719);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109768);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 123573);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133248(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12423);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12424);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 33144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12425);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 33145);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 61255);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88720);
									s.is_empty()
								}
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
			super::push(out, 12426);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33146);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 33147);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61256);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88721);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109769);
											s.is_empty()
										}
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
					super::push(out, 33148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12427);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33149);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61257);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 88722);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 33150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33151);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 61259);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88723);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33152);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61260);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 61261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сум.
pub(crate) fn n2691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2681);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12428);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 12429);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12430);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33154);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 61262);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88724);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33155);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61263);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88725);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109770);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123574);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12431);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33156);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61264);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88726);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 109771);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 123575);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12432);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33157);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 61265);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88727);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33158);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61266);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88728);
									s.is_empty()
								}
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
			super::push(out, 12433);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33159);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61267);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88729);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109772);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 123576);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тум.
pub(crate) fn n2692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2682);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12434);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 33161);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 33162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12435);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33163);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33164);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12436);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12437);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33166);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 61271);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88730);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109773);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12438);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33167);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33168);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61272);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88731);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 109774);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123577);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12439);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33169);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61273);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88732);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109775);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123578);
													match s {
														[] => true,
														['н', s @ ..] => super::t36::n133249(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33170);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12440);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33171);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уум.
pub(crate) fn n2693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2683);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 12441);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33172);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61276);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88733);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109776);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123579);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133250(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12442);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33173);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 61277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12443);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33174);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61278);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хум.
pub(crate) fn n2694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2684);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 12444);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12445);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33176);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чум.
pub(crate) fn n2695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2685);
	s.is_empty()
}

/// Words ending -шум.
pub(crate) fn n2696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2686);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12446);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 12447);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33177);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61279);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 88735);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109777);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123580);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вильгельм.
pub(crate) fn n124865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123581);
	s.is_empty()
}

/// Words ending -малькольм.
pub(crate) fn n124866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123582);
	s.is_empty()
}

/// Words ending -ьбофиллюм.
pub(crate) fn n124867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123583);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 131891);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тье-костюм.
pub(crate) fn n124868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123584);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 131892);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 136787);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 139707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -абдусалям.
pub(crate) fn n124869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123585);
	s.is_empty()
}

/// Words ending -лбан.
pub(crate) fn n12546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12483);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33218);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 33219);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 61312);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33220);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 61313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 33221);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 61314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 88757);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нбан.
pub(crate) fn n12547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12484);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 33222);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61315);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обан.
pub(crate) fn n12548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12485);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33223);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61316);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 88758);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 88759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 33224);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 33225);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 61317);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбан.
pub(crate) fn n12549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12486);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33226);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 33227);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 61318);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 61319);
					s.is_empty()
				}
				['ч', '\u{1}', s @ ..] => {
					super::push(out, 61320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 33228);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 61321);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тбан.
pub(crate) fn n12550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12487);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33229);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 61322);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 88761);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 109787);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123586);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убан.
pub(crate) fn n12551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12488);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 33230);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61323);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 88762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хбан.
pub(crate) fn n12552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12489);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33231);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 61324);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьбан.
pub(crate) fn n12553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12490);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33232);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 61325);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 88763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ван.
pub(crate) fn n2719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2709);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12491);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12492);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33233);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61326);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 61327);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88765);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109788);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123587);
													s.is_empty()
												}
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
							super::push(out, 61328);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88766);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109789);
											s.is_empty()
										}
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
					super::push(out, 33234);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61329);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33235);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61330);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12493);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 33237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12494);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33238);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33239);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33240);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61334);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88769);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109790);
											s.is_empty()
										}
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
					super::push(out, 33241);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61335);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88770);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109791);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123588);
													match s {
														[] => true,
														['с', s @ ..] => super::t36::n133253(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33242);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 33243);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 61336);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88771);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109792);
											s.is_empty()
										}
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
					super::push(out, 33244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12496);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12497);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'о', 'б', '\u{1}', s @ ..] => {
			super::push(out, 12498);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12499);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 33248);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 33249);
					match s {
						[] => true,
						['о', 'г', s @ ..] => {
							super::push(out, 61339);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 61340);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88773);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109793);
											s.is_empty()
										}
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
					super::push(out, 33250);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61341);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88774);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 109794);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12500);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 12501);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12502);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 12503);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 12504);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 12505);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33254);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61342);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 61343);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88776);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109795);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ган.
pub(crate) fn n2720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2710);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 12506);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33255);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61344);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88777);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109796);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 123589);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 12507);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33256);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61345);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88778);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109797);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123590);
													s.is_empty()
												}
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
							super::push(out, 61346);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88779);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109798);
											s.is_empty()
										}
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
					super::push(out, 33257);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 33258);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61348);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 33259);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61349);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33260);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33261);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33262);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 61351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 12508);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33263);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61352);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88782);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109799);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12509);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33264);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12510);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33265);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33266);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 33267);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61355);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88783);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 109800);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123591);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['э', s @ ..] => {
									super::push(out, 88784);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109801);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 61356);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 88785);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 109802);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 123592);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12511);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33268);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12512);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33269);
					match s {
						[] => true,
						['р', 'а', 'к', '-', s @ ..] => {
							super::push(out, 61358);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88786);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 109803);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 123593);
													match s {
														[] => true,
														['о', s @ ..] => super::t36::n133254(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61359);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 61360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 33271);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 33272);
					match s {
						[] => true,
						['у', 'х', '\u{1}', s @ ..] => {
							super::push(out, 61361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33273);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61362);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33274);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61363);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88788);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109804);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 61364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 33275);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 61365);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88789);
									s.is_empty()
								}
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
			super::push(out, 12513);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33276);
					match s {
						[] => true,
						['р', s @ ..] => super::t36::n62020(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t36::n12577(s, out),
		['м', s @ ..] => super::t36::n12578(s, out),
		['н', s @ ..] => super::t36::n12579(s, out),
		['о', s @ ..] => super::t36::n12580(s, out),
		['р', s @ ..] => super::t36::n12581(s, out),
		['т', s @ ..] => super::t36::n12582(s, out),
		['у', s @ ..] => super::t36::n12583(s, out),
		['ф', s @ ..] => super::t36::n12584(s, out),
		['ы', s @ ..] => super::t36::n12585(s, out),
		['ю', s @ ..] => super::t36::n12586(s, out),
		['я', s @ ..] => super::t36::n12587(s, out),
		_ => false,
	}
}

/// Words ending -дан.
pub(crate) fn n2721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2711);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12525);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12526);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33300);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 33301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 33302);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61394);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88806);
									s.is_empty()
								}
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
			super::push(out, 12527);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33303);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61395);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88807);
									s.is_empty()
								}
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
			super::push(out, 12528);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12530);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33306);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 33307);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61396);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88808);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109811);
											s.is_empty()
										}
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
					super::push(out, 33308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12531);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33309);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61397);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88809);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109812);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123595);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33310);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12532);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33312);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61398);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 61399);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 88810);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109813);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 123596);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33313);
					match s {
						[] => true,
						['б', 'и', s @ ..] => {
							super::push(out, 61400);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88811);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109814);
											s.is_empty()
										}
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
					super::push(out, 33314);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 61401);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еан.
pub(crate) fn n2722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2712);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 12533);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12534);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жан.
pub(crate) fn n2723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2713);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12535);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12536);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33316);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 33317);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 61402);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33318);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61403);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88814);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109815);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12537);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33319);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 61404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 33320);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61405);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33321);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 61406);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33322);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61407);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88817);
									s.is_empty()
								}
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
			super::push(out, 12538);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33323);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12539);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33324);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 61409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12540);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33325);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12541);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33326);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61413);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88818);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109816);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 61414);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88819);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 109817);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'н', '\u{1}', s @ ..] => {
					super::push(out, 33328);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 33329);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 61415);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88820);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109818);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12542);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33330);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 61416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33331);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 12543);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33332);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 61418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зан.
pub(crate) fn n2724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2714);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12544);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33333);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 33334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 33335);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61420);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 61421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33336);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33337);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 61422);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88821);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109819);
											s.is_empty()
										}
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
					super::push(out, 33338);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 33339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12546);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33340);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61423);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88822);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109820);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12547);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33341);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 61424);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88823);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109821);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 88824);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109822);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12548);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33342);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 61425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12549);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12550);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33344);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33345);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12551);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33346);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 61427);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 61428);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88825);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 61429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33347);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 61430);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88826);
									s.is_empty()
								}
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
			super::push(out, 12552);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33348);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 61431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 12553);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 33349);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 61432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

