use flexion::Path;

/// Words ending -reo.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 457);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1429);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1430);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2950);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2951);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2952);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2953);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2954);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2955);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2956);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4313);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4314);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5101);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 458);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2957);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4315);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4316);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5102);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4317);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4318);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2958);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1435);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2959);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2960);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2962);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2963);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4321);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1437);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2964);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4322);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1438);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2965);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2966);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4323);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2967);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1439);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2969);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2970);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 460);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 1443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['f', 'e', 'd', s @ ..] => {
							super::push(out, 2973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4327);
									s.is_empty()
								}
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
			super::push(out, 463);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2974);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4328);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1446);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t2::n1617(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1448);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -seo.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2978);
							match s {
								[] => true,
								['t', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2979);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 4338);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5106);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 5107);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 5404);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2980);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -teo.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 467);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2982);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4340);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4341);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 5108);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5109);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4342);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1454);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 474);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1457);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1458);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ueo.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 476);
			match s {
				[] => true,
				['r', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1460);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1461);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 480);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1464);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1465);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2988);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4344);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5110);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1466);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2989);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1467);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1468);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2991);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5111);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 5112);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5405);
													match s {
														[] => true,
														['r', s @ ..] => super::t2::n8197(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 5113);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 5406);
													match s {
														[] => true,
														['c', 'r', 'i', 'c', '\u{1}', s @ ..] => super::t2::n8198(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 5114);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5115);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 5407);
													match s {
														[] => true,
														['p', '\u{1}', s @ ..] => super::t2::n8199(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 5116);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5408);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8200(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -veo.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1469);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1470);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1472);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1473);
					match s {
						[] => true,
						['x', s @ ..] => {
							super::push(out, 2994);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4345);
									match s {
										[] => true,
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
		['e', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1475);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1476);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2995);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 486);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1478);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1479);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2997);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2998);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4346);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5117);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1480);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4347);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4348);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5118);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5119);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 4349);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 5120);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4350);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5121);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 5122);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 5409);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8201(s, out),
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
									super::push(out, 4351);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5123);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xeo.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 488);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word intellego.
pub(crate) fn n7947(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word subintellego.
pub(crate) fn n7948(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -bnego.
pub(crate) fn n1677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1504);
	match s {
		[] => true,
		['u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -enego.
pub(crate) fn n1678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1505);
	s.is_empty()
}

/// Words ending -rego.
pub(crate) fn n517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 504);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1506);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1507);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1508);
			match s {
				[] => true,
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 3013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tego.
pub(crate) fn n518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 505);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -igo.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1509);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1510);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3014);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1511);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', 'u', 'r', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1513);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1514);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1515);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1516);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3016);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3017);
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4354);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4355);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3018);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3019);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3020);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3021);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3022);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1519);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 3023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1521);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3025);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3026);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4357);
									match s {
										[] => true,
										['n', 'i', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3027);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3028);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5124);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 5125);
											match s {
												[] => true,
												['r', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 3029);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['r', 'd', s @ ..] => {
							super::push(out, 3031);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', 'l', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['i', s @ ..] => super::t2::n1716(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n525(s, out),
		['s', s @ ..] => super::t2::n526(s, out),
		['t', s @ ..] => super::t2::n527(s, out),
		['v', s @ ..] => super::t2::n528(s, out),
		['x', s @ ..] => super::t2::n529(s, out),
		_ => false,
	}
}

/// Words ending -lgo.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1540);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1541);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1542);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ngo.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1543);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1544);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3044);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4367);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 5128);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3046);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3047);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4368);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3048);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1546);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3050);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4369);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4370);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 5129);
											match s {
												[] => true,
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
						['e', s @ ..] => {
							super::push(out, 3051);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3052);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4371);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1549);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4372);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5130);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3053);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4373);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 5131);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => super::t2::n3700(s, out),
						['r', 'e', 'p', '\u{1}', s @ ..] => super::t2::n3701(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n1743(s, out),
				['m', s @ ..] => super::t2::n1744(s, out),
				['n', s @ ..] => super::t2::n1745(s, out),
				['p', s @ ..] => super::t2::n1746(s, out),
				['r', s @ ..] => super::t2::n1747(s, out),
				['t', s @ ..] => super::t2::n1748(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n533(s, out),
		['u', s @ ..] => super::t2::n534(s, out),
		_ => false,
	}
}

/// Words ending -ogo.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1561);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3074);
							match s {
								[] => true,
								['e', 'a', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'o', 'm', 'o', 'h', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					super::push(out, 1562);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rgo.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1563);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1564);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3075);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3076);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 3077);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4388);
									match s {
										[] => true,
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
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1567);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3078);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 4389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3079);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4390);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4391);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 5134);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1568);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1569);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3080);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3081);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3082);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4392);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4393);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5135);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5136);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3083);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4394);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4395);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5137);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3084);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4396);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 3085);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4397);
									match s {
										[] => true,
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
				['x', s @ ..] => {
					super::push(out, 1571);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 3086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ugo.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 528);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 529);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1572);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 3087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1573);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 3088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1574);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 1575);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word pertraho.
pub(crate) fn n7961(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -upertraho.
pub(crate) fn n7962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5412);
	match s {
		[] => true,
		['s', s @ ..] => {
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

/// Words ending -cio.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 544);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3093);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3094);
							match s {
								[] => true,
								['b', s @ ..] => super::t2::n5839(s, out),
								['c', s @ ..] => super::t2::n5840(s, out),
								['d', s @ ..] => super::t2::n5841(s, out),
								['g', s @ ..] => super::t2::n5842(s, out),
								['l', 'a', s @ ..] => super::t2::n5843(s, out),
								['m', s @ ..] => super::t2::n5844(s, out),
								['n', s @ ..] => super::t2::n5845(s, out),
								['p', s @ ..] => super::t2::n5846(s, out),
								['r', s @ ..] => super::t2::n5847(s, out),
								['u', s @ ..] => super::t2::n5848(s, out),
								['v', s @ ..] => super::t2::n5849(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t2::n3785(s, out),
						['l', s @ ..] => super::t2::n3786(s, out),
						['r', 'a', '\u{1}', s @ ..] => super::t2::n3787(s, out),
						['s', 'i', s @ ..] => super::t2::n3788(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t2::n1794(s, out),
				['l', s @ ..] => super::t2::n1795(s, out),
				['m', s @ ..] => super::t2::n1796(s, out),
				['r', s @ ..] => super::t2::n1797(s, out),
				['s', s @ ..] => super::t2::n1798(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t2::n560(s, out),
		['e', s @ ..] => super::t2::n561(s, out),
		['i', s @ ..] => super::t2::n562(s, out),
		['l', s @ ..] => super::t2::n563(s, out),
		['n', s @ ..] => super::t2::n564(s, out),
		['o', s @ ..] => super::t2::n565(s, out),
		['r', s @ ..] => super::t2::n566(s, out),
		['s', s @ ..] => super::t2::n567(s, out),
		['u', s @ ..] => super::t2::n568(s, out),
		['x', s @ ..] => super::t2::n569(s, out),
		_ => false,
	}
}

/// The word inoboedio.
pub(crate) fn n7985(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -midio.
pub(crate) fn n1842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1628);
	s.is_empty()
}

/// Words ending -ridio.
pub(crate) fn n1843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1629);
	s.is_empty()
}

/// Words ending -tidio.
pub(crate) fn n1844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1630);
	match s {
		[] => true,
		['p', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3140);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3141);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ndio.
pub(crate) fn n573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 559);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1631);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3142);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3143);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4429);
							match s {
								[] => true,
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
		['e', s @ ..] => {
			super::push(out, 1632);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3144);
					match s {
						[] => true,
						['i', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1633);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3145);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1634);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1635);
			match s {
				[] => true,
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -odio.
pub(crate) fn n574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1636);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1637);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1638);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3149);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4430);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ordio.
pub(crate) fn n575(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -udio.
pub(crate) fn n576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3150);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4431);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4432);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5143);
									match s {
										[] => true,
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
				['d', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3151);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1641);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3152);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3153);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4434);
							match s {
								[] => true,
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
		['p', s @ ..] => {
			super::push(out, 1642);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1643);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 's', '\u{1}', s @ ..] => {
			super::push(out, 1644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eio.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 562);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fio.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 563);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1645);
					match s {
						[] => true,
						['a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1646);
					match s {
						[] => true,
						['o', 's', 'b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1647);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gio.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1648);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1649);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3158);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3159);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['f', 'f', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1652);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3160);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3161);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4436);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5144);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4437);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4438);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3162);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4439);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4440);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4441);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5145);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4442);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 5146);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3163);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4443);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5147);
											match s {
												[] => true,
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
						['o', s @ ..] => {
							super::push(out, 3164);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4444);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3165);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4445);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 5148);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5417);
													s.is_empty()
												}
												['u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 5149);
											match s {
												[] => true,
												['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3166);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4446);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5150);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3167);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4447);
									match s {
										[] => true,
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
				['m', s @ ..] => {
					super::push(out, 1654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3168);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3169);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4448);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3170);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3171);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4450);
									match s {
										[] => true,
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
				['r', s @ ..] => {
					super::push(out, 1655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -hio.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 569);
			s.is_empty()
		}
		['c', 'n', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 571);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1657);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iio.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 572);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1658);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lio.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 573);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 574);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3173);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3174);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 575);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1661);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1662);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1663);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3176);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 576);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1664);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3177);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5418);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8209(s, out),
														_ => false,
													}
												}
												['o', s @ ..] => {
													super::push(out, 5419);
													match s {
														[] => true,
														['c', '\u{1}', s @ ..] => super::t2::n8210(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1665);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3178);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4452);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3179);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4453);
									match s {
										[] => true,
										['c', 'c', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1666);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1667);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3181);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4454);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5151);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3182);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4456);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4457);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3183);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3184);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4459);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3185);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4461);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 5152);
											match s {
												[] => true,
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
						['r', s @ ..] => {
							super::push(out, 3186);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3187);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4463);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4464);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5153);
											match s {
												[] => true,
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
						['x', s @ ..] => {
							super::push(out, 3188);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 1668);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 1669);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 3189);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3190);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n592(s, out),
		['o', s @ ..] => super::t2::n593(s, out),
		['p', s @ ..] => super::t2::n594(s, out),
		['u', s @ ..] => super::t2::n595(s, out),
		_ => false,
	}
}

/// Words ending -mio.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 581);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1678);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3201);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1679);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3202);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 583);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1680);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4469);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4470);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5156);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4471);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4472);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5157);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4473);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5158);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 5159);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nio.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1681);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1682);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3203);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1683);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3204);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 4474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3205);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4475);
									match s {
										[] => true,
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
		['e', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1684);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3206);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['m', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3207);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 3208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3209);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3210);
							match s {
								[] => true,
								['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 3211);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4476);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4477);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 3212);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4478);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3213);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4479);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4480);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4481);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 4482);
									match s {
										[] => true,
										['n', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3214);
							match s {
								[] => true,
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3215);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4483);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4484);
									match s {
										[] => true,
										['c', 's', s @ ..] => {
											super::push(out, 5160);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5420);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3216);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3217);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 5421);
													match s {
														[] => true,
														['s', s @ ..] => super::t2::n8211(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 5161);
											match s {
												[] => true,
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3218);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4485);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5162);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['i', s @ ..] => super::t2::n1903(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n602(s, out),
		['m', s @ ..] => super::t2::n603(s, out),
		['n', s @ ..] => super::t2::n604(s, out),
		['o', s @ ..] => super::t2::n605(s, out),
		['r', s @ ..] => super::t2::n606(s, out),
		['u', s @ ..] => super::t2::n607(s, out),
		_ => false,
	}
}

/// Words ending -pio.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 593);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 594);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1702);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3232);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4495);
									match s {
										[] => true,
										['n', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3233);
							match s {
								[] => true,
								['s', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1704);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1705);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1706);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1707);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3235);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3236);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3237);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1710);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1711);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1712);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3239);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4497);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5165);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3240);
							match s {
								[] => true,
								['o', s @ ..] => super::t2::n6069(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t2::n612(s, out),
		['o', s @ ..] => super::t2::n613(s, out),
		['p', s @ ..] => super::t2::n614(s, out),
		['r', s @ ..] => super::t2::n615(s, out),
		['s', s @ ..] => super::t2::n616(s, out),
		['u', s @ ..] => super::t2::n617(s, out),
		['x', s @ ..] => super::t2::n618(s, out),
		_ => false,
	}
}

/// Words ending -rio.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 604);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 605);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1726);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3251);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 606);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1728);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1729);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3254);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4501);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 5166);
											match s {
												[] => true,
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4502);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 4503);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5167);
											match s {
												[] => true,
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
						['e', s @ ..] => {
							super::push(out, 3255);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4504);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3256);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4505);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3257);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3258);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 's', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3259);
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 4507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1730);
					match s {
						[] => true,
						['u', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 607);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1731);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1732);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3262);
							match s {
								[] => true,
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
		['g', s @ ..] => {
			super::push(out, 608);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1733);
					match s {
						[] => true,
						['i', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1734);
					match s {
						[] => true,
						['x', s @ ..] => {
							super::push(out, 3263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1737);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 3264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1739);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3266);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 610);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1740);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4508);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4509);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t2::n626(s, out),
		['t', s @ ..] => {
			super::push(out, 612);
			match s {
				[] => true,
				['e', s @ ..] => super::t2::n1963(s, out),
				['n', s @ ..] => super::t2::n1964(s, out),
				['u', s @ ..] => super::t2::n1965(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t2::n628(s, out),
		_ => false,
	}
}

/// Words ending -sio.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 614);
			match s {
				[] => true,
				['r', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 615);
			match s {
				[] => true,
				['e', 'c', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['a', 'b', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1757);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1758);
					match s {
						[] => true,
						['c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1759);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 3296);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1760);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 617);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1761);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tio.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 618);
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1762);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1763);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3298);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 3299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1764);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3300);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1765);
					match s {
						[] => true,
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1766);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 3301);
							match s {
								[] => true,
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
		['e', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1767);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3302);
							match s {
								[] => true,
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3303);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3304);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1769);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 621);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1770);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1771);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3306);
							match s {
								[] => true,
								['n', 'i', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1772);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3307);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3308);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1773);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3309);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5177);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5178);
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 5422);
													match s {
														[] => true,
														['a', s @ ..] => super::t2::n8212(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4531);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 623);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1774);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 624);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1775);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4532);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 5423);
													match s {
														[] => true,
														['a', s @ ..] => super::t2::n8213(s, out),
														_ => false,
													}
												}
												['t', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1776);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3311);
							match s {
								[] => true,
								['m', 'i', '\u{1}', s @ ..] => super::t2::n6149(s, out),
								['n', 'i', '\u{1}', s @ ..] => super::t2::n6150(s, out),
								['s', 'i', 'd', '\u{1}', s @ ..] => super::t2::n6151(s, out),
								_ => false,
							}
						}
						['t', s @ ..] => super::t2::n4102(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t2::n640(s, out),
		['t', s @ ..] => super::t2::n641(s, out),
		['u', s @ ..] => super::t2::n642(s, out),
		_ => false,
	}
}

/// Words ending -uio.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 628);
			s.is_empty()
		}
		['l', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
			super::push(out, 629);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['e', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -vio.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 631);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 632);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 633);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 634);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1791);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3325);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4544);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4545);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5185);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1792);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1796);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3328);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4546);
									match s {
										[] => true,
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
		['i', s @ ..] => {
			super::push(out, 635);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 636);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1798);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4547);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4548);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 5186);
											match s {
												[] => true,
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4549);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4550);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5187);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 5424);
													match s {
														[] => true,
														['p', '\u{1}', s @ ..] => super::t2::n8214(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 5188);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4551);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5189);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 4552);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5190);
											match s {
												[] => true,
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
						['t', s @ ..] => {
							super::push(out, 3331);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4553);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xio.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	match s {
		[] => true,
		['n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1800);
					match s {
						[] => true,
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word insibilo.
pub(crate) fn n8029(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word obnubilo.
pub(crate) fn n8030(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ipilo.
pub(crate) fn n2037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1814);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3338);
			match s {
				[] => true,
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3339);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'r', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 3340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tilo.
pub(crate) fn n678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1816);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3341);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1817);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3342);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1818);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -llo.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 657);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1820);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1821);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3344);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3345);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 658);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1824);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3346);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3347);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 4561);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4562);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 3348);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3349);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4564);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1825);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3350);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4566);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5194);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 4567);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5195);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3351);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3352);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3353);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4569);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 3354);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4570);
									match s {
										[] => true,
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
				['f', s @ ..] => {
					super::push(out, 1826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3355);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3356);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4571);
									match s {
										[] => true,
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
				['g', s @ ..] => {
					super::push(out, 1827);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3358);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3359);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4572);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3360);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => super::t2::n4170(s, out),
						['m', s @ ..] => super::t2::n4171(s, out),
						['n', s @ ..] => super::t2::n4172(s, out),
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => super::t2::n4174(s, out),
						['r', s @ ..] => super::t2::n4175(s, out),
						['s', s @ ..] => super::t2::n4176(s, out),
						['x', 'e', '\u{1}', s @ ..] => super::t2::n4177(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t2::n2054(s, out),
				['t', s @ ..] => super::t2::n2055(s, out),
				['u', 'd', '\u{1}', s @ ..] => super::t2::n2056(s, out),
				['v', s @ ..] => super::t2::n2057(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n681(s, out),
		['o', s @ ..] => super::t2::n682(s, out),
		['u', s @ ..] => super::t2::n683(s, out),
		_ => false,
	}
}

/// Words ending -olo.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 662);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1838);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1839);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1840);
					match s {
						[] => true,
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1841);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3379);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1842);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 1843);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 1844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1845);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1846);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1847);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3382);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3383);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 668);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1849);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3384);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1850);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1851);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3385);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1852);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1853);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3386);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3387);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4582);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4583);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 5199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1854);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1855);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4584);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 4585);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5200);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 5201);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5426);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8215(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plo.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['a', 'x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1857);
					match s {
						[] => true,
						['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ulo.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 1858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1859);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							super::push(out, 3389);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1860);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1861);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4586);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5202);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 5203);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4587);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5204);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5205);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 5427);
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 5206);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 4588);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4589);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5207);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4590);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1862);
					match s {
						[] => true,
						['m', 'm', s @ ..] => {
							super::push(out, 3390);
							s.is_empty()
						}
						['t', 'a', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1863);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3391);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4592);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'p', 's', 'e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1864);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 't', 'a', 'm', 'm', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1865);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3392);
							match s {
								[] => true,
								['t', 'a', 'r', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3393);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 4593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['e', 'd', s @ ..] => {
					super::push(out, 1866);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 678);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 679);
			match s {
				[] => true,
				['l', 'u', 'p', 'e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['e', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1867);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3395);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3396);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5208);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5428);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8216(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1868);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3397);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3398);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4594);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5209);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n2110(s, out),
				['o', s @ ..] => super::t2::n2111(s, out),
				['u', s @ ..] => super::t2::n2112(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t2::n704(s, out),
		['t', s @ ..] => super::t2::n705(s, out),
		_ => false,
	}
}

/// Words ending -ylo.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1876);
					match s {
						[] => true,
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

/// The word confremo.
pub(crate) fn n8041(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rimo.
pub(crate) fn n723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 699);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1891);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1892);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3418);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1893);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3420);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1894);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3421);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1895);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mmo.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'f', 'e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -omo.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 704);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1900);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['i', 'p', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1903);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3424);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rmo.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	match s {
		[] => true,
		['a', 'x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', 'f', s @ ..] => {
			match s {
				[] => true,
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 1905);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3425);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4603);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -smo.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	s.is_empty()
}

/// Words ending -umo.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 710);
			match s {
				[] => true,
				['f', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 712);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 713);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 714);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 716);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['æ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ymo.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 717);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1909);
					match s {
						[] => true,
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ino.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 735);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1920);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1921);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3436);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3437);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3438);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4606);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5214);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 4607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1922);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3439);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4608);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3440);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1923);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3441);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4609);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 5215);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4610);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'i', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1924);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3442);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3443);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4611);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3444);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4612);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3445);
							match s {
								[] => true,
								['r', 'e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1925);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3446);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3447);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4613);
									match s {
										[] => true,
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1926);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1927);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3449);
							match s {
								[] => true,
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
		['d', s @ ..] => {
			super::push(out, 736);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 737);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1928);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 3450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1929);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1930);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3451);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4614);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4615);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5216);
											match s {
												[] => true,
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
		['l', s @ ..] => {
			super::push(out, 738);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1931);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1932);
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1933);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3452);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3453);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1934);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3454);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 4616);
									match s {
										[] => true,
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1935);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t2::n2210(s, out),
				['n', s @ ..] => super::t2::n2211(s, out),
				['r', s @ ..] => super::t2::n2212(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t2::n769(s, out),
		['p', s @ ..] => super::t2::n770(s, out),
		['r', s @ ..] => {
			super::push(out, 741);
			s.is_empty()
		}
		['s', s @ ..] => super::t2::n772(s, out),
		['t', s @ ..] => super::t2::n773(s, out),
		['u', s @ ..] => super::t2::n774(s, out),
		['v', s @ ..] => super::t2::n775(s, out),
		_ => false,
	}
}

/// Words ending -ono.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1954);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1956);
					match s {
						[] => true,
						['i', 'd', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1957);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3464);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4623);
									match s {
										[] => true,
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1958);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1959);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3465);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 5217);
											match s {
												[] => true,
												['a', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3466);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 5218);
											match s {
												[] => true,
												['a', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1960);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1962);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1963);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3467);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3468);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3469);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4626);
									match s {
										[] => true,
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
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1964);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1965);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3470);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1966);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1968);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3473);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4627);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1969);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3474);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t2::n787(s, out),
		_ => false,
	}
}

/// Words ending -rno.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1979);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 758);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1980);
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1981);
					match s {
						[] => true,
						['c', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1982);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3486);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1983);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3487);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3488);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3489);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 5219);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 4632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1984);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3490);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['x', s @ ..] => {
					super::push(out, 1985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lpo.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1998);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1999);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3507);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4642);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 3508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2000);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3509);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3510);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4643);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4644);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4645);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 4646);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2001);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -mpo.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2002);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 771);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rpo.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 776);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2006);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3515);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4647);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3516);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3517);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3518);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4648);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3519);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4649);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 777);
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2008);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2009);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -upo.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3520);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3521);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4650);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2011);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ero.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2020);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2021);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 3525);
							match s {
								[] => true,
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
		['b', s @ ..] => {
			super::push(out, 789);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2022);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2023);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4653);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5222);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 790);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2025);
					match s {
						[] => true,
						['m', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2026);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4654);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', 'c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2027);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3526);
							match s {
								[] => true,
								['v', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 791);
			match s {
				[] => true,
				['e', 'o', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2028);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3527);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 792);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2029);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3528);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2030);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3529);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2031);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3530);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3531);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3532);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4656);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3533);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3534);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4657);
									match s {
										[] => true,
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
				['m', s @ ..] => {
					super::push(out, 2032);
					match s {
						[] => true,
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2033);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3535);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3536);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4658);
									match s {
										[] => true,
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
				['o', s @ ..] => {
					super::push(out, 2034);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3537);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2035);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2036);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2037);
					match s {
						[] => true,
						['s', 'o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2038);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3538);
							match s {
								[] => true,
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
		['g', s @ ..] => super::t2::n831(s, out),
		['i', s @ ..] => {
			super::push(out, 794);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2044);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2045);
					match s {
						[] => true,
						['t', s @ ..] => super::t2::n4503(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t2::n834(s, out),
		['n', s @ ..] => super::t2::n835(s, out),
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', s @ ..] => super::t2::n837(s, out),
		['s', s @ ..] => super::t2::n838(s, out),
		['t', s @ ..] => super::t2::n839(s, out),
		['v', s @ ..] => super::t2::n840(s, out),
		['x', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => super::t2::n2395(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uiro.
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 808);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 2079);
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3573);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3574);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3575);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4679);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oro.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2080);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 3576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2081);
					match s {
						[] => true,
						['r', 'r', s @ ..] => {
							super::push(out, 3577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 811);
			match s {
				[] => true,
				['e', 'd', 'n', s @ ..] => {
					super::push(out, 2082);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 812);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'i', 'v', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', 'p', 'm', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 't', 'e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'r', 'e', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 813);
			match s {
				[] => true,
				['f', 'e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3579);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 4680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 3580);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4681);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['u', 'h', 'x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 815);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 816);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'o', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2084);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3581);
							match s {
								[] => true,
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
		['t', s @ ..] => {
			super::push(out, 818);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3582);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['p', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5229);
											s.is_empty()
										}
										['x', s @ ..] => {
											super::push(out, 5230);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 5432);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8219(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3583);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 4682);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 819);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2086);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3585);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 820);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rro.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2088);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							super::push(out, 3587);
							match s {
								[] => true,
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
		['e', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2089);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3588);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2090);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2091);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3591);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2092);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2093);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3593);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2094);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2095);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2096);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3595);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2097);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3596);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3597);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4684);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2098);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2100);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3600);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3601);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4685);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3602);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4686);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4687);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4688);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 4689);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5231);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3603);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4690);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5232);
											match s {
												[] => true,
												['r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3604);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => super::t2::n4583(s, out),
						['r', s @ ..] => super::t2::n4584(s, out),
						['s', s @ ..] => super::t2::n4585(s, out),
						['x', s @ ..] => super::t2::n4586(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t2::n2438(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tro.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 2103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['p', 'm', s @ ..] => {
					super::push(out, 2105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2106);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2107);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 828);
			match s {
				[] => true,
				['e', 'n', 'e', 'f', 'e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', 'a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2108);
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5236);
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 4692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'l', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uro.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 829);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 830);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2109);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 833);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2111);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2112);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2115);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['u', 'a', 'x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 837);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['r', 'u', 'm', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3614);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 3615);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3616);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4694);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4695);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5237);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4696);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2118);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 841);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2119);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2120);
					match s {
						[] => true,
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

/// Words ending -sso.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'e', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 860);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2136);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3628);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 3629);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 4702);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 4703);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3630);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4704);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4705);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3631);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4706);
									match s {
										[] => true,
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
				['p', s @ ..] => {
					super::push(out, 2137);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3632);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2138);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3633);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2139);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3634);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2140);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3635);
							match s {
								[] => true,
								['l', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3636);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2141);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2142);
					match s {
						[] => true,
						['c', 'e', 'o', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2143);
					match s {
						[] => true,
						['u', 'h', 'c', 'a', 'r', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2144);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3638);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 4707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3639);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2145);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3640);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3641);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2146);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3642);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4709);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5238);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 5239);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 5434);
													s.is_empty()
												}
												_ => false,
											}
										}
										['p', s @ ..] => {
											super::push(out, 5240);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['x', 'e', '\u{1}', s @ ..] => {
											super::push(out, 5241);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 862);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 863);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cuso.
pub(crate) fn n913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 865);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2150);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 3644);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4710);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2151);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word calefacto.
pub(crate) fn n8059(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ecto.
pub(crate) fn n921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 870);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 2161);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2162);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2163);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3650);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3651);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3652);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3653);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3655);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 3656);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3657);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3658);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4717);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5245);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 5435);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2164);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2165);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3659);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3660);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3661);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3662);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3663);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3664);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3665);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4718);
							match s {
								[] => true,
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2166);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3666);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3667);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4719);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5246);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3668);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4720);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4721);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5248);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4722);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 4723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2167);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2168);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 2169);
			match s {
				[] => true,
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3670);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4724);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3671);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3672);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4725);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dnicto.
pub(crate) fn n4680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3673);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nnicto.
pub(crate) fn n4681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3674);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 4727);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ricto.
pub(crate) fn n2531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2173);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3675);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ncto.
pub(crate) fn n924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 872);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -octo.
pub(crate) fn n925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 873);
	match s {
		[] => true,
		['n', 'b', s @ ..] => {
			super::push(out, 2174);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ucto.
pub(crate) fn n926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 874);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2175);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2176);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3678);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3679);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3680);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3681);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3682);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eto.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2178);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2180);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 877);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 878);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 879);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 881);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 882);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2183);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3683);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3684);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2184);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3685);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 883);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2185);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2186);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3686);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3687);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2187);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2188);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 884);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 885);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 886);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ito.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 887);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2190);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2191);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2192);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3690);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4729);
									match s {
										[] => true,
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3691);
							match s {
								[] => true,
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2193);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3692);
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4730);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 5249);
											match s {
												[] => true,
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2194);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3693);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2195);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2196);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2197);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3694);
							match s {
								[] => true,
								['b', 'u', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2198);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2199);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3695);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4732);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3696);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4733);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 3697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2200);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3698);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 4734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2201);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3699);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2203);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2204);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 891);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2205);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3702);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3703);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2206);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2207);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3704);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4736);
									match s {
										[] => true,
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
				['o', s @ ..] => {
					super::push(out, 2208);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2209);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['g', 'e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4737);
									match s {
										[] => true,
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
				['u', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2210);
					match s {
						[] => true,
						['c', 'a', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t2::n2577(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t2::n945(s, out),
		['n', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n2582(s, out),
				['g', 'o', 'c', 'c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => super::t2::n2584(s, out),
				['o', 's', '\u{1}', s @ ..] => super::t2::n2585(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t2::n947(s, out),
		['r', s @ ..] => super::t2::n948(s, out),
		['s', s @ ..] => super::t2::n949(s, out),
		['t', s @ ..] => super::t2::n950(s, out),
		['u', s @ ..] => super::t2::n951(s, out),
		['v', s @ ..] => super::t2::n952(s, out),
		_ => false,
	}
}

/// Words ending -lto.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 199);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2232);
					match s {
						[] => true,
						['i', 'f', 'f', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4767);
									match s {
										[] => true,
										['a', 'b', s @ ..] => {
											super::push(out, 5261);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 5439);
													match s {
														[] => true,
														['s', '\u{1}', s @ ..] => super::t2::n8223(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3725);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4768);
									match s {
										[] => true,
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
				['s', s @ ..] => {
					super::push(out, 2234);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3726);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3727);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3728);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3729);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4769);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4770);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5262);
											match s {
												[] => true,
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

/// Words ending -nto.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 903);
			match s {
				[] => true,
				['l', 'p', 'm', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 904);
			match s {
				[] => true,
				['c', 'c', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'e', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2235);
					match s {
						[] => true,
						['i', 'l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2236);
					match s {
						[] => true,
						['s', 'o', s @ ..] => {
							super::push(out, 3730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2237);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3731);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3732);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3733);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'r', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oto.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 905);
			match s {
				[] => true,
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 906);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2238);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 907);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2240);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2241);
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pto.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 909);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 910);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2243);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4772);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 911);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rto.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 913);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2245);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3736);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2246);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2247);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2248);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3738);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4773);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3739);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4774);
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 3740);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4775);
									match s {
										[] => true,
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
				['t', s @ ..] => {
					super::push(out, 2249);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 4776);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5263);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3743);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4777);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5264);
											match s {
												[] => true,
												['r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 914);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2251);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3744);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2252);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3745);
							s.is_empty()
						}
						['p', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3747);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3748);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3749);
							match s {
								[] => true,
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sto.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 916);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 917);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2254);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2255);
					match s {
						[] => true,
						['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2256);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2257);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2258);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2259);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3751);
							match s {
								[] => true,
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
		['d', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2261);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3752);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3753);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2262);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2263);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3754);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4780);
									match s {
										[] => true,
										['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'o', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2264);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2266);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3756);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 921);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3757);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3758);
							match s {
								[] => true,
								['u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2269);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3761);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3762);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4783);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4784);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4785);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 5265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3763);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3764);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4786);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4787);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3765);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4788);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5266);
											match s {
												[] => true,
												['r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3766);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4789);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4790);
									match s {
										[] => true,
										['c', s @ ..] => {
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
						['r', s @ ..] => {
							super::push(out, 3767);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3768);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 3769);
							match s {
								[] => true,
								['e', s @ ..] => super::t2::n6663(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2271);
					match s {
						[] => true,
						['n', s @ ..] => super::t2::n4830(s, out),
						_ => false,
					}
				}
				['x', s @ ..] => super::t2::n2663(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t2::n976(s, out),
		['n', s @ ..] => super::t2::n977(s, out),
		['o', s @ ..] => super::t2::n978(s, out),
		['r', s @ ..] => super::t2::n979(s, out),
		['u', s @ ..] => super::t2::n980(s, out),
		['x', s @ ..] => super::t2::n981(s, out),
		_ => false,
	}
}

