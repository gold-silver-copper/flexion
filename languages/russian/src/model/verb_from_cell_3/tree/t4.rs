use flexion::Path;

/// Words ending -тришь.
pub(crate) fn n438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2283);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12598);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 12599);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12600);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12601);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12602);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2284);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 6548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12604);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12605);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12606);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12607);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19005);
									s.is_empty()
								}
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
			super::push(out, 2285);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6549);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19006);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19007);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24716);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 19008);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24717);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2286);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6550);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19009);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19010);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 19011);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19012);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24718);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2287);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6551);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19013);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19014);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19015);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 19016);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19017);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24719);
											s.is_empty()
										}
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
					super::push(out, 6552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12608);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12609);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 12610);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 12611);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19018);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19019);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 19020);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19021);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24720);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12612);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12613);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12614);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6553);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							super::push(out, 12615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6554);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19023);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 19024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уришь.
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2288);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6555);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6556);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6557);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6558);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6559);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12619);
							match s {
								[] => true,
								['л', 'а', 'к', 'с', s @ ..] => {
									super::push(out, 19025);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 12620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12621);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6561);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6562);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2289);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6563);
					match s {
						[] => true,
						['л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19026);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19027);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19028);
									s.is_empty()
								}
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
			super::push(out, 2290);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6564);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6565);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12625);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 12626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6566);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6567);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6568);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6569);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12628);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19029);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2291);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6571);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6572);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6573);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12631);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19030);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19031);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 19032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6574);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6575);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2292);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6576);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6577);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2293);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6578);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6579);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12635);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 12636);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 12637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6580);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 6581);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6582);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6583);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6584);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6585);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12639);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 19034);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 24721);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29213);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 29214);
													s.is_empty()
												}
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
							super::push(out, 12640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6586);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12641);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6587);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 6588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12642);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6589);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2294);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6590);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12645);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24722);
											s.is_empty()
										}
										['т', s @ ..] => super::t6::n26188(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => super::t6::n6887(s, out),
				['х', s @ ..] => super::t6::n6888(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t6::n2341(s, out),
		['п', s @ ..] => super::t6::n2342(s, out),
		['т', s @ ..] => super::t6::n2343(s, out),
		['ш', s @ ..] => super::t6::n2344(s, out),
		['щ', s @ ..] => super::t6::n2345(s, out),
		_ => false,
	}
}

/// Words ending -фришь.
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2300);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12669);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хришь.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2301);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2302);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12671);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12672);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12673);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12674);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19043);
									s.is_empty()
								}
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
			super::push(out, 2303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6612);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6613);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2304);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6614);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щришь.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыришь.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2306);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6615);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6616);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2307);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6617);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12677);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 19044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2308);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6618);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6619);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19045);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19046);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24727);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29216);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2309);
			match s {
				[] => true,
				['ш', '\u{1}', s @ ..] => {
					super::push(out, 6620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2310);
			match s {
				[] => true,
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12679);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12680);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6621);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6622);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6623);
					s.is_empty()
				}
				['с', 'а', 'м', '\u{1}', s @ ..] => {
					super::push(out, 6624);
					s.is_empty()
				}
				['ш', '\u{1}', s @ ..] => {
					super::push(out, 6625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юришь.
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 443);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2312);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6626);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 12682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2313);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 2314);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2315);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2316);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12683);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12684);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яришь.
pub(crate) fn n445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2317);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2318);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6630);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2319);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6631);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 6632);
					match s {
						[] => true,
						['и', 'ф', 'с', s @ ..] => {
							super::push(out, 12687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6633);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12688);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24728);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24729);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12689);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 19048);
									s.is_empty()
								}
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
			super::push(out, 2320);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2321);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 6634);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 2322);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2323);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6636);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -заколбасишь.
pub(crate) fn n30991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29217);
	s.is_empty()
}

/// Words ending -наколбасишь.
pub(crate) fn n30992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29218);
	s.is_empty()
}

/// Words ending -красишь.
pub(crate) fn n6950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6653);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12711);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12712);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 12713);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19054);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12714);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 12715);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 12716);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12717);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 12718);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word пидорасишь.
pub(crate) fn n20132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19057);
	s.is_empty()
}

/// Words ending -опидорасишь.
pub(crate) fn n20133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19058);
	s.is_empty()
}

/// Words ending -спидорасишь.
pub(crate) fn n20134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19059);
	s.is_empty()
}

/// Words ending -тпидорасишь.
pub(crate) fn n20135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19060);
	s.is_empty()
}

/// Words ending -уасишь.
pub(crate) fn n2378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2332);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 6655);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -есишь.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2333);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6656);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6657);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12720);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6658);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6659);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6660);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6661);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2334);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6662);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6663);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6664);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6665);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6666);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2335);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6667);
					match s {
						[] => true,
						['к', 'а', s @ ..] => {
							super::push(out, 12724);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 12725);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19063);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19064);
									s.is_empty()
								}
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
			super::push(out, 2336);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6668);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12726);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12727);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 19065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6669);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6670);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6671);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12730);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19066);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19067);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19068);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12731);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 19070);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29219);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 29220);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2337);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6672);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6673);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6674);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6675);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2338);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6676);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -исишь.
pub(crate) fn n448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2339);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6677);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6678);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6679);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йсишь.
pub(crate) fn n449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2341);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6681);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ксишь.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2342);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6682);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2343);
			match s {
				[] => true,
				['л', 'ф', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12736);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2344);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 6684);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12738);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2345);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2346);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12740);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мсишь.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2347);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нсишь.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2348);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2349);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6688);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19071);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19072);
									s.is_empty()
								}
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
			super::push(out, 2350);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6689);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осишь.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2351);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6690);
					match s {
						[] => true,
						['у', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19073);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19074);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24735);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 19075);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24736);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 24737);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29221);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6691);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19076);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19077);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24738);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2352);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6692);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6693);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6694);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6695);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6696);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12743);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 12744);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 19078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6697);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6698);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2353);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6699);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12747);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19079);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 24739);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 19080);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 24740);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 12748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19081);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19082);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 24741);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 29222);
													s.is_empty()
												}
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
							super::push(out, 12749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2354);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6700);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19083);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 19084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6701);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 12750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6702);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6703);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6704);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6705);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12751);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12752);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24742);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 24743);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29223);
													s.is_empty()
												}
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
							super::push(out, 12753);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6706);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6707);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19086);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 19087);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 24744);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 24745);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => super::t6::n20164(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => super::t6::n13491(s, out),
						['р', s @ ..] => super::t6::n13492(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t6::n7007(s, out),
				['т', s @ ..] => super::t6::n7008(s, out),
				['у', s @ ..] => super::t6::n7009(s, out),
				['ы', s @ ..] => super::t6::n7010(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t6::n2401(s, out),
		['с', s @ ..] => super::t6::n2402(s, out),
		_ => false,
	}
}

/// Words ending -псишь.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 2357);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12777);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсишь.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2358);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2359);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12779);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12780);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6719);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12781);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 19113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 6720);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12782);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12783);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12784);
							match s {
								[] => true,
								['д', 'е', s @ ..] => {
									super::push(out, 19114);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 24756);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссишь.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2360);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6721);
					match s {
						[] => true,
						['о', 'л', 'г', 'о', s @ ..] => {
							super::push(out, 12786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6722);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усишь.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 456);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 2361);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6723);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 6724);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6725);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6726);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6727);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6728);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12788);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6729);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6730);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6731);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12790);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2362);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12791);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12792);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6733);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2363);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6734);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6735);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6736);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12796);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12797);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 12798);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19115);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12799);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12800);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 12801);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12802);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12803);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19118);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 12804);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12805);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19119);
									s.is_empty()
								}
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
			super::push(out, 2364);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6738);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6739);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысишь.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2365);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2366);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6741);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6742);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6743);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6744);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12808);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6745);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эсишь.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 2367);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 6746);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word виноватишь.
pub(crate) fn n31000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29226);
	s.is_empty()
}

/// Words ending -бвиноватишь.
pub(crate) fn n31001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29227);
	s.is_empty()
}

/// Words ending -выгатишь.
pub(crate) fn n13554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12817);
	s.is_empty()
}

/// Words ending -датишь.
pub(crate) fn n2417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2371);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6755);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иатишь.
pub(crate) fn n2418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2372);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 6756);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12818);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -катишь.
pub(crate) fn n2419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2373);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6757);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6758);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6759);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6760);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6761);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12819);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -латишь.
pub(crate) fn n2420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2374);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6762);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 12820);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12821);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6764);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12823);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12824);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 12825);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6765);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12826);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24757);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24758);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29228);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6766);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12827);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -матишь.
pub(crate) fn n2421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2375);
	match s {
		[] => true,
		['с', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12828);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 12829);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19129);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24759);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 19130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6767);
			match s {
				[] => true,
				['о', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19131);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 19132);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24760);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 24761);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 24762);
									s.is_empty()
								}
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
			super::push(out, 6768);
			match s {
				[] => true,
				['о', 'л', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19133);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19134);
							s.is_empty()
						}
						['с', 'и', s @ ..] => {
							super::push(out, 19135);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 19136);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -натишь.
pub(crate) fn n2422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2376);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6769);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19137);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19138);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 24765);
									s.is_empty()
								}
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
			super::push(out, 6770);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6771);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12831);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24766);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 24767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оатишь.
pub(crate) fn n2423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2377);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6772);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 12832);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -патишь.
pub(crate) fn n2424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2378);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6773);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19139);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 19140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12834);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24768);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 24769);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29229);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 24770);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29230);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 24771);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29231);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24772);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29232);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 24773);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 24774);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 24775);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24776);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29233);
											s.is_empty()
										}
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
					super::push(out, 12835);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ратишь.
pub(crate) fn n2425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2379);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6774);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6775);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6776);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12837);
					match s {
						[] => true,
						['в', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24777);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24778);
									s.is_empty()
								}
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
			super::push(out, 6777);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12838);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19142);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6778);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12839);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 12840);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19145);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 24779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 19146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6779);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12841);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12842);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12843);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19147);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 19148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12844);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19149);
							s.is_empty()
						}
						['а', 'р', 'д', s @ ..] => {
							super::push(out, 19150);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24780);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 19151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6780);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12846);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6781);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12847);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хатишь.
pub(crate) fn n2426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2380);
	match s {
		[] => true,
		['о', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12848);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 6782);
			match s {
				[] => true,
				['р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19153);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19154);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гтишь.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 460);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2381);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12850);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етишь.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 461);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2382);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 2383);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6783);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6784);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2384);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6786);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12853);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12854);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12855);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12856);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12857);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12858);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19157);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 19158);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24781);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 19159);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 24782);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12859);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12860);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12861);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6788);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 6789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12863);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 12864);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12865);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12866);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19162);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 19163);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 24783);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12867);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12868);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19165);
									s.is_empty()
								}
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
			super::push(out, 2385);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2386);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6791);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6792);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 6793);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6794);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12869);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6795);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6796);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12871);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6797);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6798);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12873);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 19167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6799);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2387);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6800);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6801);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6802);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6803);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6804);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2388);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6805);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 12876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6806);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19168);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 19169);
									s.is_empty()
								}
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
			super::push(out, 2389);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6807);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 6808);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2390);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6809);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 12880);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6810);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6811);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12882);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12883);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6812);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6813);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2391);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6814);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t6::n2438(s, out),
		['ф', s @ ..] => super::t6::n2439(s, out),
		['ш', s @ ..] => super::t6::n2440(s, out),
		_ => false,
	}
}

/// Words ending -итишь.
pub(crate) fn n463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 462);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2395);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12890);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12891);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 12892);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19177);
									s.is_empty()
								}
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
			super::push(out, 2396);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6820);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12893);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19178);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6821);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12894);
							match s {
								[] => true,
								['о', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24791);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24792);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2397);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2398);
			match s {
				[] => true,
				['г', 'а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12895);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12896);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12897);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19180);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 12898);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12899);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2399);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6823);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6824);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6825);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12901);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12902);
							match s {
								[] => true,
								['в', 'д', s @ ..] => {
									super::push(out, 19182);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 24793);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													super::push(out, 29235);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2400);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йтишь.
pub(crate) fn n464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 463);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2401);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6827);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ктишь.
pub(crate) fn n465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 464);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2403);
			match s {
				[] => true,
				['ф', '\u{1}', s @ ..] => {
					super::push(out, 6829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2404);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6830);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтишь.
pub(crate) fn n466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 465);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2405);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12903);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12904);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12905);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19183);
									s.is_empty()
								}
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
			super::push(out, 2406);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12906);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19184);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 12907);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтишь.
pub(crate) fn n467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 466);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2407);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6831);
					match s {
						[] => true,
						['а', 'т', '\u{1}', s @ ..] => {
							super::push(out, 12909);
							s.is_empty()
						}
						['ф', '\u{1}', s @ ..] => {
							super::push(out, 12910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2408);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6832);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2409);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12912);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12913);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 12914);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19186);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12915);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19187);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 19188);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24794);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12916);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12917);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12918);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6834);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 6835);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12920);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12921);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отишь.
pub(crate) fn n468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 467);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2410);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6836);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12923);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19191);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12924);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6837);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 12925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2411);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6838);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 12926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6839);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 12927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2412);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6840);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6841);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 6842);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19194);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19195);
									s.is_empty()
								}
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
			super::push(out, 2413);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6843);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6844);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2414);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6845);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6846);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 12930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6847);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 19197);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 19198);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 24795);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2415);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6848);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12931);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12932);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19199);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 19200);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24796);
											s.is_empty()
										}
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
					super::push(out, 6849);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12933);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19201);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19202);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24797);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 12934);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19203);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 19204);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19205);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19206);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24798);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 12935);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19207);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19208);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19209);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19210);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19211);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24799);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 12936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19212);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19213);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19214);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24800);
											s.is_empty()
										}
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
					super::push(out, 6850);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12937);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12938);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12939);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19215);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2416);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6852);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2417);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6853);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t6::n2464(s, out),
		['с', s @ ..] => {
			super::push(out, 2419);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6858);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => super::t6::n7165(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t6::n2466(s, out),
		['ш', s @ ..] => super::t6::n2467(s, out),
		_ => false,
	}
}

/// Words ending -птишь.
pub(crate) fn n469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 468);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2422);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6862);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12958);
							match s {
								[] => true,
								['и', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24810);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24811);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2423);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12959);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12960);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12961);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12962);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртишь.
pub(crate) fn n470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 469);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2424);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 6863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2425);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12964);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12965);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12966);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19234);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 24812);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 19235);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 24813);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29239);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 29240);
													s.is_empty()
												}
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
							super::push(out, 12967);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12968);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19236);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24814);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12969);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19237);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6865);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12970);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 19238);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 6866);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12971);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12972);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12973);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19239);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12974);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12975);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19240);
									s.is_empty()
								}
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
			super::push(out, 2426);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12976);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12977);
							s.is_empty()
						}
						['е', 'р', '\u{1}', s @ ..] => {
							super::push(out, 12978);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 6868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2427);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6869);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12980);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 19241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стишь.
pub(crate) fn n471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 470);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2428);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6870);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6871);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12981);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12982);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19242);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12983);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19243);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 12984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6872);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6873);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19245);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19246);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6874);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12986);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19247);
									match s {
										[] => true,
										['о', 'к', '\u{1}', s @ ..] => {
											super::push(out, 24815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12987);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19248);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12988);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12989);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12990);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12991);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12992);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19249);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24816);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 19250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12993);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 6876);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2429);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6877);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12995);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12996);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12997);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 19252);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24817);
											match s {
												[] => true,
												['л', 'б', '\u{1}', s @ ..] => {
													super::push(out, 29241);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 19253);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24818);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 19254);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24819);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24820);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 24821);
											s.is_empty()
										}
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
					super::push(out, 6878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6879);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19255);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19256);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 12999);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24822);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24823);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24824);
											s.is_empty()
										}
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
					super::push(out, 6880);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 13000);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19258);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6881);
					match s {
						[] => true,
						['к', s @ ..] => super::t6::n13749(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13002);
							s.is_empty()
						}
						['о', s @ ..] => super::t6::n13751(s, out),
						_ => false,
					}
				}
				['ч', s @ ..] => super::t6::n7190(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t6::n2476(s, out),
		['к', s @ ..] => {
			super::push(out, 2431);
			s.is_empty()
		}
		['л', s @ ..] => super::t6::n2478(s, out),
		['м', s @ ..] => super::t6::n2479(s, out),
		['о', s @ ..] => super::t6::n2480(s, out),
		['р', s @ ..] => super::t6::n2481(s, out),
		['у', s @ ..] => super::t6::n2482(s, out),
		['ь', s @ ..] => super::t6::n2483(s, out),
		_ => false,
	}
}

/// Words ending -утишь.
pub(crate) fn n472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 471);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2438);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 2439);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6907);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6908);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6909);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13052);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 13053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6910);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2440);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6911);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6912);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2441);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6914);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6915);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13055);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 13056);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19293);
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29254);
													s.is_empty()
												}
												['а', s @ ..] => {
													match s {
														[] => true,
														['з', s @ ..] => super::t6::n33840(s, out),
														['н', s @ ..] => super::t6::n33841(s, out),
														_ => false,
													}
												}
												['е', s @ ..] => {
													super::push(out, 29255);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 29256);
													s.is_empty()
												}
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
							super::push(out, 13057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6916);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6917);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6918);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13060);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 13061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6919);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6920);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2442);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6921);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2443);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6922);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13064);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13065);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13066);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13067);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13068);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 6923);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13069);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19295);
									s.is_empty()
								}
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
			super::push(out, 2444);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6924);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13070);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13071);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19296);
									s.is_empty()
								}
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
			super::push(out, 2445);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6925);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6926);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6927);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6928);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6929);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 2446);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6930);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фтишь.
pub(crate) fn n473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 472);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2447);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2448);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6932);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13073);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19297);
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 19298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 6933);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2449);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 2450);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6935);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтишь.
pub(crate) fn n474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2451);
			match s {
				[] => true,
				['р', 'а', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13075);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6936);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2452);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2453);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13078);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 2454);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13080);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13081);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чтишь.
pub(crate) fn n475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2455);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2456);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6940);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытишь.
pub(crate) fn n476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 475);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 2457);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13083);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13084);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19299);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2458);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6941);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6942);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютишь.
pub(crate) fn n477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2459);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2460);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2461);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 2462);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2463);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятишь.
pub(crate) fn n478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 477);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2464);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13086);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2465);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6949);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6950);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 6951);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6952);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6953);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13089);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19300);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19301);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19302);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19303);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19304);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24838);
											s.is_empty()
										}
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
					super::push(out, 6954);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13090);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13091);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6955);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6956);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6957);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едачишь.
pub(crate) fn n7288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6972);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19316);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19317);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елдачишь.
pub(crate) fn n13866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13114);
	s.is_empty()
}

/// Words ending -рдачишь.
pub(crate) fn n7290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6974);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13115);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удачишь.
pub(crate) fn n7291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6975);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19318);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', '\u{1}', s @ ..] => {
			super::push(out, 13116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жачишь.
pub(crate) fn n2524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2477);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6976);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зачишь.
pub(crate) fn n2525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2478);
	match s {
		[] => true,
		['а', 'к', '\u{1}', s @ ..] => {
			super::push(out, 6977);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лачишь.
pub(crate) fn n2526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2479);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6978);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6979);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 6980);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13118);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24843);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24844);
									s.is_empty()
								}
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
			super::push(out, 6981);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6982);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 13120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6983);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13121);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19320);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мачишь.
pub(crate) fn n2527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2480);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6984);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 6985);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13122);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24845);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24846);
									s.is_empty()
								}
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
			super::push(out, 6986);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 6987);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 6988);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -начишь.
pub(crate) fn n2528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2481);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6989);
			match s {
				[] => true,
				['р', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19322);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6990);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13125);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13126);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6991);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13128);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пачишь.
pub(crate) fn n2529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2482);
	match s {
		[] => true,
		['л', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13129);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13130);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рачишь.
pub(crate) fn n2530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2483);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6992);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6993);
			match s {
				[] => true,
				['е', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19324);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6994);
			match s {
				[] => true,
				['х', '\u{1}', s @ ..] => {
					super::push(out, 13131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6995);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13132);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6996);
			match s {
				[] => true,
				['а', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19326);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19327);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6997);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19329);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сачишь.
pub(crate) fn n2531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2484);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6998);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6999);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 7000);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тачишь.
pub(crate) fn n2532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2485);
	match s {
		[] => true,
		['н', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13136);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7001);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13138);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13139);
					match s {
						[] => true,
						['п', 'а', s @ ..] => {
							super::push(out, 19331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'а', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13140);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 13141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7002);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хачишь.
pub(crate) fn n2533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2486);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7003);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13142);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шачишь.
pub(crate) fn n2534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2487);
	match s {
		[] => true,
		['и', '\u{1}', s @ ..] => {
			super::push(out, 7004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бчишь.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 485);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2488);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7005);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вчишь.
pub(crate) fn n487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 486);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2489);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13143);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13144);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19332);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гчишь.
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 487);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2490);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13146);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13147);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13148);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19334);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13149);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19335);
									s.is_empty()
								}
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
			super::push(out, 2491);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13150);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13151);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 7006);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ечишь.
pub(crate) fn n489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 488);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2492);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7007);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13153);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2493);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7009);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7010);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 13156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19337);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19338);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7011);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7012);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7013);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7014);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7015);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2494);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2495);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2496);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2497);
			match s {
				[] => true,
				['е', 'п', 'о', s @ ..] => {
					super::push(out, 7019);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', 'и', 'т', 'о', 'р', 'п', 'а', s @ ..] => {
							super::push(out, 13163);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19340);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 13164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13165);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13166);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ичишь.
pub(crate) fn n490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2498);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2499);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 7021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7022);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7023);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 7024);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13168);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 13169);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7025);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13171);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13172);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 19344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7026);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7027);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 13173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7028);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7029);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2501);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7031);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19345);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19346);
									s.is_empty()
								}
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
			super::push(out, 2502);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7032);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19347);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7033);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2503);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7034);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2504);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13177);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13178);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13179);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13180);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13181);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 7037);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13182);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19350);
									s.is_empty()
								}
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
			super::push(out, 2505);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7038);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 2506);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7039);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 13184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кчишь.
pub(crate) fn n491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 490);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2507);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7040);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лчишь.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 491);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2508);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13185);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13186);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13187);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13188);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мчишь.
pub(crate) fn n493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 492);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2509);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2510);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2511);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2512);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7041);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчишь.
pub(crate) fn n494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 493);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2513);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7042);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7043);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13189);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24847);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 24848);
											s.is_empty()
										}
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
					super::push(out, 7044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2514);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7045);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19352);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19353);
									s.is_empty()
								}
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
			super::push(out, 2515);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7046);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7047);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13190);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 13191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2516);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 2517);
			match s {
				[] => true,
				['л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13192);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13193);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13194);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13195);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13196);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13197);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13198);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13199);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очишь.
pub(crate) fn n495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 494);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2518);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7050);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7051);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13201);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24849);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24850);
											s.is_empty()
										}
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
					super::push(out, 7052);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2519);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2520);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7054);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 7055);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13204);
							match s {
								[] => true,
								['о', 'п', 'у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24851);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 24852);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 24853);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2521);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7056);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13205);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 19359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7057);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13206);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13207);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 13208);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13209);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 19360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13210);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19361);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24854);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29257);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2522);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7058);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 7059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13211);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13212);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13213);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7060);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 13215);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 13216);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19362);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 19363);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 19364);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19365);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24855);
											s.is_empty()
										}
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
					super::push(out, 7061);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13218);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7063);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13220);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19366);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24856);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 19367);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19368);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24857);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2523);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7064);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7065);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7066);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7067);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7068);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7069);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13222);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 19369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7070);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7071);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7072);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2524);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7073);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7074);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t6::n2572(s, out),
		['с', s @ ..] => {
			super::push(out, 2526);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7081);
					match s {
						[] => true,
						['о', s @ ..] => super::t6::n14003(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t6::n7409(s, out),
				['ы', s @ ..] => super::t6::n7410(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t6::n2574(s, out),
		_ => false,
	}
}

/// Words ending -пчишь.
pub(crate) fn n496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 495);
	match s {
		[] => true,
		['а', 'р', 'а', 'х', '\u{1}', s @ ..] => {
			super::push(out, 2528);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рчишь.
pub(crate) fn n497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 496);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2529);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7096);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13266);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13267);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2530);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7097);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 7098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13269);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2531);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13271);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13272);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 13273);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7100);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13275);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13277);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13278);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 13279);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13280);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13281);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13282);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13283);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2532);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7103);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7104);
					s.is_empty()
				}
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13285);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13286);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13288);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13289);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13291);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7105);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7106);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13293);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19400);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 7107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2533);
			match s {
				[] => true,
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13294);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тчишь.
pub(crate) fn n498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 497);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2534);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2535);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7109);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учишь.
pub(crate) fn n499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 498);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2536);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2537);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7110);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7111);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13296);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2538);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7112);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7113);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7114);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 13298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7115);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 7116);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13300);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 19401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7117);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13301);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13302);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 13303);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 19402);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24870);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24871);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7118);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 13305);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 19403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7119);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7120);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 13307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7121);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2539);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13309);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13310);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 13311);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13312);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 19405);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 19406);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 19407);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 19408);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24872);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2540);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2541);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7124);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7125);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2542);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2543);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7127);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2544);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7129);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 7130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2545);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2546);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7132);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7133);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2547);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7135);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7136);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 7137);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13316);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 19409);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7138);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 7139);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7140);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7141);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7142);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7144);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7145);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7146);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7147);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7148);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2549);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2550);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7150);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7151);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7152);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7153);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2551);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7154);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 7155);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7156);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7157);
					match s {
						[] => true,
						['е', s @ ..] => super::t6::n14083(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t6::n7490(s, out),
				['о', s @ ..] => super::t6::n7491(s, out),
				['ы', s @ ..] => super::t6::n7492(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t6::n2599(s, out),
		['т', s @ ..] => super::t6::n2600(s, out),
		['щ', s @ ..] => super::t6::n2601(s, out),
		['ы', s @ ..] => super::t6::n2602(s, out),
		['я', s @ ..] => super::t6::n2603(s, out),
		_ => false,
	}
}

/// Words ending -ычишь.
pub(crate) fn n500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 499);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2557);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7177);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7178);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2558);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7180);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 13337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2559);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7182);
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19414);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 19415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2560);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2561);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7185);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7186);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2562);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7188);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7189);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7190);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7191);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 13338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьчишь.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 500);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2563);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13339);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2564);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 7192);
					match s {
						[] => true,
						['н', 'ы', s @ ..] => {
							super::push(out, 13341);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючишь.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 501);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2565);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2566);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13342);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13343);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7194);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13345);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13346);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13347);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19417);
									s.is_empty()
								}
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
			super::push(out, 2567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7195);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19418);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19419);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19420);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19421);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24873);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2568);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13348);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13350);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13351);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 13352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 7198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2569);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2570);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячишь.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 502);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2571);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2572);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13354);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 13355);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13356);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2573);
			match s {
				[] => true,
				['з', 'и', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13358);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13359);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2574);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7201);
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 7202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2575);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7203);
					match s {
						[] => true,
						['х', '\u{1}', s @ ..] => {
							super::push(out, 13361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2576);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7204);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['в', 'с', 'а', s @ ..] => {
							super::push(out, 13362);
							s.is_empty()
						}
						['с', 'о', s @ ..] => {
							super::push(out, 13363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7205);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2577);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7207);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7208);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7209);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13366);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19422);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 19423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 13367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19424);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 19425);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19426);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19427);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 13368);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19428);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7210);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13369);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19430);
									match s {
										[] => true,
										['м', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29260);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 29261);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 19431);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 24874);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2578);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13370);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2579);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7211);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13372);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13373);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13374);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13375);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 19432);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2580);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7212);
					s.is_empty()
				}
				['н', 'а', 'м', 'о', s @ ..] => {
					super::push(out, 7213);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -корешишь.
pub(crate) fn n14159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13396);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19443);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19444);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word порешишь.
pub(crate) fn n20539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19445);
	s.is_empty()
}

/// Words ending -трешишь.
pub(crate) fn n7575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7238);
	s.is_empty()
}

/// Words ending -ырешишь.
pub(crate) fn n7576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7239);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13398);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тешишь.
pub(crate) fn n2637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2590);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7240);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7241);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 7242);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7243);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7244);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 7245);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13400);
					match s {
						[] => true,
						['е', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24876);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24877);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ишишь.
pub(crate) fn n507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 506);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 2591);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2592);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7247);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2593);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кшишь.
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 507);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2594);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7248);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мшишь.
pub(crate) fn n509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 508);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2595);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2596);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7249);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7250);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 7251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2597);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2598);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ошишь.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 509);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2599);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7252);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13402);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13403);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13404);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 13405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2600);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7254);
					match s {
						[] => true,
						['и', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19448);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19449);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24879);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2601);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2602);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7256);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7257);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19450);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19451);
									s.is_empty()
								}
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
			super::push(out, 2603);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7258);
					match s {
						[] => true,
						['о', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19452);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19453);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19454);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19455);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24880);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2604);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7259);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19456);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19457);
									s.is_empty()
								}
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
			super::push(out, 2605);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7260);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7261);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2606);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7262);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 13408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7263);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13409);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13410);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 13411);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7264);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13412);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13413);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13414);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13415);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 19460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13416);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7265);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13417);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19462);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 19463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 13418);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19464);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 13419);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19466);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 24881);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13420);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19467);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7266);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13421);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24882);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 24883);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29262);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 24884);
											match s {
												[] => true,
												['а', s @ ..] => super::t6::n31038(s, out),
												_ => false,
											}
										}
										['ы', s @ ..] => super::t6::n26358(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t6::n2654(s, out),
		_ => false,
	}
}

/// Words ending -ршишь.
pub(crate) fn n511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 510);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2608);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13423);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 19473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19474);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19475);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19476);
									s.is_empty()
								}
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
			super::push(out, 2609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7270);
					s.is_empty()
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13424);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13425);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13426);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13427);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13428);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13429);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19478);
									s.is_empty()
								}
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
			super::push(out, 2610);
			match s {
				[] => true,
				['б', 'е', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13430);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13431);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13433);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13434);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13435);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ушишь.
pub(crate) fn n512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 511);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2611);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2612);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7272);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2613);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7273);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2614);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7274);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7275);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7276);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7277);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13439);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13440);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 19479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7278);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7279);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2615);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7280);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 13442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2616);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13443);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13444);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13445);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13446);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13447);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 19480);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 19481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13448);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7282);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19483);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19484);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19485);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19486);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24886);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2617);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2618);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7284);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2619);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7285);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13450);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 13451);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 13452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7286);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 7287);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7288);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7289);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7290);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7291);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7292);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 7293);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7294);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13457);
							match s {
								[] => true,
								['р', 'у', s @ ..] => {
									super::push(out, 19487);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 24887);
											s.is_empty()
										}
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
					super::push(out, 7295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13458);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7296);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2621);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7297);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7298);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7299);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7300);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7301);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13461);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19488);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24888);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2622);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7302);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7303);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7304);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13462);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19489);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 19490);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24889);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13463);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 19491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7305);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7306);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13464);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чшишь.
pub(crate) fn n513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 512);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 2623);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышишь.
pub(crate) fn n514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 513);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2624);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7308);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7309);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2625);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13465);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13466);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19493);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13467);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13468);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19494);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 13469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2626);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 2627);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьшишь.
pub(crate) fn n515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 514);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2628);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7313);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 13470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7314);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2629);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7315);
					match s {
						[] => true,
						['м', 'у', 'о', s @ ..] => {
							super::push(out, 13472);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юшишь.
pub(crate) fn n516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 515);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 2630);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2631);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7317);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13473);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яшишь.
pub(crate) fn n517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 516);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 2632);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7318);
					match s {
						[] => true,
						['т', 'в', s @ ..] => {
							super::push(out, 13475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2633);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2634);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word вытаращишь.
pub(crate) fn n31040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29265);
	s.is_empty()
}

/// Words ending -еборщишь.
pub(crate) fn n14259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13487);
	s.is_empty()
}

/// Words ending -оборщишь.
pub(crate) fn n14260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13488);
	s.is_empty()
}

/// Words ending -морщишь.
pub(crate) fn n7684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7341);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13489);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13490);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19504);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 19505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13491);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13492);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 13493);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13494);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 13495);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19507);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -порщишь.
pub(crate) fn n7685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7342);
	match s {
		[] => true,
		['о', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19508);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19509);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 19510);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тщишь.
pub(crate) fn n524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 523);
	s.is_empty()
}

/// Words ending -ущишь.
pub(crate) fn n525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 524);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7343);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7344);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 7345);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7346);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7347);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7348);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7349);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7350);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7351);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ющишь.
pub(crate) fn n526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 525);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2647);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13497);
							s.is_empty()
						}
						['а', 'з', s @ ..] => {
							super::push(out, 13498);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13499);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13500);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13501);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13502);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19511);
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 19512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ящишь.
pub(crate) fn n527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 526);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2648);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ешься.
pub(crate) fn n531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 528);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2654);
			s.is_empty()
		}
		['а', s @ ..] => super::t6::n2702(s, out),
		['б', s @ ..] => {
			super::push(out, 2656);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13689);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13690);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13691);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 25619);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 25620);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 25621);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 29844);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 25622);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 25623);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 25624);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 29845);
													s.is_empty()
												}
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
							super::push(out, 13692);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 19958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 13693);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13694);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7375);
					match s {
						[] => true,
						['ш', 'ы', s @ ..] => {
							super::push(out, 13695);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19960);
									s.is_empty()
								}
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
			super::push(out, 2657);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7376);
					match s {
						[] => true,
						['р', 'ы', s @ ..] => {
							super::push(out, 13696);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7377);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 13697);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 13698);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 19962);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 25625);
											s.is_empty()
										}
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
					super::push(out, 7378);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19963);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 19964);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19965);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 19966);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 19967);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 25626);
											s.is_empty()
										}
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
					super::push(out, 7379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13699);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13700);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13701);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 19968);
									match s {
										[] => true,
										['о', 'о', s @ ..] => {
											super::push(out, 25627);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29846);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 19969);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 19970);
									match s {
										[] => true,
										['о', 'о', s @ ..] => {
											super::push(out, 25628);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29847);
													s.is_empty()
												}
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
							super::push(out, 13702);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t6::n2705(s, out),
		['е', s @ ..] => super::t6::n2706(s, out),
		['ж', s @ ..] => super::t6::n2707(s, out),
		['з', s @ ..] => {
			super::push(out, 2661);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7408);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13762);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20064);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 20065);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 20066);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 25663);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 13763);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 20067);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 25664);
											s.is_empty()
										}
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
					super::push(out, 7409);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 7410);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13764);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 20068);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7411);
					match s {
						[] => true,
						['р', 'г', '\u{1}', s @ ..] => {
							super::push(out, 13765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t6::n2709(s, out),
		['к', s @ ..] => super::t6::n2710(s, out),
		['л', s @ ..] => super::t6::n2711(s, out),
		['м', s @ ..] => super::t6::n2712(s, out),
		['н', s @ ..] => super::t6::n2713(s, out),
		['о', s @ ..] => super::t6::n2714(s, out),
		['р', s @ ..] => super::t6::n2715(s, out),
		['с', s @ ..] => super::t6::n2716(s, out),
		['т', s @ ..] => super::t6::n2717(s, out),
		['у', s @ ..] => super::t6::n2718(s, out),
		['ч', s @ ..] => super::t6::n2719(s, out),
		['ш', s @ ..] => super::t6::n2720(s, out),
		['щ', s @ ..] => super::t6::n2721(s, out),
		['ъ', s @ ..] => super::t6::n2722(s, out),
		['ы', s @ ..] => super::t6::n2723(s, out),
		['ь', s @ ..] => super::t6::n2724(s, out),
		['ю', s @ ..] => super::t6::n2725(s, out),
		['я', s @ ..] => super::t6::n2726(s, out),
		_ => false,
	}
}

/// Words ending -ишься.
pub(crate) fn n532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 529);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2680);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7534);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14203);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20670);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 20671);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 20672);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 20673);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26296);
											s.is_empty()
										}
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
					super::push(out, 7535);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14204);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14205);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 14206);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2681);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7536);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 14208);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 14209);
							match s {
								[] => true,
								['к', 'с', '\u{1}', s @ ..] => {
									super::push(out, 20674);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 20675);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', '\u{1}', s @ ..] => {
									super::push(out, 20676);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 20677);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 26297);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', 'о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 14210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t6::n7881(s, out),
				['л', s @ ..] => super::t6::n7882(s, out),
				['м', s @ ..] => super::t6::n7883(s, out),
				['о', s @ ..] => super::t6::n7884(s, out),
				['р', s @ ..] => super::t6::n7885(s, out),
				['у', s @ ..] => super::t6::n7886(s, out),
				['ы', s @ ..] => super::t6::n7887(s, out),
				['ю', s @ ..] => super::t6::n7888(s, out),
				['я', s @ ..] => super::t6::n7889(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t6::n2729(s, out),
		['г', s @ ..] => super::t6::n2730(s, out),
		['д', s @ ..] => super::t6::n2731(s, out),
		['е', s @ ..] => super::t6::n2732(s, out),
		['ж', s @ ..] => super::t6::n2733(s, out),
		['з', s @ ..] => super::t6::n2734(s, out),
		['л', s @ ..] => super::t6::n2735(s, out),
		['м', s @ ..] => super::t7::n2736(s, out),
		['н', s @ ..] => super::t7::n2737(s, out),
		['о', s @ ..] => super::t7::n2738(s, out),
		['п', s @ ..] => super::t7::n2739(s, out),
		['р', s @ ..] => super::t7::n2740(s, out),
		['с', s @ ..] => super::t7::n2741(s, out),
		['т', s @ ..] => super::t7::n2742(s, out),
		['у', s @ ..] => super::t7::n2743(s, out),
		['ф', s @ ..] => super::t7::n2744(s, out),
		['ч', s @ ..] => super::t7::n2745(s, out),
		['ш', s @ ..] => super::t7::n2746(s, out),
		['щ', s @ ..] => super::t7::n2747(s, out),
		['ь', s @ ..] => super::t7::n2748(s, out),
		_ => false,
	}
}

/// Words ending -еваешь.
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 545);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2729);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2730);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15072);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 15073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7797);
					s.is_empty()
				}
				['о', 'п', s @ ..] => {
					super::push(out, 7798);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7799);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15074);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15075);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15076);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2731);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2732);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7800);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15077);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 15078);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22353);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 15079);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22354);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 27768);
											s.is_empty()
										}
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
					super::push(out, 7801);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 7802);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7803);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7804);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7805);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7806);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7807);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7808);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7809);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7810);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7811);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2733);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7812);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7813);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7814);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15086);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22356);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2735);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7816);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7817);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7818);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15088);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22358);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 27769);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 15089);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 15090);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 22359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 15091);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 22360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 15092);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 15093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7819);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15094);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 22361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15095);
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 15096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7820);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15097);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22362);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 22363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 15098);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 15099);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7821);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15100);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22366);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 22367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15101);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15102);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22368);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27770);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 22369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7822);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 15103);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 22370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t7::n2784(s, out),
		['н', s @ ..] => super::t7::n2785(s, out),
		['о', s @ ..] => super::t7::n2786(s, out),
		['п', s @ ..] => super::t7::n2787(s, out),
		['р', s @ ..] => super::t7::n2788(s, out),
		['с', s @ ..] => {
			super::push(out, 2740);
			s.is_empty()
		}
		['т', s @ ..] => super::t7::n2790(s, out),
		['у', s @ ..] => super::t7::n2791(s, out),
		['ч', s @ ..] => super::t7::n2792(s, out),
		['ш', s @ ..] => super::t7::n2793(s, out),
		['щ', s @ ..] => super::t7::n2794(s, out),
		_ => false,
	}
}

/// Words ending -иваешь.
pub(crate) fn n549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 546);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2746);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7855);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7856);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7857);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15139);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 15140);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15141);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'у', s @ ..] => {
					super::push(out, 7858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7859);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', s @ ..] => {
									super::push(out, 22400);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 22401);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27777);
											s.is_empty()
										}
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
					super::push(out, 7860);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 7861);
					match s {
						[] => true,
						['т', 'с', '\u{1}', s @ ..] => {
							super::push(out, 15144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7862);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15145);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22402);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 22403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 15146);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 15147);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15148);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 27778);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 27779);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 22406);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27780);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31154);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 31155);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 27781);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 27782);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 22407);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 27783);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22408);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 27784);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 22409);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 22410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15149);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 15150);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15151);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 7863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2747);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
							super::push(out, 15152);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22412);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'о', s @ ..] => {
							super::push(out, 15153);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', 'о', 'о', s @ ..] => {
					super::push(out, 7864);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 7865);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7866);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7867);
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', 'о', s @ ..] => {
							super::push(out, 15155);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 15156);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', 'о', s @ ..] => {
					super::push(out, 7868);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7869);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7870);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15159);
							s.is_empty()
						}
						['о', s @ ..] => super::t7::n15987(s, out),
						_ => false,
					}
				}
				['т', 'о', 'о', s @ ..] => super::t7::n8228(s, out),
				['у', 'о', s @ ..] => super::t7::n8229(s, out),
				['ы', 'в', 'о', s @ ..] => super::t7::n8230(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t7::n2797(s, out),
		['г', s @ ..] => super::t7::n2798(s, out),
		['е', s @ ..] => super::t7::n2799(s, out),
		['ж', s @ ..] => super::t7::n2800(s, out),
		['з', s @ ..] => super::t7::n2801(s, out),
		['к', s @ ..] => super::t7::n2802(s, out),
		['л', s @ ..] => super::t7::n2803(s, out),
		['н', s @ ..] => super::t7::n2804(s, out),
		['о', s @ ..] => super::t7::n2805(s, out),
		['п', s @ ..] => super::t7::n2806(s, out),
		['р', s @ ..] => super::t7::n2807(s, out),
		['х', s @ ..] => super::t7::n2808(s, out),
		['ч', s @ ..] => super::t7::n2809(s, out),
		['ш', s @ ..] => super::t7::n2810(s, out),
		['щ', s @ ..] => super::t7::n2811(s, out),
		_ => false,
	}
}

/// Words ending -ываешь.
pub(crate) fn n552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 549);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2770);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8010);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15427);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22686);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 8011);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8012);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15429);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 15430);
							match s {
								[] => true,
								['х', 'и', s @ ..] => {
									super::push(out, 22687);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15431);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15432);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22688);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28012);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 22689);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 28013);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 22690);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 28014);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 31256);
													s.is_empty()
												}
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
							super::push(out, 15433);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22691);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 15434);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22692);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 28015);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 31257);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 22693);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 22694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8013);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8014);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8015);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8016);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15438);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 15439);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15440);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8017);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8018);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8019);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15442);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8020);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 8021);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t7::n2820(s, out),
		['д', s @ ..] => {
			super::push(out, 2772);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8029);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22722);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28034);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31281);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22723);
									match s {
										[] => true,
										['о', 'о', s @ ..] => {
											super::push(out, 28035);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 31282);
													s.is_empty()
												}
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
							super::push(out, 15470);
							match s {
								[] => true,
								['к', 'с', s @ ..] => super::t7::n24044(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => super::t7::n16329(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t7::n8390(s, out),
				['з', s @ ..] => super::t7::n8391(s, out),
				['и', s @ ..] => super::t7::n8392(s, out),
				['р', s @ ..] => super::t7::n8393(s, out),
				['ы', s @ ..] => super::t7::n8394(s, out),
				['я', s @ ..] => super::t7::n8395(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t7::n2822(s, out),
		['л', s @ ..] => super::t7::n2823(s, out),
		['м', s @ ..] => super::t7::n2824(s, out),
		['н', s @ ..] => super::t7::n2825(s, out),
		['п', s @ ..] => super::t7::n2826(s, out),
		['р', s @ ..] => super::t7::n2827(s, out),
		['с', s @ ..] => super::t7::n2828(s, out),
		['т', s @ ..] => super::t7::n2829(s, out),
		['ц', s @ ..] => super::t7::n2830(s, out),
		_ => false,
	}
}

/// Words ending -стегаешь.
pub(crate) fn n8507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8145);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15618);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15619);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15620);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15621);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 15622);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 15623);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22856);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -згаешь.
pub(crate) fn n556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 553);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2789);
			match s {
				[] => true,
				['р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15624);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2790);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 8146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2791);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15627);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22857);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22858);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 22859);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22860);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 22861);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28139);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 15628);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22862);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22863);
									s.is_empty()
								}
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
			super::push(out, 2792);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15629);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -игаешь.
pub(crate) fn n557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 554);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2793);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15631);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15632);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 22864);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 15633);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15634);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15635);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 22865);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 22866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15636);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2794);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2795);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8148);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8149);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2796);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мгаешь.
pub(crate) fn n558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 555);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 2799);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8152);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нгаешь.
pub(crate) fn n559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 556);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2800);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8153);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -огаешь.
pub(crate) fn n560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 557);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 2801);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8154);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15639);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15640);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8155);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8156);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8157);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8158);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2802);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8159);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15644);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15645);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15646);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15647);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22870);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22871);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 28140);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 28141);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 28142);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 22872);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22873);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 22874);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28143);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргаешь.
pub(crate) fn n561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 558);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2803);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8161);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8162);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15648);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15649);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 15650);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15651);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15652);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22875);
									s.is_empty()
								}
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
			super::push(out, 2804);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15653);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15654);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15655);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 15656);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8164);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -угаешь.
pub(crate) fn n562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 559);
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8165);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8166);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8167);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8168);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2805);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8169);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8170);
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15659);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15660);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15661);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8171);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2806);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8172);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгаешь.
pub(crate) fn n563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2807);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2808);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8173);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2809);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 8175);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15663);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15664);
							s.is_empty()
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
				['\u{1}', s @ ..] => {
					super::push(out, 8176);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8177);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15665);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15666);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8178);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8179);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15670);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15671);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8181);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8182);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 15674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8183);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьгаешь.
pub(crate) fn n564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 2810);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -югаешь.
pub(crate) fn n565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 562);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 2811);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15676);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 15677);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 15678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ягаешь.
pub(crate) fn n566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 563);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2812);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8184);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8185);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2813);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8186);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8187);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15682);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15683);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22879);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22880);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 28144);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2814);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8188);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8189);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							super::push(out, 15685);
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							super::push(out, 15686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2815);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8190);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8191);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8192);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8193);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8194);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8195);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8196);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8197);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8198);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8199);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едаешь.
pub(crate) fn n571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 568);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2827);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2828);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15704);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2829);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8224);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8225);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8226);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15706);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15707);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 22892);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28147);
											s.is_empty()
										}
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
					super::push(out, 8227);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2830);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2831);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2832);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2833);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8231);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							super::push(out, 15709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2834);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8233);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15710);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 15711);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22893);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15712);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 15713);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15714);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 15715);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 15716);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8234);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2835);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8236);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22897);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22898);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 28148);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22899);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 28149);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 15719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8237);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 8238);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8239);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2836);
			s.is_empty()
		}
		['ъ', s @ ..] => {
			super::push(out, 2837);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8241);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 15721);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2838);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ждаешь.
pub(crate) fn n572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 569);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2839);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8244);
					s.is_empty()
				}
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15722);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15723);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28150);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 28151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'о', s @ ..] => {
					super::push(out, 8245);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15724);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22901);
									s.is_empty()
								}
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
			super::push(out, 2840);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2841);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8246);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2842);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2843);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8248);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 8249);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15726);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22902);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22903);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8250);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8251);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8252);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22905);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 28152);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22906);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28153);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 15727);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15728);
							s.is_empty()
						}
						['с', 'а', 'р', 'о', s @ ..] => {
							super::push(out, 15729);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22907);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аздаешь.
pub(crate) fn n2895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2844);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 8253);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8254);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15731);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15732);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15733);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -создаешь.
pub(crate) fn n8629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8257);
	s.is_empty()
}

/// Words ending -уздаешь.
pub(crate) fn n2898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2847);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8258);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15734);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 15735);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -идаешь.
pub(crate) fn n574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 571);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2848);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8260);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8261);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8262);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2849);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 2850);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2851);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8265);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8266);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8267);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15737);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8269);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2852);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндаешь.
pub(crate) fn n575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 572);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2853);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8271);
					match s {
						[] => true,
						['а', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22913);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22914);
									s.is_empty()
								}
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
			super::push(out, 2854);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2855);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8273);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 15740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8274);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 2856);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8275);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одаешь.
pub(crate) fn n576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 573);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2857);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8276);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2858);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2859);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15742);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15743);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15744);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15745);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8279);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22916);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22917);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22918);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22919);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22920);
									s.is_empty()
								}
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
			super::push(out, 2860);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2861);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8281);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							super::push(out, 15746);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22921);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', 'р', 'о', s @ ..] => {
							super::push(out, 15747);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сдаешь.
pub(crate) fn n577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 574);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2862);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2863);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тдаешь.
pub(crate) fn n578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 575);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2865);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удаешь.
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 576);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 2866);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2867);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8284);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8285);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдаешь.
pub(crate) fn n580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 577);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2868);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2869);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8286);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8287);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8288);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8289);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдаешь.
pub(crate) fn n581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 578);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2870);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', 'о', s @ ..] => {
							super::push(out, 15748);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядаешь.
pub(crate) fn n582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 579);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2871);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8290);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8291);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8292);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8293);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8294);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'г', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15750);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2872);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15752);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15753);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нижаешь.
pub(crate) fn n2936(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8311);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8312);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15776);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртижаешь.
pub(crate) fn n8692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8313);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15777);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лжаешь.
pub(crate) fn n589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 586);
	match s {
		[] => true,
		['о', 'д', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8314);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8315);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8316);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нжаешь.
pub(crate) fn n590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 587);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ожаешь.
pub(crate) fn n591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 588);
	match s {
		[] => true,
		['б', 'о', 'а', s @ ..] => {
			super::push(out, 2884);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2885);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2886);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2887);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2888);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8319);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8320);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 8321);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8322);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15782);
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22935);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28158);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22936);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28159);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 22937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8323);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15783);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22938);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 22939);
									s.is_empty()
								}
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
			super::push(out, 2889);
			s.is_empty()
		}
		_ => false,
	}
}

